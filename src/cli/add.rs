extern crate git2;
extern crate just;

use self::git2::Repository;
use std;

fn git_url() -> String {
  String::from("https://github.com/")
}

pub fn run(repo: &str, destination: std::path::PathBuf) -> Result<git2::Repository, git2::Error> {
  println!("Adding {}...", repo);
  let mut source = git_url();
  source.push_str(repo);

  Repository::clone(&source, &destination)
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;
  use std::path::Path;


  fn clear_just_test_directory() -> bool {
    match fs::remove_dir_all(".just-test") {
      Ok(_) => {
        match fs::create_dir(".just-test") {
          Ok(_) => false,
          Err(_) => false
        }
      },
      Err(_) => false
    }
  }

  #[test]
  fn clones_a_new_repo_if_one_does_not_exist () {
    clear_just_test_directory();

    let repo = run("radar/dot-files", Path::new(".just-test/radar/dot-files").to_path_buf());
    assert!(repo.is_ok());
  }

  #[test]
  fn fails_to_clone_if_repo_already_exists () {
    clear_just_test_directory();
    let create_dir = fs::create_dir_all(".just-test/radar/dot-files");
    assert!(create_dir.is_ok());

    let repo = run("radar/dot-files", Path::new(".just-test/radar/dot-files").to_path_buf());
    assert!(repo.is_err());
  }
}
