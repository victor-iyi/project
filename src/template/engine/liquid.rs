use crate::error::Result;

use serde::Serialize;

pub(crate) fn parse<T: Serialize>(
  content: &str,
  variables: &T,
) -> Result<String> {
  let template = liquid::ParserBuilder::with_stdlib()
    .build()
    .unwrap()
    .parse(content)?;

  // Convert variables to Liquid Object.
  let globals = liquid::model::to_object(variables)?;

  // Render template.
  let output = template.render(&globals)?;

  Ok(output)
}
