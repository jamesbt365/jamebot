use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GuildConfig {
    pub prefix: Option<String>,
    pub modules: ModuleConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModuleConfig {
    pub mod_config: Moderation,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Moderation {
    pub enabled: bool,
    pub elevation_settings: ElevationSettings
}


// TODO:
// Allow higher settings - if depending on user (role? (provided disallow of the role that allows it)), self protect?
// elevation limit (what position is max)
// max elevation time
// cooldown per executer, executee
// allow multiple applications at once?
// Safeguarding settings to prevent infinite execution
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ElevationSettings {
    pub enabled: bool,
}


impl ModuleConfig {
    pub fn new() -> Self {
        ModuleConfig {
            mod_config: Moderation {
                enabled: false,
                elevation_settings: ElevationSettings {
                    enabled: false,
                }
            },
        }
    }
}

impl GuildConfig {
    pub fn new(prefix: Option<String>) -> Self {
        GuildConfig {
            prefix: prefix.or(Some("-".to_string())),
            modules: ModuleConfig::new(),
        }
    }

    #[inline]
    pub fn prefix(mut self, prefix: Option<String>) -> Self {
        self.prefix = prefix;
        self
    }
}

impl Default for GuildConfig {
    fn default() -> Self {
        Self::new(Some("-".to_string()))
    }
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self::new()
    }
}
