use crate::{insta_assert_parser, script_file_parser};
use indoc::indoc;

#[test]
fn empty() {
	insta_assert_parser! {
		script_file_parser();

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
