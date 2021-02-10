//! `project` is a command line utility to help developers get up and running as quickly as possible with a new project leveraging other
//! projects or pre-existing git repository as a template.
//!
//! It has a very simple and intuitive commands to help you pull project locally or
//! from a remote git repository. Templates could also be customized using templating engines such as [`Handlebars`] or [`Liquid`]. A
//! configuration file (`"template.toml"`) can also help developers customize how the new project is generated.
//!
//! [`project`]: https://github.com/victor-iyi/project
//!
//! ## Command Line Interface (CLI)
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
//! ```
//!
//! ### Usage
//!
//! Let's start by asking for `--help` and see what we get.
//!
//! ```sh
//! $ project --help
//! project 0.1.0
//! Victor I. Afolabi <javafolabi@gmail.com
//! Create a new project from existing project/template.
//!
//! USAGE:
//!     project [FLAGS] [SUBCOMMAND]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -q, --quiet      Supress all output. Progress is not reported to the standard error stream.
//!         --version    Prints version information
//!     -V, --verbose    Run verbosely.
//!
//! SUBCOMMANDS:
//!     git     Initalize project from a GitHub template
//!     help    Prints this message or the help of the given subcommand(s)
//!     init    Initialize new project from current dir.
//!     new     Creates a new project from a local template.
//! ```
//!
//!
//! You can get help messages on different subcommands provided:
//!
//! ```sh.
//! $ project help new
//! project-new
//! Creates a new project from a local template.
//!
//! USAGE:
//!     project new <template> [name]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! ARGS:
//!     <template>    Path to a local template directory.
//!     <name>        Name of the project / directory name.
//! ```
//!
//!
//! A simple view to how you can invoke the subcommands:
//!
//! ```sh
//! $ project init <repo>
//! $ project new <template> <name>
//! $ project git <remote> <name> --branch master
//! ```
//!
//! To start a new project from a local template:
//!
//! ```sh
//! $ project new ../relative/path/to/template my-project
//!
//! ProjectInfo: ProjectInfo { name: "my-project", path: "/Users/user/project/my-project" }
//! TemplateOptions: Local("/Users/user/hbs-template")
//!
//! Done generating template into /Users/user/project/my-project
//! Success!
//! Project name: my-project
//! Project path: /Users/user/project/my-project
//! Verbose: false | quite: false
//! ```
//!
//! ## Templates
//!
//! For more control of the generated project, you can create a `"template.toml"` file to configure how files are generated, variables that can
//! be substituted, directories and files to include/exclude, and many more. Your template configuration file can also contain placeholders
//! which are provided out-of-the-box. Currently supported palceholders are:
//!
//! - `{{project-name}}` - This is supplied by either passing the `name` argument to the CLI or automatically inferred from the base project
//!   directory.
//!
//! - `{{author-name}}` - Author's name is deteremed from your `cargo` or `git` configuration or a fallback to environment variables. You can also manually set the `$NAME` or `$USERNAME` environment variable.
//!
//! - `{{author-email}}` - Author's email, like `author-name`, it's gotten from your `cargo` or `git` configuration and a fallback to environment variables. You can also manually set `$EMAIL` environment variable.
//!
//!
//! ### Example
//!
//! ```rust, no_run
//! use project::{ProjectInfo, TemplateOptions, Template};
//!
//! # #[clippy::needless_doctest_main]
//! fn main() {
//!   let project = ProjectInfo::from("./my-project");
//!   let options = TemplateOptions::new("victor-iyi/project", None);
//!
//!   let template = Template::new(&project, &options);
//!   match &template.generate() {
//!     Ok(_) => {
//!       println!("Finished!");
//!       println!("\tcd {}", &project.rel_path().display());
//!     }
//!     Err(err) => eprintln!("Error generating project. {}", err),
//!   }
//!
//!   # std::fs::remove_dir_all(&project.path()).unwrap();
//! }
//! ```
//!
//! A simple example of the `"template.toml"` configuration file.
//!
//! ```toml
//! # Available built-in placholder variables are project-name, author-name, author-email.
//!
//! [variables]
//! project = "{{project-name}}"
//! author = "{{author-name}}"
//! author_email = "{{author-email}}"
//! description = "A template project"
//! py_version = "3.7"
//!
//! # Replace these directory with the value.
//! # e.g  path/to/template/file is renamed to path/to/my_project/file
//! [rename]
//! template = "{{project-name}}"
//! bin = "scripts"
//!
//! # Files or directories present in the templates can also be filtered out of the target project.
//! [filters]
//! exclude = ["venv", ".vscode", ".DS_Store"]
//! ```
//!
//! > **NOTE**: *Every files that ends with either `".hbs"` or `".liquid"` is rendered. Since it is a templating engine, logics, conditionals,*
//! > *loops are also evaluated and rendered. After the render, the files are saved without the template extensions. E.g. `setup.cfg.hbs` is*
//! > *rendered and saved as `setup.cfg`.*
//! >
//! > **WARNING**: *For files without extensions, the template extensions are still required to treat it as a candidate*
//! > *for template rendering, otherwise the files are just copied over as-is into the target project.*
//!
//! Additionally, all `filters` and `tags` of the [`Handlebars`] and [`Liquid`] templating language are
//! supported.
//! For more information, checkout the [`Handlebars`] and [`Liquid`] documentation on `Tags` and `Filters`.
//!
//! More [handlebars helpers] are supported in [`handlebars.rs`] and you can also add more helpers to the default ones already provided.
//! See [`handlebars.rs`] and [handlebars helpers docs] for more information.
//!
//! [`Handlebars`]: https://handlebarsjs.com
//! [`Liquid`]: https://shopify.github.io/liquid/
//! [`handlebars.rs`]: ./src/template/engine/handlebars.rs
//! [handlebars helpers docs]: https://docs.rs/handlebars/3.5.2/handlebars/struct.Handlebars.html#method.register_helper

mod authors;
mod cli;
mod emoji;
mod error;
pub mod git;
mod info;
mod template;
pub mod util;

// Exported public API.
pub use self::cli::{Arguments, Cli};
pub use self::error::{Error, ErrorKind, Result};
pub use self::info::{ProjectInfo, TemplateOptions};
pub use self::template::Template;
