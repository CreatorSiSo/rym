use insta::assert_snapshot;
use rymx::{std_lib, AriadneEmitter, Env};

#[test]
fn main() {
    insta::glob!("**/*.rym", |path| {
        let src = std::fs::read_to_string(path).unwrap();
        let mut out: Vec<u8> = vec![];
        let (sender, emitter) = AriadneEmitter::new(&mut out);
        let mut env = Env::from_constants(std_lib::CONSTANTS.into_iter().chain(std_lib::OTHER));

        std::thread::spawn(move || {
            let module = rymx::compile_module(sender.clone(), &src)?;
            rymx::interpret(sender, &mut env, module);
            Some(())
        });

        emitter.emit_all_blocking();
        assert_snapshot!(String::from_utf8(out).unwrap());
    })
}
