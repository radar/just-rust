extern crate ansi_term;
extern crate clap;
extern crate just;
mod command;

use clap::{App, Arg, SubCommand};
use command::{add_command, use_command};

fn main() {
  let matches = App::new("just").version("v1.0-beta")
    .subcommand(SubCommand::with_name("add")
      .arg(Arg::with_name("repo").required(true))
    )
    .subcommand(SubCommand::with_name("use")
      .arg(Arg::with_name("repo").required(true))
      .arg(
        Arg::with_name("files")
          .required(true)
          .min_values(1)
      )
    )
  .get_matches();

  if let Some(matches) = matches.subcommand_matches("add") {
    add_command::run(matches);
  }

  if let Some(matches) = matches.subcommand_matches("use") {
    use_command::run(matches);
  }
}
