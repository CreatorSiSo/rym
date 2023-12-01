use insta::assert_snapshot;
use rymx::{interpret, std_lib, Diagnostics, Env, SourceId};

#[test]
fn main() {
	insta::glob!("**/*.rym", |path| {
		let src = std::fs::read_to_string(path).unwrap();
		let mut diag = Diagnostics::new(Box::new(std::io::sink()), Box::new(std::io::sink()));
		let mut env = Env::from_constants(std_lib::CONSTANTS.into_iter().chain(std_lib::OTHER));

		if let Ok(module) = rymx::compile_module(&mut diag, &src, SourceId::File(path.to_path_buf())) {
			interpret(&mut diag, &mut env, module);
		}
		assert_snapshot!(diag.outputs_dump() + &diag.reports_dump());
	})
}
