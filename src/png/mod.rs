mod chunk;
mod encoded_png;
mod png_data;
mod png_ico_writer;
pub(crate) mod png_meta;
mod png_parser;

pub use png_data::PngData;
pub use png_ico_writer::{convert, convert_file};
pub use png_parser::PngParser;

pub type Result<T> = std::result::Result<T, &'static str>;
