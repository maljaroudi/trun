use serde::Deserialize;

/// Opts is used to add any optional instructions that may be shared between 
/// modules such as the ability to silence errors. 
#[derive(Deserialize, Default)]
pub struct Opts {
    pub panics: Option<bool>,
    pub debug: Option<bool>,
}



