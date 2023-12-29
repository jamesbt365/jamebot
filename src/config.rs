use serde::Deserialize;
use serenity::model::Permissions;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GuildConfig {
    pub prefix: Option<String>,
    pub modules: ModuleConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModuleConfig {
    pub mod_config: Option<Moderation>,
    pub utility_config: Option<Utility>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Moderation {
    pub enabled: bool,
    pub elevation_settings: Option<ElevationSettings>,
    pub guild_update_settings: Option<GUpdateSettings>,
    pub new_member_warning: Option<NewMemberWarnings>
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Utility {
    pub enabled: bool,
}



// TODO:
// Allow higher settings - if depending on user (role? (provided disallow of the role that allows it)), self protect?
// elevation limit (what position is max)

// allow multiple applications at once?
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ElevationSettings {
    pub enabled: bool,
    pub permission: Option<Permissions>,
    pub max_time: Option<u16>, // seconds.
    pub allow_mult: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GUpdateSettings {
    pub enabled: bool,
    // banner, image, name
    pub basic: bool,
    pub roles: RoleSettings,
    pub expressions: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct RoleSettings {
    pub enabled: bool,
    pub verbose: bool,
}


#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct NewMemberWarnings {
    pub enabled: bool,
    pub threshold: std::time::Duration,
}



impl ModuleConfig {
    pub fn new() -> Self {
        ModuleConfig {
            mod_config: None,
            utility_config: None
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
