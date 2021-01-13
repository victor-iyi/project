// Copyright (c) 2020 Victor I. Afolabi
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
//! Command line interface.
//!
//! An ASCII art depiction may help explain this better.
//!
//! ```txt
//!                       Top Level App (lotlinx)                 TOP (binary name)
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
//! $ lotlinx init <repo>
//! $ lotlinx new <template> <name>
//! $ lotlinx git <remote> <name> --branch master
//!```
//!
//! `--branch`, like any other flags has a short form `-b`.
//!
//! Help messages for any subcommands.
//!
//! ```sh.
//! $ lotlinx --help
//! $ lotlinx help new
//! ```
//!
//! Notice you can add `--verbose` (`-V`) or `--quiet` (`-q`) on all levels.
//!
//! ```sh
//! $ lotlinx new <template> <name> --verbose
//!```
//!
//! You can also view more info: eg. version info
//!
//! ```sh
//! $ lotlinx --version
//! ```
//!
use crate::{git::GitOptions, util};

use clap::{App, AppSettings, Arg};

use std::{path::PathBuf, str::FromStr};

#[derive(Debug, Clone)]
pub enum TemplateType {
  /// Use a template from Git.
  Git(GitOptions),
  /// Use a local template.
  Local(PathBuf),
}

impl TemplateType {
  /// Initlize either a local or a remote template (with default branch).
  fn new(local: Option<&str>, git: Option<&str>) -> Self {
    match local {
      Some(_local) => Self::new_local(_local),
      None => match git {
        Some(repo) => Self::new_git(repo, None),
        None => panic!("`git` or `local` must be specified."),
      },
    }
  }

  /// Initialize a new git template with default branch.
  fn new_git(git: &str, branch: Option<&str>) -> Self {
    let branch = match branch {
      Some(b) => Some(b.to_string()),
      None => None,
    };

    TemplateType::Git(
      GitOptions::new(git, branch).unwrap_or_else(|err| panic!("{}", err)),
    )
  }

  /// Initlizes a new local template path.
  fn new_local(local: &str) -> Self {
    TemplateType::Local(
      PathBuf::from_str(local)
        .unwrap_or_else(|err| panic!("Invalid local path: {}", err)),
    )
  }
}

/// Command line argument configuration.
#[derive(Debug, Clone)]
pub struct Config {
  /// Project name.
  pub name: String,
  /// Project name and project's directory.
  pub path: PathBuf,
  /// Template type. git template? local template?
  pub template_type: TemplateType,
  /// Run verbosely.
  pub verbose: bool,
  /// Supress all output.
  pub quiet: bool,
}

impl Config {
  /// Creates a new configuration.
  pub fn new(
    path: &str,
    local: Option<&str>,
    git: Option<&str>,
    verbose: bool,
    quiet: bool,
  ) -> Self {
    Config {
      name: util::basename(path).into(),
      path: PathBuf::from(path),
      template_type: TemplateType::new(local, git),
      verbose,
      quiet,
    }
  }

  /// Create a new configuration with a given `TemplateType`.
  pub fn with_template_type(path: &str, template_type: TemplateType) -> Self {
    Config {
      name: util::basename(path).into(),
      path: PathBuf::from(path),
      template_type,
      verbose: false,
      quiet: false,
    }
  }

  /// Creates an empty configuration with default/zero-values.
  pub fn empty() -> Self {
    Config {
      name: String::new(),
      path: PathBuf::new(),
      template_type: TemplateType::new_local(""),
      verbose: false,
      quiet: false,
    }
  }
}

/// Lotlinx command line utilities.
pub struct Cli<'a> {
  /// Cli's configuration.
  pub config: Config,
  /// Command line argument matches.
  matches: clap::ArgMatches<'a>,
}

impl Default for Cli<'_> {
  fn default() -> Self {
    Self {
      matches: Self::default_args(),
      config: Config::empty(),
    }
  }
}

impl<'a> Cli<'a> {
  /// Creates default arguments with `Cli::default()`
  /// then parses the default arguments with `build_config()`.
  pub fn new() -> Cli<'a> {
    let mut cli = Self::default();
    cli.build_config();
    cli
  }

  /// Create new Cli instance from `clap::ArgMaches<'a>` instance.
  pub fn from_matches(matches: clap::ArgMatches<'a>) -> Self {
    Cli {
      matches,
      config: Config::empty(),
    }
  }

  /// Retrieve Cli's configuration.
  pub fn get_config(&self) -> &Config {
    &self.config
  }
}

// Priveate impl block.
impl<'a> Cli<'a> {
  /// Creates default `clap::ArgMaches` and builts it in `Cli::build_config()`.
  fn default_args() -> clap::ArgMatches<'a> {
    App::new(clap::crate_name!())
      .version(clap::crate_version!())
      .about(clap::crate_description!())
      .author(clap::crate_authors!())
      // Create a new lotlinx project
      .subcommand(
        // $ lotlinx new <template-path> <project-name>
        App::new("new")
          .about("Creates a new project from a local template.")
          .args(&[
            // Required args...
            Arg::with_name("template")
              .help("Path to a local template directory.")
              .index(1).required(true),
            Arg::with_name("name")
              .help("Name of the project / directory name.")
              .required(true),
          ])
      )
      .subcommand(
        // lotlinx git <repo> <project-name> --branch develop
        App::new("git")
          .about("Initalize project from a GitHub template")
          .setting(AppSettings::ArgRequiredElseHelp)
          .args(&[
            Arg::with_name("remote")
              .help("URL to remote repo or `owner/repo` for short.")
              .required(true),
            Arg::with_name("name")
              .help("Name of the project / directory name.")
              .required(true)
          ]).args(&[
            Arg::with_name("branch")
              .long("branch").short("b")
              .takes_value(true)
              .help("Sepcify which branch to checkout. If no brach is given the repo's `HEAD` branch is used.")
          ])
      )
      .subcommand(
        // $ lotlinx init <repo/local>
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
  fn build_config(&mut self) {
    // Process subcommands.
    match self.matches.subcommand() {
      // "new" subcommand.
      ("new", Some(sub_new)) => {
        // lotlinx new <local> <name>
        if let Some(local) = sub_new.value_of("template") {
          self.config.template_type = TemplateType::new_local(local);
        }

        // lotlinx new <local> <name>
        if let Some(name) = sub_new.value_of("name") {
          self.config.name = util::basename(name).into();
          self.config.path = PathBuf::from(name);
        }
      }
      // "git" subcommand.
      ("git", Some(sub_git)) => {
        // lotlinx git <remote> <name>
        if let Some(remote) = sub_git.value_of("remote") {
          // Set the remote with the branch (if given).
          self.config.template_type =
            TemplateType::new_git(remote, sub_git.value_of("branch"));
        }
        // lotlinx git <remote> <name>
        if let Some(name) = sub_git.value_of("name") {
          self.config.name = util::basename(name).into();
          self.config.path = PathBuf::from(name);
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
    };

    self.config.verbose = self.matches.is_present("verbose");
    self.config.quiet = self.matches.is_present("quiet");
  }
}
