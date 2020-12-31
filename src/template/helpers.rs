use handlebars::{
  Context, Handlebars, Helper, HelperResult, Output, RenderContext,
};

/// Handlebars helper to replace a string by another in the vars.
///
/// The helper takes two parameters:
/// * from: the string to replace
/// * to: the replacement string
///
/// ```properties
/// {{replace input "Roger" "Brian"}}
/// ```
/// Every occurence of "Roger" in *input* will be replaced by "Brian"
pub fn replace(
  h: &Helper<'_, '_>,
  _: &Handlebars<'_>,
  _: &Context,
  _rc: &mut RenderContext<'_, '_>,
  out: &mut dyn Output,
) -> HelperResult {
  // get parameter from helper or throw an error
  let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
  let from = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
  let to = h.param(2).and_then(|v| v.value().as_str()).unwrap_or("");
  out.write(param.replace(from, to).as_ref())?;
  Ok(())
}

/// Handlebars helpers for to uppercase the input
///
/// The helper doesn't take any argument :
/// ```properties
/// {{up param}}
/// ```
pub fn up(
  h: &Helper<'_, '_>,
  _: &Handlebars<'_>,
  _: &Context,
  _rc: &mut RenderContext<'_, '_>,
  out: &mut dyn Output,
) -> HelperResult {
  // get parameter from helper or throw an error
  let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
  out.write(param.to_uppercase().as_ref())?;
  Ok(())
}

/// Handlebars helpers for to lowercase the input
///
/// The helper doesn't take any argument :
/// ```properties
/// {{low input}}
/// ```
pub fn low(
  h: &Helper<'_, '_>,
  _: &Handlebars<'_>,
  _: &Context,
  _rc: &mut RenderContext<'_, '_>,
  out: &mut dyn Output,
) -> HelperResult {
  // get parameter from helper or throw an error
  let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
  out.write(param.to_lowercase().as_ref())?;
  Ok(())
}

/// Handlebars helper to apppend a string by another in the vars.
///
/// The helper takes one parameter:
/// * to_append: the string to append to *input*
///
/// ```properties
/// {{append input "-suffix" }}
/// ```
pub fn append(
  h: &Helper<'_, '_>,
  _: &Handlebars<'_>,
  _: &Context,
  _rc: &mut RenderContext<'_, '_>,
  out: &mut dyn Output,
) -> HelperResult {
  // get parameter from helper or throw an error
  let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
  let to_append = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
  out.write(format!("{}{}", param, to_append).as_ref())?;
  Ok(())
}

/// Handlebars helper to prepend a string to the input.
///
/// The helper takes one parameters:
/// * to_prepend: the string to prepend
///
/// ```properties
/// {{prepend input "prefix-"}}
/// ```
pub fn prepend(
  h: &Helper<'_, '_>,
  _: &Handlebars<'_>,
  _: &Context,
  _rc: &mut RenderContext<'_, '_>,
  out: &mut dyn Output,
) -> HelperResult {
  // get parameter from helper or throw an error
  let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
  let to_prepend = h.param(1).and_then(|v| v.value().as_str()).unwrap_or("");
  out.write(format!("{}{}", to_prepend, param).as_ref())?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  use handlebars::Handlebars;
  use log::LevelFilter;
  use pretty_assertions::assert_eq;

  use std::collections::BTreeMap;
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
  fn should_replace() {
    setup();
    let mut vars = BTreeMap::new();
    vars.insert("teacher", "Repeat after me");
    vars.insert("sentence", "Roger is in the kitchen");

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("replace", Box::new(replace));
    let res = handlebars
      .render_template(
        "{{teacher}}: {{replace sentence \"Roger\" \"Brian\"}}.",
        &vars,
      )
      .unwrap();
    println!("{}", res);
    assert_eq!(res, "Repeat after me: Brian is in the kitchen.");
  }

  #[test]
  fn should_uppercase() {
    setup();
    let mut vars = BTreeMap::new();
    vars.insert("teacher", "Repeat after me");
    vars.insert("sentence", "Brian is in the kitchen");

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("up", Box::new(up));
    let res = handlebars
      .render_template("{{teacher}}: {{up sentence}}.", &vars)
      .unwrap();
    println!("{}", res);
    assert_eq!(res, "Repeat after me: BRIAN IS IN THE KITCHEN.");
  }

  #[test]
  fn should_lowercase() {
    setup();
    let mut vars = BTreeMap::new();
    vars.insert("teacher", "Repeat after me");
    vars.insert("sentence", "Brian IS IN THE KITCHEN");

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("low", Box::new(low));
    let res = handlebars
      .render_template("{{teacher}}: {{low sentence}}.", &vars)
      .unwrap();
    println!("{}", res);
    assert_eq!(res, "Repeat after me: brian is in the kitchen.");
  }

  #[test]
  fn should_append() {
    setup();
    let mut vars = BTreeMap::new();
    vars.insert("teacher", "Repeat after me");
    vars.insert("sentence", "Brian");

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("append", Box::new(append));
    let res = handlebars
      .render_template(
        r#"{{teacher}}: {{append sentence " is in the kitchen"}}."#,
        &vars,
      )
      .unwrap();
    println!("{}", res);
    assert_eq!(res, "Repeat after me: Brian is in the kitchen.");
  }

  #[test]
  fn should_prepend() {
    setup();
    let mut vars = BTreeMap::new();
    vars.insert("teacher", "Repeat after me");
    vars.insert("sentence", "is in the kitchen");

    let mut handlebars = Handlebars::new();
    handlebars.register_helper("prepend", Box::new(prepend));
    let res = handlebars
      .render_template(r#"{{teacher}}: {{prepend sentence "Brian "}}."#, &vars)
      .unwrap();
    println!("{}", res);
    assert_eq!(res, "Repeat after me: Brian is in the kitchen.");
  }
}
