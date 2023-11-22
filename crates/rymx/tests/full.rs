use insta::assert_snapshot;
use rymx::Diagnostics;

#[test]
fn main() {
	insta::glob!("**/*.rym", |path| {
		let src = std::fs::read_to_string(path).unwrap();
		let mut diag = Diagnostics::new(Box::new(std::io::stderr()), Box::new(std::io::stderr()));
		let _ = rymx::compile_module(&mut diag, &src);
		assert_snapshot!(diag.outputs_dump() + &diag.reports_dump());
	})
}
