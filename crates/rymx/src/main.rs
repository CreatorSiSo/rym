use clap::{arg, command, Command};
use rustyline::{error::ReadlineError, Editor};
use rymx::{compile_module, compile_stmt, interpret, AriadneEmitter, Env};
use std::{fs::read_to_string, path::PathBuf};

#[derive(Debug)]
struct Arguments {}

fn main() -> anyhow::Result<()> {
    let mut command = command!()
        .arg(arg!(-w --write <"results|reports"> ... "What to write into a debug file"))
        .subcommand(Command::new("repl").about("Start a repl session"))
        .subcommand(
            Command::new("run")
                .about("Execute a file")
                .arg(arg!(<file> "File to execute")),
        );

    let help_str = command.render_help();
    let global_matches = command.get_matches();
    let write_flags: Vec<String> = global_matches
        .get_many("write")
        .map_or(vec![], |option| option.cloned().collect());

    match global_matches.subcommand() {
        Some(("repl", _)) => cmd_repl(write_flags)?,
        Some(("run", sub_matches)) => cmd_run(
            write_flags,
            sub_matches.get_one::<String>("file").unwrap().into(),
        )?,
        _ => print!("{}", help_str.ansi()),
    }

    Ok(())
}

fn cmd_repl(_write_flags: Vec<String>) -> anyhow::Result<()> {
    let mut editor: Editor<(), _> = Editor::new()?;
    if editor.load_history(".history").is_err() {
        println!("No previous history.");
    }

    // let debug_out = if write_flags.is_empty() {
    //     // Deletes repl.debug to avoid confusion and stupid debugging on
    //     // why my program does not output debug info, just because I set
    //     // the wrong flags and its not supposed to do anything
    //     let _ = std::fs::remove_file("repl.debug");
    //     Box::new(std::io::sink())
    // } else {
    //     // Overwrites the previous contents of repl.debug
    //     Box::new(File::create(PathBuf::from("repl.debug"))?)
    // };

    let (sender, mut emitter) = AriadneEmitter::new(std::io::stderr());
    let mut env = Env::new(sender.clone()).with_constants(rymx::std_lib::CONSTANTS);
    let src_id = emitter.source_map.add("repl", "");
    loop {
        let readline = editor.readline("➤ ");

        match readline {
            Ok(line) => {
                match line.as_str().trim_end() {
                    "" => continue,
                    ":help" => continue,
                    _ => (),
                }

                editor.add_history_entry(&line).unwrap();
                emitter.source_map.replace(src_id, &line);

                compile_stmt(sender.clone(), &line, src_id)
                    .and_then(|expr| interpret(&mut env, expr))
                    .inspect(|value| println!("{value}"));
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }

        emitter.emit_all();
    }

    editor.save_history(".history")?;
    Ok(())
}

fn cmd_run(_write_flags: Vec<String>, path: PathBuf) -> anyhow::Result<()> {
    let src = read_to_string(&path)?;
    // TODO reset current_dir when executing non const code
    // let prev_current_dir = std::env::current_dir()?;
    // std::env::set_current_dir(path.parent().unwrap())?;
    // let debug_out = if write_flags.is_empty() {
    //     Box::new(std::io::sink())
    // } else {
    //     Box::new(File::create(path.with_extension("debug"))?)
    // };

    let (sender, mut emitter) = AriadneEmitter::new(std::io::stderr());
    let src_id = emitter.source_map.add(path.to_string_lossy(), &src);

    std::thread::spawn(move || {
        let module = compile_module(sender.clone(), &src, src_id)?;
        let mut env = Env::new(sender).with_constants(rymx::std_lib::CONSTANTS);
        interpret(&mut env, module);
        Some(())
    });

    emitter.emit_all_blocking();
    Ok(())
}
