use crate::{insta_assert_parser, parse_script_file};
use indoc::indoc;

#[test]
fn empty() {
	insta_assert_parser! {
		parse_script_file;

		indoc!(r#"
		// This is a comment
		print("What's your name?");
		let name: String = read_line(stdin);
		print("Hi, ", name, "!");"#),

		indoc!(r#"
		let name = readLine(stdin);
		if name == "" then
			print("Poor soul, you lost your name?")
		else if name == "name" then
			print("Very funny, your name is name.")
		else
			print("Hi, ", name, "!");"#),
	}
}
