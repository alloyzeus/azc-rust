//

#[derive(Clone, Debug)]
pub struct Expiration {
    // Master flag
    pub enabled: bool,

    // A flag to allow override the config at runtime.
    pub runtime_overrideable: bool,
    //TODO: duration
}

impl Default for Expiration {
    fn default() -> Self {
        Self {
            enabled: false,
            runtime_overrideable: false,
        }
    }
}
