// Copyright (c) 2020 Victor I. Afolabi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
//! Command line interface.
//! An ASCII art depiction may help explain this better. Using a fictional version of git as the demo
//! subject. Imagine the following are all subcommands of git (note, the author is aware these aren't
//! actually all subcommands in the real git interface, but it makes explanation easier)
//!
//!            Top Level App (lotlinx)                         TOP
//!                           |
//!       ----------------------------------------
//!      /            |                \          \
//!    new          push              add       commit     LEVEL 1
//!     |           / \            /    \       |
//!  project  origin   remote    ref    name   message     LEVEL 2
//!           /                  /\
//!        path            remote  local                   LEVEL 3
//!
//! Given the above fictional subcommand hierarchy, valid runtime uses would be (not an all inclusive
//! list):
//!
//! $ lotlinx new project
//! $ lotlinx push origin path
//! $ lotlinx add ref local
//! $ lotlinx commit message
//!
//! Notice only one command per "level" may be used. You could not, for example, do:
//!
//! $ lotlinx new project push origin path
//!
//! It's also important to know that subcommands each have their own set of matches and may have args
//! with the same name as other subcommands in a different part of the tree hierarchy (i.e. the arg
//! names aren't in a flat namespace).
//!
//! In order to use subcommands in clap, you only need to know which subcommand you're at in your
//! tree, and which args are defined on that subcommand.
//!
//! Let's make a quick program to illustrate. We'll be using the same example as above but for
//! brevity sake we won't implement all of the subcommands, only a few.

use clap::{App, AppSettings, Arg};

/// Lotlinx command line utilities.
#[derive(Debug)]
pub struct Cli<'a> {
  #[doc(hidden)]
  pub matches: clap::ArgMatches<'a>,
}

impl<'a> Cli<'a> {
  pub fn new() -> Cli<'a> {
    Cli {
      matches: Cli::get_matches(),
    }
  }

  pub fn get_matches() -> clap::ArgMatches<'a> {
    App::new("lotlinx")
      .version(clap::crate_version!())
      .author(clap::crate_authors!())
      .about("Project template generator")
      // Create a new lotlinx project
      .subcommand(
        App::new("new")
          .about("Creates a new project template")
          .arg(
            Arg::with_name("project")
              .help("Name of the project.")
              .required(true),
          )
          .arg(
            Arg::with_name("engine")
              .short("e")
              .long("engine")
              .help("Template engnine to be used.")
              .takes_value(true)
              .possible_values(&["tf", "keras"]),
          )
          .arg(
            Arg::with_name("runtime")
              .short("r")
              .long("runtime")
              .help("GCS runtime version.")
              .default_value("2.1")
              .takes_value(true),
          ),
      )
      .subcommand(
        App::new("init")
          .about("Initialize new project from current dir.")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .subcommand(
            App::new("remote") // Subcommands can have their own subcommands,
              // which in turn have their own subcommands
              .about("pushes remote things")
              .arg(
                Arg::with_name("repo")
                  .required(true)
                  .help("The remote repo to push things to"),
              ),
          )
          .subcommand(App::new("local").about("pushes local things")),
      )
      .subcommand(
        App::new("git")
          .about("Initalize project from a GitHub")
          .version("v2.0 (I'm versioned differently") // or different version from their parents
          .setting(AppSettings::ArgRequiredElseHelp) // They can even have different settings
          .arg(
            Arg::with_name("repo")
              .help("URL to remote repo")
              .required(true),
          ),
      )
      .get_matches()
  }
}

impl Cli<'_> {
  pub fn build(&self) {
    // Process subcommands.
    match self.matches.subcommand() {
      // "new" subcommand.
      ("new", Some(sub_new)) => {
        // lotlinx new <project>
      }
      // "init" subcommand.
      ("init", Some(sub_init)) => {
        // lotlinx init
      }
      ("git", Some(sub_git)) => {
        // lotlinx git <repo>
      }
      _ => {
        eprintln!("Unrecognized command.");
      }
    }
  }
}

impl<'a> Default for Cli<'_> {
  fn default() -> Self {
    Cli::new()
  }
}
