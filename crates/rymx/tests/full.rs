use insta::assert_snapshot;
use rymx::{interpret, std_lib, AriadneEmitter, Env};

#[test]
fn main() {
    insta::glob!("**/*.rym", |path| {
        let src = std::fs::read_to_string(path).unwrap();
        let mut out = String::new();
        let (sender, emitter) = AriadneEmitter::new_string_out(&mut out);
        let mut env = Env::from_constants(std_lib::CONSTANTS.into_iter().chain(std_lib::OTHER));

        std::thread::spawn(move || {
            if let Some(module) = rymx::compile_module(sender.clone(), &src) {
                interpret(sender, &mut env, module);
            }
        });

        for diagnostic in emitter.receiver.iter() {
            emitter.emit(diagnostic);
            println!("Finished");
        }

        assert_snapshot!(out);
    })
}
