extern crate clap;
extern crate git2;
extern crate just;

use std::path::PathBuf;

pub fn run(matches: &clap::ArgMatches) {
  let repo = matches.value_of("repo").unwrap();
  let files: Vec<&str> = matches.values_of("files").unwrap().collect();
  let file_paths: Vec<&PathBuf> = files.iter().map(|f|
    just::path(&format!("{repo}/{f}", repo=repo, f=f))
  ).collect();


  validate_files_are_present(&file_paths);
}

fn validate_files_are_present(files: Vec<&PathBuf>) -> Result<&[PathBuf], Vec<&PathBuf>> {
  let mut missing_files = files.iter().filter(|f| !f.exists());
  if missing_files.by_ref().count() == 0 {
    return Ok(files);
  }

  Err(missing_files.collect())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn uses_a_single_file() {
    run("radar/dot-files", &["gitaliases"])
  }

  #[test]
  fn uses_multiple_files() {
    // Check that .just-aliases has the files written to it
  }

  #[test]
  fn complains_if_a_file_does_not_exist() {
    run("radar/dot-files", &["missing-in-action"])
  }
}
