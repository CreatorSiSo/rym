use crate::{unquote, Error};

#[test]
fn test_unescape() {
	assert_eq!(unquote("").unwrap_err(), Error::NotEnoughChars { need: 2 },);
	assert_eq!(unquote("foobar").unwrap_err(), Error::UnrecognizedQuote,);
	assert_eq!(unquote("'foobar").unwrap_err(), Error::Unterminated,);
	assert_eq!(unquote("'foo'bar'").unwrap_err(), Error::IllegalChar,);
	assert_eq!(unquote("'foobar\\'").unwrap_err(), Error::Unterminated,);
	assert_eq!(unquote("'test  \\q'").unwrap_err(), Error::UnrecognizedEscapePrefix("\\q".into()),);
	assert_eq!(unquote("'\\00'").unwrap_err(), Error::NotEnoughChars { need: 1 },);

	assert_eq!(
		unquote(r#""\"Fran & Freddie's Diner	☺\\\"""#).unwrap(),
		r#""Fran & Freddie's Diner	☺\""#,
	);
	assert_eq!(unquote(r#""""#).unwrap(), "");
	assert_eq!(unquote(r#""\"""#).unwrap(), r#"""#);

	assert_eq!(
		unquote(r#"'"Fran & Freddie\'s Diner	☺\\"'"#).unwrap(),
		r#""Fran & Freddie's Diner	☺\""#,
	);
	assert_eq!(unquote("''").unwrap(), "");
	assert_eq!(unquote(r#"'\''"#).unwrap(), "'");

	assert_eq!(unquote("'\\n'").unwrap(), "\n");
	assert_eq!(unquote("'\\101'").unwrap(), "A");
	assert_eq!(unquote("'\\x76'").unwrap(), "\x76");
	assert_eq!(unquote("'\\u2714'").unwrap(), "\u{2714}");
	assert_eq!(unquote("'\\U0001f427'").unwrap(), "\u{1f427}");

	assert_eq!(unquote(r#""Hello World!\n""#).unwrap(), "Hello World!\n");
	assert_eq!(unquote(r#"'c'"#).unwrap(), "c");
	assert_eq!(unquote(r#"'\n'"#).unwrap(), "\n");
	assert_eq!(unquote(r#"'\u24B6'"#).unwrap(), "\u{24B6}");
	assert_eq!(unquote(r#"'\u8DEF'"#).unwrap(), "\u{8DEF}");
}
