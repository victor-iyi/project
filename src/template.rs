use crate::{
  cli::{Arguments, Cli},
  info::TemplateMeta,
};

pub(crate) mod config;
#[cfg(feature = "git")]
#[cfg(feature = "hbs")]
pub mod git;
#[cfg(feature = "hbs")]
pub mod guidon;
#[cfg(feature = "hbs")]
pub(crate) mod helpers;
pub(crate) mod parser;

/// Template builds and generates the project from a given template.
pub struct Template {
  template: TemplateMeta,
}

impl From<Arguments> for Template {
  fn from(args: Arguments) -> Template {
    Template {
      template: TemplateMeta::new(&args.project, &args.template),
    }
  }
}

impl From<Cli<'_>> for Template {
  fn from(cli: Cli) -> Template {
    Self::from(cli.args)
  }
}

impl Template {
  fn new(template_info: TemplateMeta) -> Template {
    Template {
      template: template_info,
    }
  }
}

impl Default for Template {
  fn default() -> Template {
    Template {
      template: TemplateMeta::default(),
    }
  }
}

// impl Template {
//   pub fn generate(&self, dest: &dyn AsRef<Path>) -> Result<()> {
//     // Target destination where template will be created.
//     let target: PathBuf = dest.as_ref().to_owned();

//     // Create destination folders.
//     std::fs::create_dir_all(dest.as_ref())?;

//     // Walk the path and copy src path over to dest path.
//     for entry in WalkDir::new(&self.path).into_iter().filter_map(|e| e.ok()) {
//       if entry.path().is_dir() {
//         std::fs::create_dir_all(entry.path())?;
//         continue;
//       } else if entry.path().is_file() {
//         // Open the file.
//         std::fs::copy(entry.path(), &target)?;
//       } else {
//         eprintln!("Do not know what's happening here...");
//       }
//       println!("{}", &entry.path().display());
//     }
//     Ok(())
//   }
// }
