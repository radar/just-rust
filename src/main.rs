extern crate ansi_term;
extern crate clap;
extern crate just;
mod cli;

use ansi_term::Colour::{Red, Green};
use clap::{App, Arg, SubCommand};
use cli::add;

fn main() {
  let matches = App::new("just").version("v1.0-beta")
    .subcommand(SubCommand::with_name("add")
      .arg(Arg::with_name("repo").required(true))
    )
  .get_matches();

  if let Some(matches) = matches.subcommand_matches("add") {
    let mut url = String::from("https://github.com/");
    let repo_name = matches.value_of("repo").unwrap();
    url.push_str(repo_name);

    println!("{}", Green.paint(format!("Cloning {}...", repo_name)));

    match add::run(&url, just::path(repo_name)) {
      Ok(_repo) => println!("{}", Green.paint(format!("Added {}", repo_name))),

      Err(add::AddError::AlreadyExists) => {
        println!("{}", Red.paint(format!("You have already added {}.", repo_name)));
      }

      Err(add::AddError::GenericGit2Error) => {
       println!("{}", Red.paint(format!("Something went wrong. Please try again.")));
      }
    }
  }
}
