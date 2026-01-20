pub mod hashing;
pub mod manifest;
pub mod schema;

pub fn default_ron_options() -> ron::options::Options {
    ron::options::Options::default().with_default_extension(
        ron::extensions::Extensions::UNWRAP_VARIANT_NEWTYPES
            | ron::extensions::Extensions::IMPLICIT_SOME,
    )
}
