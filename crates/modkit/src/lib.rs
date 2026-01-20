pub mod hashing;
pub mod manifest;
pub mod schema;

pub const DEFAULT_RON_EXTENSIONS: ron::extensions::Extensions =
    ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES
        | ron::extensions::Extensions::EXPLICIT_STRUCT_NAMES;

pub fn default_ron_options() -> ron::options::Options {
    ron::options::Options::default().with_default_extension(DEFAULT_RON_EXTENSIONS)
}
