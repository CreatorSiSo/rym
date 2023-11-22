use insta::assert_snapshot;
use rymx::{Diagnostics, SourceId};

#[test]
fn main() {
	insta::glob!("**/*.rym", |path| {
		let src = std::fs::read_to_string(path).unwrap();
		let mut diag = Diagnostics::new(Box::new(std::io::sink()), Box::new(std::io::sink()));
		let _ = rymx::compile_module(&mut diag, &src, SourceId::File(path.to_path_buf()));
		assert_snapshot!(diag.outputs_dump() + &diag.reports_dump());
	})
}
