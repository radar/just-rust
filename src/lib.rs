use std::env;

fn directory() -> std::path::PathBuf {
  let home = match env::home_dir() {
    Some(path) => path,
    None => {
      panic!("Couldn't work out your home directory")
    }
  };

  home.join(".just")
}

pub fn path(other_path: &str) -> std::path::PathBuf {
  directory().join(other_path)
}
