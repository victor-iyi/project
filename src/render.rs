use crate::error::Result;

pub trait Renderer {
  fn render(writer: dyn std::io::Write) -> Result<()>;
}
