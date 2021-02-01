use crate::{authors, error::Result};

use regex::Regex;
use std::{fs, io::Read, path::Path};

/// Default variables substitution in `template.toml`.
///
/// - `{{ project-name }}` - Project name.
///
/// - `{{ author-name }}` - Author's name, gotten from Git config.
///
/// - `{{ author-email }}` - Author's email address, gotten from Git config.
fn default_variables(
  haystack: &str,
  project_name: &str,
  author_name: &str,
  author_email: &str,
) -> Result<String> {
  // Project name.
  let result =
    Regex::new(r"\{\{\s?project-name\s?\}\}")?.replace_all(haystack, project_name);

  // Author name.
  let result =
    Regex::new(r"\{\{\s?author-name\s?\}\}")?.replace_all(&result, author_name);

  // Author email.
  let result = Regex::new(r"\{\{\s?author-email\s?\}\}")?
    .replace_all(&result, author_email);

  Ok(result.to_string())
}

/// Replacement of [`default_variables`] in a given template file.
///
/// # Example
///
/// ```rust, ignore
/// let parsed = parse_template_file("path/to/template.toml", "my_project");
///
/// match parsed {
///   Ok(template) => println!("{}", template),
///   Err(err) => eprintln!("{}", err),
/// }
/// ```
/// [`default_variables`]: fn.default_variables
pub(super) fn parse_template_file(
  template_file: &Path,
  project_name: &str,
) -> Result<String> {
  // Open template file.
  let mut file = fs::File::open(template_file)?;

  // Read templat file to a string.
  let mut template_string = String::new();
  file.read_to_string(&mut template_string)?;

  // Get author's name & email from env.
  let (author_name, author_email) =
    authors::discover_author().unwrap_or_default();

  // Perform replacement.
  default_variables(
    &template_string,
    project_name,
    &author_name,
    &author_email.unwrap_or_default(),
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_variables() {
    let template_str = r#"
[variables]
name = "{{ project-name }}"
email = "[{{author-name} <{{author-email}}>]"

[directories]
template = "{{project-name}}"
  "#;

    let expected_str = r#"
[variables]
name = "lotlinx"
email = "[Victor I. Afolabi <vafolabi@lotlinx.com>]"

[directories]
template = "lotlinx"
  "#;

    let res = default_variables(
      template_str,
      "lotlinx",
      "Victor I. Afolabi",
      "vafolabi@lotlinx.com",
    );
    assert!(res.is_ok());

    if let Ok(expected) = res {
      assert_eq!(expected_str, &expected);
    }
  }
}
