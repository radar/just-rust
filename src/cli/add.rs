extern crate git2;
extern crate just;

use std::path::PathBuf;

#[derive(Debug)]
pub enum AddError {
  AlreadyExists,
  GenericGit2Error,
}

// TODO: This is an attempt to shield main.rs from git2::Error handling
// Am I even doing it right? Is this the "Rust way"?
impl From<git2::Error> for AddError {
    fn from(e: git2::Error) -> Self {
      match e.class() {
        git2::ErrorClass::Invalid => {
          match e.code() {
            git2::ErrorCode::Exists => AddError::AlreadyExists,
            _ => AddError::GenericGit2Error,
          }
        }
        _ => AddError::GenericGit2Error
      }
    }
}

pub fn run(repo: &str, destination: PathBuf) -> Result<git2::Repository, AddError> {
  match git2::Repository::clone(&repo, &destination) {
    Ok(repo) => Ok(repo),
    Err(err) => Err(AddError::from(err)),
  }
}

#[cfg(test)]
mod test {
  extern crate tempdir;

  use super::*;
  use std;
  use std::fs;
  use self::tempdir::TempDir;

  fn create_test_directory() -> std::result::Result<TempDir, std::io::Error> {
    let dir = TempDir::new("just-test")?;

    Ok(dir)
  }

  #[test]
  fn clones_a_new_repo_if_one_does_not_exist () {
    let destination = create_test_directory().unwrap().into_path();

    let result = run("https://github.com/radar/dot-files", destination);

    assert!(result.is_ok());
  }

  #[test]
  fn fails_to_clone_if_repo_already_exists () {
    let destination = create_test_directory().unwrap();
    let dotfile = destination.path().join("gitaliases");
    let destination_path = destination.into_path();
    let create_dir = fs::create_dir_all(destination_path.clone());
    assert!(create_dir.is_ok());

    let touch_file = fs::OpenOptions::new().create(true).write(true).open(dotfile);
    assert!(touch_file.is_ok());

    let result = run("https://github.com/radar/dot-files", destination_path);
    assert!(result.is_err());
  }
}
