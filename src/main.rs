extern crate ansi_term;
extern crate clap;
extern crate just;
mod cli;

use ansi_term::Colour::Red;
use clap::{App, Arg, SubCommand};

fn main() {
  let matches = App::new("fake").version("v1.0-beta")
    .subcommand(SubCommand::with_name("add")
      .arg(Arg::with_name("repo").required(true))
    )
  .get_matches();

  if let Some(matches) = matches.subcommand_matches("add") {
    let repo = matches.value_of("repo").unwrap();

    match cli::add::run(repo, just::path(repo)) {
      Ok(repo) => println!("{}", repo.path().display()),
      Err(err) => {
        println!("{}", Red.paint("Failed to add repo"));
        println!("{}", Red.paint(err.to_string()))
      }
    }
  }
}
