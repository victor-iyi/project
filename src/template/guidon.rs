use crate::error::{Error, ErrorKind, Result};
use crate::template::helpers;

use handlebars::Handlebars;
use log::{debug, error, info};
use serde::Deserialize;

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{copy, create_dir_all, read_to_string, remove_file, File};
use std::marker::Sized;
use std::path::{Path, PathBuf};

pub type VariablesCallback<'a> = dyn Fn(&mut BTreeMap<String, String>) + 'a;
pub type RenderCallback<'a> = dyn Fn(String) -> String + 'a;

static TEMPLATE_FILE: &str = "template.toml";

#[derive(Deserialize)]
pub struct Template {
  pub variables: BTreeMap<String, String>,
}

/// Try to initialize Guidon from different sources
pub trait TryNew<A> {
  /// Try to initialize from an object A
  fn try_new(src: A) -> Result<Self>
  where
    Self: Sized;
}

/// The Guidon structure
#[derive(Default)]
pub struct Guidon<'a> {
  pub(crate) path: PathBuf,
  no_template_dir: bool,
  use_lax_mode: bool,
  pub(crate) variables: BTreeMap<String, String>,
  variables_callback: Option<Box<VariablesCallback<'a>>>,
  render_callback: Option<Box<RenderCallback<'a>>>,
}

impl<'a, P: 'a + AsRef<Path>> TryNew<P> for Guidon<'a> {
  /// Initialization from a folder path or a file path
  /// * dir_path : the base directory for guidon-cli template. The file `template.toml` is
  /// searched in this directory.
  /// * file_path : the path of the template file. The working dir will be the file parent directory.
  fn try_new(path: P) -> Result<Self> {
    // Path is a dir. Let find template.toml in it
    let g = if path.as_ref().is_dir() {
      let tplt_file_path = path.as_ref().join(TEMPLATE_FILE);

      let tplt_str = read_to_string(&tplt_file_path).unwrap_or_else(|_| {
        panic!("No template found at {}", tplt_file_path.to_string_lossy())
      });
      let tplt: Template = toml::from_str(&tplt_str)?;
      let mut guidon = Guidon::default();
      guidon.path = path.as_ref().to_path_buf();
      guidon.variables = tplt.variables;
      guidon
    } else {
      // Path point to the variable file. Working dir is assumed to be the parent dir
      let tplt_str = read_to_string(path.as_ref())?;
      let tplt: Template = toml::from_str(&tplt_str)?;
      let mut guidon = Guidon::default();
      guidon.path = path
        .as_ref()
        .parent()
        .ok_or_else(|| {
          Error::new(
            ErrorKind::Io,
            &format!("Path not found: {}", path.as_ref().display()),
          )
        })?
        .to_path_buf();
      guidon.variables = tplt.variables;
      guidon
    };
    Ok(g)
  }
}

impl<'a> Guidon<'a> {
  /// Creates a new Guidon from a path
  pub fn new(path: PathBuf) -> Self {
    let mut g = Guidon::default();
    g.path = path;
    g
  }

  /// Sets the substitutions variables
  pub fn variables(&mut self, vars: BTreeMap<String, String>) {
    self.variables = vars;
  }

  /// If set to `true` guidon-cli will try to parse a file structure in a folder called `template`
  /// located in the given input path.
  /// If set to `false` guidon-cli will parse the given input folder.
  ///
  /// By default guidon-cli will use a template dir.
  pub fn use_template_dir(&mut self, tplt: bool) -> &mut Self {
    self.no_template_dir = !tplt;
    self
  }

  /// Wether to use or no Handlebars strict mode.
  ///
  /// If set to `true` (default value), an error will be raised if a variable is not defined.
  /// If set to `false` missing variables will be set to an empty string.
  pub fn use_strict_mode(&mut self, strict: bool) -> &mut Self {
    self.use_lax_mode = !strict;
    self
  }

  /// Provides a callback to perform an operation on the variables map.
  /// Can be used to change default variables values.
  ///
  /// # Arguments
  /// * `cb`: callback. A closure with takes a `BTreeMap<String, String>` as parameter and returns
  /// a `BTreeMap<String, String>`.
  ///
  /// # Example
  /// ```rust, no_run
  ///   use guidon::Guidon;
  ///   use std::collections::BTreeMap;
  ///   use std::path::PathBuf;
  ///
  ///   let cb = |h: &mut BTreeMap<String, String>| {
  ///             h.iter_mut().for_each(|(_, v)|  *v +=" cb");
  ///    };
  ///   let mut guidon = Guidon::new(PathBuf::from("template/path"));
  ///   guidon.set_variables_callback(cb);
  /// ```
  pub fn set_variables_callback<F>(&mut self, cb: F)
  where
    F: Fn(&mut BTreeMap<String, String>) + 'a,
  {
    self.variables_callback = Some(Box::new(cb) as Box<VariablesCallback>);
  }

