use insta::assert_snapshot;
use rymx::{std_lib, AriadneEmitter, Env};

#[test]
fn main() {
    insta::glob!("**/*.rym", |path| {
        let src = std::fs::read_to_string(path).unwrap();
        let mut out: Vec<u8> = vec![];
        let writer = strip_ansi_escapes::Writer::new(&mut out);
        let (sender, mut emitter) = AriadneEmitter::new(writer);
        let src_id = emitter.source_map.add(path.to_string_lossy(), &src);

        std::thread::spawn(move || {
            let mut env = Env::new(sender.clone())
                .with_constants(std_lib::CONSTANTS.into_iter().chain(std_lib::OTHER));
            let module = rymx::compile_module(sender, &src, src_id)?;
            rymx::interpret(&mut env, module);
            Some(())
        });

        emitter.emit_all_blocking();
        assert_snapshot!(String::from_utf8(out).unwrap());
    })
}
