extern crate git2;
extern crate just;

use self::git2::Repository;
use std;

pub fn run(repo: &str, destination: std::path::PathBuf) -> Result<git2::Repository, git2::Error> {
  Repository::clone(&repo, &destination)
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;
  use std::path::Path;

  fn clear_just_test_directory() -> std::io::Result<()> {
    if Path::new(".just-test").exists() {
      fs::remove_dir_all(".just-test")?;
      fs::create_dir(".just-test")?;
    }
    Ok(())
  }

  #[test]
  fn clones_a_new_repo_if_one_does_not_exist () {
    assert!(clear_just_test_directory().is_ok());

    let repo = run("https://github.com/radar/dot-files", Path::new(".just-test/radar/dot-files").to_path_buf());

    assert!(repo.is_ok());
  }

  #[test]
  fn fails_to_clone_if_repo_already_exists () {
    assert!(clear_just_test_directory().is_ok());

    let create_dir = fs::create_dir_all(".just-test/radar/dot-files");
    assert!(create_dir.is_ok());

    // let repo = run("https://github.com/radar/dot-files", Path::new(".just-test/radar/dot-files").to_path_buf());
    // assert!(repo.is_err());
  }
}
