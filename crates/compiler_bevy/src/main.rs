use bevy::prelude::*;
use rym_lexer::tokenize;
use std::ops::Range;

#[derive(Resource)]
struct ProjectDirectory(std::path::PathBuf);

#[derive(Debug, Component)]
struct SourceFile {
	path: std::path::PathBuf,
	data: Option<String>,
}

#[derive(Debug, Component)]
struct Span(Range<usize>);

#[derive(Debug, Component)]
struct Path(Vec<String>);

#[derive(Debug, Component)]
enum Expr {
	Literal,
}

#[derive(Debug, Component)]
struct Var {
	name: String,
	value: Option<Expr>,
}

#[derive(Debug, Component)]
enum Stmt {
	Var(Var),
	Expr(Expr),
}

#[derive(Debug, Component)]
struct Block(Vec<Stmt>);

#[derive(Bundle)]
struct FunctionBundle {
	name: Path,
	return_type: Path,
	body: Block,
}

fn main() {
	App::new()
		.insert_resource(current_project_dir())
		.add_startup_system(discover_files)
		.add_system(read_files)
		.run();
}

fn current_project_dir() -> ProjectDirectory {
	ProjectDirectory(std::env::current_dir().unwrap())
}

fn discover_files(mut commands: Commands, project_dir: Res<ProjectDirectory>) {
	let mut pattern = project_dir.0.clone();
	pattern.push("**/*.ry[sm]");
	let files = glob::glob(pattern.to_str().unwrap()).unwrap().filter_map(Result::ok);

	for file in files {
		commands.spawn(SourceFile { path: file, data: None });
	}
}

fn read_files(mut files: Query<&mut SourceFile>) {
	for mut file in &mut files {
		file.data = std::fs::read_to_string(&file.path).ok();
		if let Some(data) = &file.data {
			tokenize(data).collect::<Vec<_>>();
		}
		println!("{file:?}");
	}
}

// query.par_for_each(5, |item| {})
