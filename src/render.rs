use crate::{error::Result, template::Template};

use std::path::Path;

pub trait Renderer {
  fn render(writer: dyn std::io::Write) -> Result<()>;

  fn generate(template: Template, target_path: &dyn AsRef<Path>) -> Result<()> {
    Ok(())
  }
}
