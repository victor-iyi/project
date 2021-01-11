use handlebars::{Handlebars, HelperDef};

use crate::template::helpers;

/// Helper function
///
/// Note:
/// - `&Helper`: current helper template information, contains name, params, hashes and nested template
/// - `&Registry`: the global registry, you can find templates by name from registry
/// - `&Context`: the whole data to render, in most case you can use data from `Helper`
/// - `&mut RenderContext`: you can access data or modify variables (starts with @)/partials in render context, for example, @index of #each. See its document for detail.
/// - `&mut dyn Output`: where you write output to
///
/// # Example
///
/// The following creates an upper case function helper.
///
/// ```ignore
///
/// use handlebars::{
///   Context, Handlebars, Helper, HelperResult, Output, RenderContext,
/// };
///
/// pub fn upper(
///   h: &Helper<'_, '_>,
///   _: &Handlebars<'_>,
///   _: &Context,
///   _rc: &mut RenderContext<'_, '_>,
///   out: &mut dyn Output,
/// ) -> HelperResult {
///   // get parameter from helper or throw an error
///   let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
///   out.write(param.to_uppercase().as_ref())?;
///   Ok(())
/// }
/// ```
///
/// You can add it to template like so:
///
/// ```
/// use lotlinx::Template;
///
/// let t = Template::new("path/to/src", "path/to/dest");
/// t.re
/// ```
pub(crate) type HelperFn = dyn HelperDef + Send + Sync;

/// Register builtin default Handlebar helpers.
#[inline]
pub(crate) fn register_default_helpers(handlebars: &mut Handlebars) {
  // Register handlebars helpers.
  register_helper_fn(handlebars, "replace", Box::new(helpers::replace));
  register_helper_fn(handlebars, "append", Box::new(helpers::append));
  register_helper_fn(handlebars, "prepend", Box::new(helpers::prepend));
  register_helper_fn(handlebars, "up", Box::new(helpers::up));
  register_helper_fn(handlebars, "low", Box::new(helpers::low));
}

/// Register a new handlebar helper function.
#[inline]
pub(crate) fn register_helper_fn(
  hbs: &mut Handlebars,
  name: &str,
  helper_fn: Box<HelperFn>,
) {
  // Register handlebars helpers.
  hbs.register_helper(name, helper_fn);
}
