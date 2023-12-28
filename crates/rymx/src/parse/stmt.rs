use super::{common::*, error::ParseError, type_parser};
use crate::{ast::*, tokenize::Token, Span};
use chumsky::{prelude::*, util::MaybeRef};
use std::collections::HashMap;

pub fn stmt_parser(src: &str) -> impl Parser<TokenStream, Stmt, Extra> + Clone {
    recursive(|stmt| {
        let expr = expr_parser(src, stmt.clone());

        // type_def ::=  "type" ident "=" type ";"
        let type_def = just(Token::Type)
            .ignore_then(ident_parser(src))
            .then_ignore(just(Token::Assign))
            .then(type_parser(src))
            .then_ignore(just(Token::Semi))
            .map(|(name, rhs)| Stmt::Type(name.into(), rhs))
            .labelled("type definition")
            .boxed();

        // function_def ::= "fn" ident "(" parameters ")" type? "=>" expr ";"?
        let function_def = just(Token::Fn)
            .ignore_then(ident_parser(src))
            .then(
                parameters_parser(src)
                    .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
            )
            .then(type_parser(src).or_not())
            .then_ignore(just(Token::ThickArrow))
            .then(expr_parser(src, stmt))
            .then(just(Token::Semi).or_not().map(|semi| semi.is_none()))
            .validate(|((rest, body), missing_semi), extra, emitter| {
                // Not emitting "missing semicolon error" for functions
                // that use a block expression as body
                if missing_semi && !matches!(body, Expr::Block(..)) {
                    emitter.emit(ParseError::expected_found(
                        [Some(MaybeRef::Val(Token::Semi))],
                        None,
                        Span::new(extra.span().end, extra.span().end),
                    ))
                }
                (rest, body)
            })
            .map(|(((name, params), maybe_type), body)| {
                Stmt::Variable(
                    VariableKind::Const,
                    name.into(),
                    Type::Unkown, // TODO Use proper function type
                    Expr::Function(Function {
                        params,
                        named_params: HashMap::new(),
                        return_type: maybe_type.unwrap_or(Type::Unit),
                        body: Box::new(body),
                    }),
                )
            });

        // variable ::= ("const" | "let" | "let mut") ident (":" type)? "=" expr ";"
        let variable = choice((
            just(Token::Const).to(VariableKind::Const),
            just(Token::Let)
                .then(just(Token::Mut))
                .to(VariableKind::LetMut),
            just(Token::Let).to(VariableKind::Let),
        ))
        .then(ident_parser(src))
        .then(just(Token::Colon).ignore_then(type_parser(src)).or_not())
        .then_ignore(just(Token::Assign))
        .then(expr.clone())
        .then_ignore(just(Token::Semi))
        .map(|(((kind, name), typ), rhs)| {
            Stmt::Variable(kind, name.into(), typ.unwrap_or(Type::Unkown), rhs)
        })
        .labelled("variable")
        .boxed();

        choice((
            expr.then_ignore(just(Token::Semi)).map(Stmt::Expr),
            type_def,
            function_def,
            variable,
        ))
    })
}

