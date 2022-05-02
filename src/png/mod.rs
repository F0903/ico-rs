mod chunk;
mod encoded_png;
mod png_ico_writer;
pub(crate) mod png_meta;
mod png_parser;

pub use png_ico_writer::write_ico;
pub use png_parser::PngParser;

pub type Result<T> = std::result::Result<T, &'static str>;
