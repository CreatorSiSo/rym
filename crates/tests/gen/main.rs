use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string};
use std::path::Path;

#[derive(Debug)]
enum GenError {
	Args(&'static str),
	Format(String),
	Io(std::io::Error),
}

fn main() -> Result<(), GenError> {
	let Some(dir_path) = std::env::args().nth(1) else {
		return Err(GenError::Args("Expected absolute path to crates/tests"))
	};

	let read_dir = match read_dir(&dir_path) {
		Ok(read_dir) => read_dir,
		Err(err) => return Err(GenError::Io(err)),
	};

	let mut src_files = Vec::new();

	for read_dir_result in read_dir {
		match read_dir_result {
			Ok(entry) => {
				let file_path = entry.path();
				if file_path.is_file() && (file_path.extension() == Some(OsStr::new("rym"))) {
					println!("Reading: {:?}", entry.path());

					match read_to_string(&file_path) {
						Ok(src) => src_files.push((file_path, src)),
						Err(err) => return Err(GenError::Io(err)),
					};
				}
			}
			Err(err) => return Err(GenError::Io(err)),
		}
	}

	let mut generated =
		String::from("#[rustfmt::skip]\npub const SOURCES: [(&'static str, &'static str); ");
	generated.push_str(&src_files.len().to_string());
	generated.push_str("] = [");
	for (path, src) in src_files {
		match path.to_str() {
			Some(path_str) => generated.push_str(&((String::from("(r#\"") + path_str) + "\"#, r#\"")),
			None => {
				return Err(GenError::Format(format!(
					"Could not convert `{:?}` to string",
					path
				)))
			}
		}
		generated.push_str(&(src + "\"#), "));
	}
	generated.push_str("];");

	std::fs::write(Path::new(&dir_path).join("sources.rs"), generated).map_err(GenError::Io)
}