/// Only works when called from stmt_parser!
fn expr_parser<'src>(
    src: &'src str,
    stmt: impl Parser<'src, TokenStream<'src>, Stmt, Extra<'src>> + Clone + 'src,
) -> impl Parser<TokenStream, Expr, Extra> + Clone {
    recursive(|expr| {
        // literal ::= int | float | string
        let literal = literal_parser(src).map(Expr::Literal);

        // function ::= fn "(" parameters ")" type? "=>" expr
        let function = just(Token::Fn)
            .ignore_then(
                parameters_parser(src)
                    .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
            )
            .then(type_parser(src).or_not())
            .then(just(Token::ThickArrow).ignore_then(expr.clone()))
            .map(|((params, return_type), body)| {
                Expr::Function(Function {
                    params,
                    named_params: HashMap::new(),
                    return_type: return_type.unwrap_or(Type::Unkown),
                    body: Box::new(body),
                })
            })
            .labelled("function");

        // array ::= "[" (expr ";" expr | (expr ",")* expr?) "]"
        let array = choice((
            expr.clone()
                .then_ignore(just(Token::Semi))
                .then(expr.clone())
                .map(|(value, length)| Expr::ArrayWithRepeat(value.into(), length.into())),
            expr.clone()
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<Expr>>()
                .map(Expr::Array),
        ))
        .delimited_by(just(Token::BracketOpen), just(Token::BracketClose))
        .labelled("array")
        .boxed();

        // block ::= "{" statement* expr? "}"
        let block = stmt
            .clone()
            .repeated()
            .collect::<Vec<Stmt>>()
            .then(expr.clone().or_not())
            .delimited_by(just(Token::BraceOpen), just(Token::BraceClose))
            .map(|(mut statements, final_expr)| {
                if let Some(expr) = final_expr {
                    if !matches!(expr, Expr::Return(..) | Expr::Break(..)) {
                        statements.push(Stmt::Expr(Expr::Break(Box::new(expr))));
                    }
                }
                Expr::Block(statements)
            })
            .labelled("block")
            .boxed();

        // atom ::= literal | ident | array | "(" expr ")" | block
        let atom = choice((
            literal,
            ident_parser(src).map(String::from).map(Expr::Ident),
            array,
            expr.clone()
                .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
            block,
        ))
        .labelled("atom");

        let basic = {
            use chumsky::pratt::{infix, left, postfix, prefix, right};

            let binary = |associativity, token, op| {
                infix(associativity, just(token), move |l, r| {
                    Expr::Binary(op, Box::new(l), Box::new(r))
                })
            };

            atom.clone().pratt((
                // field ::= atom "." ident
                postfix(
                    7,
                    just(Token::Dot).ignore_then(ident_parser(src).map(String::from)),
                    |l, field| Expr::FieldAccess(Box::new(l), field),
                ),
                // subscript ::= field "." "[" expr "]"
                postfix(
                    6,
                    just(Token::Dot).ignore_then(
                        expr.clone()
                            .delimited_by(just(Token::BracketOpen), just(Token::BracketClose)),
                    ),
                    |l, index| Expr::Subscript(Box::new(l), Box::new(index)),
                ),
                // call ::= field "(" (expr ("," expr)*)? ")"
                postfix(
                    6,
                    expr.clone()
                        .separated_by(just(Token::Comma))
                        .collect::<Vec<Expr>>()
                        .delimited_by(just(Token::ParenOpen), just(Token::ParenClose)),
                    |l, args| Expr::Call(Box::new(l), args),
                ),
                // unary ::= ("-" | "not") call
                prefix(5, just(Token::Not), |r| {
                    Expr::Unary(UnaryOp::Not, Box::new(r))
                }),
                prefix(5, just(Token::Minus), |r| {
                    Expr::Unary(UnaryOp::Neg, Box::new(r))
                }),
                // mul_div ::= unary ("*" | "/") unary
                binary(left(4), Token::Star, BinaryOp::Mul),
                binary(left(4), Token::Slash, BinaryOp::Div),
                // add_sub ::= mul_div ("*" | "/") mul_div
                binary(left(3), Token::Plus, BinaryOp::Add),
                binary(left(3), Token::Minus, BinaryOp::Sub),
                // compare ::= add_sub ("==" | "!=" | "<" | "<=" | ">" | ">=") add_sub
                // TODO Require parentheses
                binary(left(2), Token::Eq, BinaryOp::Eq),
                binary(left(2), Token::NotEq, BinaryOp::NotEq),
                binary(left(2), Token::LessThan, BinaryOp::LessThan),
                binary(left(2), Token::LessThanEq, BinaryOp::LessThanEq),
                binary(left(2), Token::GreaterThan, BinaryOp::GreaterThan),
                binary(left(2), Token::GreaterThanEq, BinaryOp::GreaterThanEq),
                // assign ::= compare "=" compare
                binary(right(1), Token::Assign, BinaryOp::Assign),
                // break ::= "break" assign
                prefix(0, just(Token::Break), |r| Expr::Break(Box::new(r))),
                // return_value ::= "return" assign
                prefix(0, just(Token::Return), |r| Expr::Return(Box::new(r))),
            ))
        };

        let if_else = just(Token::If)
            .ignore_then(expr.clone())
            .then_ignore(just(Token::Then))
            .then(expr.clone())
            .then(just(Token::Else).ignore_then(expr.clone()).or_not())
            .map(|((cond, then_branch), else_branch)| {
                Expr::IfElse(
                    Box::new(cond),
                    Box::new(then_branch),
                    Box::new(else_branch.unwrap_or(Expr::Unit)),
                )
            });

        // return ::= "return"
        let r#return = just(Token::Return).to(Expr::Return(Box::new(Expr::Unit)));

        // expr ::= function | if_else | basic | atom
        choice((function, if_else, basic, atom, r#return))
            .labelled("expression")
            .boxed()
    })
}
