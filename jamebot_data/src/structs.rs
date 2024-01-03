use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct GuildConfig {
    pub prefix: Option<String>,
    pub modules: ModuleConfig,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ModuleConfig {
    pub mod_config: Option<Moderation>,
    pub utility_config: Option<Utility>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Moderation {
    pub enabled: bool,
    pub guild_update_settings: Option<GUpdateSettings>,
    pub new_member_warning: Option<NewMemberWarnings>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Utility {
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct GUpdateSettings {
    pub enabled: bool,
    // banner, image, name
    pub basic: bool,
    pub roles: RoleSettings,
    pub expressions: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct RoleSettings {
    pub enabled: bool,
    pub verbose: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct NewMemberWarnings {
    pub enabled: bool,
    pub threshold: std::time::Duration,
}

impl ModuleConfig {
    pub fn new() -> Self {
        ModuleConfig {
            mod_config: None,
            utility_config: None,
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