  /// Provides a callback to be called when a variables is not found in the configuration file.
  ///
  /// # Arguments
  /// * `cb`: callback. A closure with takes the expected key as a `String` parameter and returns
  /// the value to use as a `String`.
  ///
  /// # Example
  /// ```rust, no_run
  ///   use guidon::Guidon;
  ///   use std::collections::BTreeMap;
  ///   use std::path::PathBuf;
  ///
  ///   // this callback will add `-cb` to the key as a value
  ///   let cb = |h: String| {
  ///           let mut s = h.clone();
  ///           s.push_str("-cb");
  ///           s
  ///    };
  ///   let mut guidon = Guidon::new(PathBuf::from("template/path"));
  ///   guidon.set_render_callback(cb);
  /// ```
  pub fn set_render_callback<F>(&mut self, cb: F)
  where
    F: Fn(String) -> String + 'a,
  {
    self.render_callback = Some(Box::new(cb) as Box<RenderCallback>);
  }

  /// Apply template.
  /// The substitution will be performed with variables provided in the config file.
  /// The input dir is deduced from the path given at guidon-cli initialization:
  /// * `given/path/template` if `use_template_dir` is `true` (default behaviour)
  /// * `given/path` if `use_template_dir` is `false`
  ///
  /// # Arguments
  /// * `to_dir` : the directory where the templated file structure will be created.
  pub fn apply_template<T>(&mut self, to_dir: T) -> Result<()>
  where
    T: AsRef<Path>,
  {
    info!("Applying template for {}", self.path.display().to_string());

    // 1 - Check substition values (callback)
    // call callback
    if let Some(cb) = &self.variables_callback {
      debug!("Calling variable callback");
      cb(&mut self.variables);
    }

    // 2 - Create destination directory
    // Fails if the destination dir doesn't exists
    create_dir_all(to_dir.as_ref())?;

    // 3 - Parse template dirs, apply template and copy to destination
    let mut handlebars = Handlebars::new();
    handlebars.register_helper("replace", Box::new(helpers::replace));
    handlebars.register_helper("append", Box::new(helpers::append));
    handlebars.register_helper("prepend", Box::new(helpers::prepend));
    handlebars.register_helper("up", Box::new(helpers::up));
    handlebars.register_helper("low", Box::new(helpers::low));
    handlebars.set_strict_mode(!self.use_lax_mode);
    let template_dir = if !self.no_template_dir {
      self.path.join("template")
    } else {
      self.path.to_path_buf()
    };
    self.parse_dir(
      &template_dir.canonicalize()?,
      &to_dir.as_ref().canonicalize()?,
      &handlebars,
    )?;

    info!("Template applied.");
    Ok(())
  }

  // Convert a file or folder name if its templated (my-{{wonderful}}-name)
  // Current limitation: don't handle name with several substitution
  // placeholders (like my-{{very}}-{{beautiful}}-name)
  // TODO: Consider the use of regex
  fn convert_file_name(
    &self,
    entry_path: &Path,
    hb: &Handlebars,
  ) -> Result<String> {
    let name = entry_path
      .file_name()
      .and_then(OsStr::to_str)
      .ok_or_else(|| Error::new(ErrorKind::NotFound, "Can't extract dir name"))?
      .to_string();

    hb.render_template(&name, &self.variables).map_err(|e| {
      Error::new(ErrorKind::Error, &e.as_render_error().unwrap().desc)
    })
  }

