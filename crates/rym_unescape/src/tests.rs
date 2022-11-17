use crate::{unquote, Error};

#[test]
fn test_unescape() {
	assert_eq!(unquote("").unwrap_err(), Error::NotEnoughChars,);
	assert_eq!(unquote("foobar").unwrap_err(), Error::UnrecognizedQuote,);
	assert_eq!(unquote("'foobar").unwrap_err(), Error::UnexpectedEOF,);
	assert_eq!(unquote("'foo'bar'").unwrap_err(), Error::IllegalChar,);
	assert_eq!(unquote("'foobar\\'").unwrap_err(), Error::UnexpectedEOF,);
	assert_eq!(unquote("'\\q'").unwrap_err(), Error::UnrecognizedEscape,);
	assert_eq!(unquote("'\\00'").unwrap_err(), Error::UnrecognizedEscape,);

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

	assert_eq!(unquote("``").unwrap(), "");
	assert_eq!(unquote(r#"`\``"#).unwrap(), "`");

	assert_eq!(unquote("'\\n'").unwrap(), "\n");
	assert_eq!(unquote("'\\101'").unwrap(), "A");
	assert_eq!(unquote("'\\x76'").unwrap(), "\x76");
	assert_eq!(unquote("'\\u2714'").unwrap(), "\u{2714}");
	assert_eq!(unquote("'\\U0001f427'").unwrap(), "\u{1f427}");
}
