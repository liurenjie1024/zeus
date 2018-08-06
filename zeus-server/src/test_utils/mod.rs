pub mod parser;


use std::path::PathBuf;

pub fn file_in_project_dir(path: &str) -> PathBuf {
  let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  p.push(path);
  p
}