  // Recursively parse the files and folder in the given directory,
  // trying to apply handlebars substitution
  // TODO Consider the use of walkdir
  fn parse_dir(
    &mut self,
    dir: &Path,
    to: &Path,
    hb: &Handlebars,
  ) -> Result<()> {
    debug!("Parsing dir {}", dir.display().to_string());
    if !dir.is_dir() {
      return Err(Error::new(ErrorKind::NotADirectory, "Not a directory"));
    }

    // for each entry…
    for entry in dir.read_dir()? {
      match entry {
        Ok(entry) => {
          // … performs name substitution if necessary…
          let entry_path = entry.path();
          debug!("Parsing entry {}", entry_path.display().to_string());

          let name = self.convert_file_name(&entry_path, hb)?;
          // … if it's a directory, creates the targe directory…
          if entry_path.is_dir() {
            let target_path = to.join(name);
            debug!("Creating folder {}", target_path.display().to_string());
            create_dir_all(&target_path)?;
            self.parse_dir(&entry_path, &target_path, hb)?;
          } else {
            // … if it's a file…
            debug!("Entry is a file");
            if name.ends_with("hbs") {
              //… call handlebars and write the processed file to target…
              debug!("Entry is a template source");
              let mut source_template = File::open(entry.path())?;
              let t = dir.join(name);
              let target_file_name = t.file_stem().unwrap();
              let target_path = to.join(target_file_name);
              debug!("Copying entry to {}", target_path.display().to_string());
              // … if a variable is not found, execute the callback…
              if let Some(cb) = &self.render_callback {
                loop {
                  if target_path.exists() {
                    remove_file(&target_path)?;
                  }
                  let mut output_file = File::create(&target_path)?;
                  match hb.render_template_source_to_write(
                    &mut source_template,
                    &self.variables,
                    &mut output_file,
                  ) {
                    Err(e) => {
                      if let Some(error) = e.as_render_error() {
                        //TODO : propose commit to handlebars to retrieve the key
                        debug!("Missing value for key : {}", error.desc);
                        let variable: &str =
                          error.desc.split('"').nth(1).unwrap();
                        let value = cb(variable.to_string());
                        self.variables.insert(variable.to_string(), value);
                        source_template = File::open(entry.path())?;
                      } else {
                        debug!("Handlebars error");
                        return Err(e.into());
                      }
                    }
                    Ok(_) => {
                      debug!(
                        "Mapping done for {}",
                        target_file_name.to_string_lossy()
                      );
                      break;
                    }
                  }
                }
              } else {
                let mut output_file = File::create(&target_path)?;
                hb.render_template_source_to_write(
                  &mut source_template,
                  &self.variables,
                  &mut output_file,
                )?;
              }
            } else {
              // If no substitution, direct copy of the file
              let target_path = to.join(name);
              debug!("Copying entry to {}", target_path.display().to_string());
              copy(&entry_path, to.join(target_path))?;
            }
          }
        }
        Err(e) => error!("Unparsable dir entry : {}", e.to_string()),
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use handlebars::to_json;
  use log::LevelFilter;
  // use pretty_assertions::assert_eq;
  use std::collections::BTreeMap;
  use std::path::PathBuf;
  use std::sync::Once;

  static INIT: Once = Once::new();

  fn setup() {
    INIT.call_once(|| {
      let _ = env_logger::builder()
        .is_test(true)
        .filter_level(LevelFilter::Trace)
        .try_init();
    });
  }

  #[test]
  fn should_parse_toml() {
    setup();
    let toml_content = r#"
          [variables]
          key1 = "value1"
          key2 = "value2"
          "#;

    let tplt: Template = toml::from_str(toml_content).unwrap();
    let mut guidon = Guidon::default();
    guidon.variables = tplt.variables;
    guidon.use_template_dir(true);
    assert_eq!(guidon.render_callback.is_none(), true);
    assert_eq!(guidon.no_template_dir, false);
    assert_eq!(guidon.variables["key1"], "value1".to_string())
  }

  #[test]
  fn test_variables_callback() {
    let cb = |h: &mut BTreeMap<String, String>| {
      h.iter_mut().for_each(|(_, v)| *v += " cb");
    };

    setup();
    let mut guidon = Guidon::default();

    guidon.set_variables_callback(cb);

    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert("toto".to_owned(), "tutu".to_string());
    map.insert("titi".to_string(), "tata".to_string());
    map.insert("riri".to_string(), "fifi".to_string());
    cb(&mut map);

    assert_eq!(map["toto"], "tutu cb".to_string());
    assert_eq!(map["titi"], "tata cb".to_string());
    assert_eq!(map["riri"], "fifi cb");
  }

  #[test]
  fn should_convert_file_name2() {
    let path = PathBuf::from("test/my-{{plop}}-file.{{ext}}.hbs");
    let mut map = BTreeMap::new();
    map.insert("plop", "beautiful");
    map.insert("ext", "txt");
    let data = to_json(&map);
    let hb = handlebars::Handlebars::new();
    let out = hb.render_template(path.to_str().unwrap(), &data).unwrap();
    assert_eq!("test/my-beautiful-file.txt.hbs", out);
  }
}
