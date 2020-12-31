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
pub struct Cli<'a> {
  #[doc(hidden)]
  pub matches: clap::ArgMatches<'a>,
}

impl<'a> Cli<'a> {
  /// Create an empty Cli. If you want Cli with default arguments,
  /// consider using `Cli::default()`.
  pub fn new() -> Cli<'a> {
    Cli {
      matches: App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .author(clap::crate_authors!())
        .get_matches(),
    }
  }

  /// Create new Cli instance from `clap::ArgMaches<'a>` instance.
  pub fn from_matches(matches: clap::ArgMatches<'a>) -> Cli {
    Cli { matches }
  }

  /// Creates default `clap::ArgMaches` and builts it in `Cli::build_default()`.
  pub fn default_args() -> clap::ArgMatches<'a> {
    App::new(clap::crate_name!())
      .version(clap::crate_version!())
      .about(clap::crate_description!())
      .author(clap::crate_authors!())
      // Create a new lotlinx project
      .subcommand(
        App::new("new")
          .about("Creates a new project template")
          .arg(
            Arg::with_name("project")
              .help("Name of the project.")
              .required(true),
          )
          .args(&[
            // Flags: [must have `.short()` or `.long()`]
            // Options: [must have either `.short()` or `.long()` & `takes_value(true)]
            Arg::with_name("engine")
              .short("e")
              .long("engine")
              .help("Template engnine to be used.")
              .takes_value(true)
              .default_value("tf")
              .possible_values(&["tf", "keras"]),
            Arg::with_name("runtime")
              .short("r")
              .long("runtime")
              .help("GCS runtime version.")
              .default_value("2.1")
              .takes_value(true),
            Arg::with_name("py_version")
              .long("py_version")
              .help("Python version.")
              .default_value("3.7")
              .takes_value(true),
            Arg::with_name("bucket")
              .short("b")
              .long("bucket")
              .help("GCS bucket")
              .takes_value(true)
              .default_value("lotlinxdata"),
          ]),
      )
      .subcommand(
        App::new("git")
          .about("Initalize project from a GitHub template")
          .setting(AppSettings::ArgRequiredElseHelp)
          .arg(
            Arg::with_name("repo")
              .help("URL to remote repo")
              .required(true),
          ),
      )
      .subcommand(
        App::new("init")
          .about("Initialize new project from current dir.")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .arg(
            Arg::with_name("repo")
              .required(true)
              .help("The remote repo to push things to"),
          )
          .arg(
            Arg::with_name("branch")
              .short("b")
              .long("branch")
              .help("Branch name to checkout.")
              .takes_value(true)
              .default_value("master"),
          ),
      )
      .get_matches()
  }
}

impl<'a> Cli<'a> {
  /// Modifies the `matches` field of `Cli`.
  pub fn set_matches(mut self, matches: clap::ArgMatches<'a>) -> Cli<'a> {
    self.matches = matches;
    self
  }

  /// Builds the default argument created in `Cli::default_args()` and retrives the values.
  pub fn build_default(&self) {
    // Process subcommands.
    match self.matches.subcommand() {
      // "new" subcommand.
      ("new", Some(sub_new)) => {
        // lotlinx new <project>
        if let Some(project) = sub_new.value_of("project") {
          println!("Project name: {}", project);
        }
        // lotlinx new <project> --engine tf
        if let Some(engine) = sub_new.value_of("engine") {
          println!("Template engine: {}", engine);
        }
        // lotlinx new <project> --runtime 2.1
        if let Some(runtime) = sub_new.value_of("runtime") {
          println!("GCS runtime: {}", runtime);
        }
        // lotlinx new <project> --py_version 3.7
        if let Some(py_version) = sub_new.value_of("py_version") {
          println!("Python version: {}", py_version);
        }
        // lotlinx new <project> --bucket lotlinxdata
        if let Some(bucket) = sub_new.value_of("bucket") {
          println!("GCS bucket: {}", bucket);
        }
      }
      // "git" subcommand.
      ("git", Some(sub_git)) => {
        // lotlinx git <repo>
        if let Some(repo) = sub_git.value_of("repo") {
          println!("Git repo: {}", repo);
        }
        // lotlinx git <repo> --branch develop
        if let Some(branch) = sub_git.value_of("branch") {
          println!("Branch selected: {}", branch);
        }
      }
      // "init" subcommand.
      ("init", Some(_sub_init)) => {
        // lotlinx init
      }
      _ => {
        // Unrecognized command or above subcommands was not used.
        eprintln!("Unrecognized command.\n{}", self.matches.usage());
      }
    }
  }
}

impl Default for Cli<'_> {
  fn default() -> Self {
    Self {
      matches: Self::default_args(),
    }
  }
}
