// Copyright (c) 2020 Victor I. Afolabi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
//! Command line interface.
//!
//! An ASCII art depiction may help explain this better.
//!
//! ```txt
//!                       Top Level App (project)                 TOP (binary name)
//!                               |
//!            --------------------------------------------
//!          /       |            |       |       \        \
//!      init       new          git     help --verbose --quiet   LEVEL 1 (subcommands)
//!       |        /  \         /   \
//!     repo  template name  remote name                          LEVEL 2 (args)
//!       |                    |
//! --branch               --branch                               LEVEL 3 (flags)
//!```
//!
//! # Usage
//!
//! Subcommands
//!
//!```sh
//! $ project init <repo>
//! $ project new <template> <name>
//! $ project git <remote> <name> --branch master
//!```
//!
//! `--branch`, like any other flags has a short form `-b`.
//!
//! Help messages for any subcommands.
//!
//! ```sh.
//! $ project --help
//! $ project help new
//! ```
//!
//! Notice you can add `--verbose` (`-V`) or `--quiet` (`-q`) on all levels.
//!
//! ```sh
//! $ project new <template> <name> --verbose
//!```
//!
//! You can also view more info: eg. version info
//!
//! ```sh
//! $ project --version
//! ```
//!
use crate::{
  emoji,
  info::{ProjectInfo, TemplateOptions},
};

use clap::{App, AppSettings, Arg};
use console::style;

pub struct Arguments {
  /// Project information.
  pub project: ProjectInfo,
  /// Template options.
  pub template: TemplateOptions,
  /// Verbosity level.
  pub verbose: bool,
  /// Supress output.
  pub quiet: bool,
}

impl Arguments {
  pub fn new(name: &str, path: &str, branch: Option<&str>) -> Arguments {
    Arguments {
      project: ProjectInfo::from(name),
      template: TemplateOptions::new(path, branch),
      verbose: false,
      quiet: false,
    }
  }
}

impl From<&str> for Arguments {
  fn from(path: &str) -> Arguments {
    Arguments {
      project: ProjectInfo::default(),
      template: TemplateOptions::new(path, None),
      verbose: false,
      quiet: false,
    }
  }
}

impl Default for Arguments {
  fn default() -> Arguments {
    Arguments {
      project: ProjectInfo::default(),
      template: TemplateOptions::default(),
      verbose: false,
      quiet: false,
    }
  }
}

/// Project command line utilities.
pub struct Cli<'a> {
  /// Command line arguments.
  pub args: Arguments,
  /// Command line argument matches.
  matches: clap::ArgMatches<'a>,
}

impl Default for Cli<'_> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Cli<'a> {
  /// Creates default arguments with `Cli::default()`
  /// then parses the default arguments with `parse_args()`.
  pub fn new() -> Cli<'a> {
    let mut cli = Self {
      args: Arguments::default(),
      matches: Self::default_args(),
    };
    cli.parse_args();
    cli
  }

  /// Create new Cli instance from `clap::ArgMaches<'a>` instance.
  pub fn from_matches(matches: clap::ArgMatches<'a>) -> Self {
    Cli {
      matches,
      args: Arguments::default(),
    }
  }
}

// Priveate impl block.
impl<'a> Cli<'a> {
  /// Creates default `clap::ArgMaches` and builts it in `Cli::parse_args()`.
  fn default_args() -> clap::ArgMatches<'a> {
    App::new(clap::crate_name!())
      .version(clap::crate_version!())
      .about(clap::crate_description!())
      .author(clap::crate_authors!())
      // Create a new project project
      .subcommand(
        // $ project new <template-path> <project-name>
        App::new("new")
          .about("Creates a new project from a local template.")
          .args(&[
            // Required args...
            Arg::with_name("template")
              .help("Path to a local template directory.")
              .index(1).required(true),
            Arg::with_name("name")
              .help("Name of the project / directory name.")
              .index(2).allow_hyphen_values(true),
          ])
      )
      .subcommand(
        // project git <repo> <project-name> --branch develop
        App::new("git")
          .about("Initalize project from a GitHub template")
          .setting(AppSettings::ArgRequiredElseHelp)
          .args(&[
            Arg::with_name("remote")
              .help("URL to remote repo or `owner/repo` for short.")
              .index(1)
              .required(true),
            Arg::with_name("name")
              .help("Name of the project / directory name.")
              .index(2)
              .takes_value(true)
          ]).args(&[
            Arg::with_name("branch")
              .long("branch").short("b")
              .takes_value(true)
              .help("Sepcify which branch to checkout. If no brach is given the repo's `HEAD` branch is used.")
          ])
      )
      .subcommand(
        // $ project init <repo/local>
        App::new("init")
          .about("Initialize new project from current dir.")
          .setting(AppSettings::SubcommandRequiredElseHelp)
          .arg(
            Arg::with_name("repo")
              .required(true)
              .help("Path to a remote template or a local template."),
          )
          .arg(
            Arg::with_name("branch")
              .short("b")
              .long("branch")
              .help("Branch name to checkout.")
              .takes_value(true),
          ),
      )
      .args(&[
        // Flags: [must have `.short()` or `.long()`]
        // Options: [must have either `.short()` or `.long()` & `takes_value(true)]
        Arg::with_name("verbose")
          .short("V")
          .long("verbose")
          .help("Run verbosely."),
        Arg::with_name("quiet")
          .short("q")
          .long("quiet")
          .help("Supress all output. Progress is not reported to the standard error stream."),
      ])
      .get_matches()
  }

  /// Builds the default argument created in `Cli::default_args()` and retrives the values.
  fn parse_args(&mut self) {
    // Process subcommands.
    match self.matches.subcommand() {
      // "new" subcommand.
      ("new", Some(sub_new)) => {
        // project new <local> <name>
        let path = sub_new.value_of("template").unwrap();
        let name = sub_new.value_of("name").unwrap();
        self.args = Arguments::new(name, path, None);
      }
      // "git" subcommand.
      ("git", Some(sub_git)) => {
        // project git <remote> <name>
        let path = sub_git.value_of("remote").unwrap();
        let name = sub_git.value_of("name").unwrap();
        let branch = sub_git.value_of("branch");
        self.args = Arguments::new(name, path, branch);
      }
      // "init" subcommand.
      ("init", Some(sub_init)) => {
        // project init <repo>
        let path = sub_init.value_of("repo").unwrap();
        // TODO: Add `branch` to arguments.
        self.args = Arguments::from(path);
      }
      _ => {
        // Unrecognized command or above subcommands was not used.
        eprintln!(
          "{} {} {}",
          emoji::SHRUG,
          style("Unrecognized command.\n").bold().yellow(),
          style(&self.matches.usage()).bold().yellow()
        );
        std::process::exit(0);
      }
    };

    self.args.verbose = self.matches.is_present("verbose");
    self.args.quiet = self.matches.is_present("quiet");
  }
}
