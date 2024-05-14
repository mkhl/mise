use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::path::PathBuf;

use crate::forge::unalias_forge;
use crate::forge::ForgeType;
use crate::{dirs, registry};

#[derive(Clone, PartialOrd, Ord)]
pub struct ForgeArg {
    pub id: String,
    pub name: String,
    pub forge_type: ForgeType,
    pub input: String,
    pub cache_path: PathBuf,
    pub installs_path: PathBuf,
    pub downloads_path: PathBuf,
}

impl From<&str> for ForgeArg {
    fn from(input: &str) -> Self {
        let s = registry::get(input).unwrap_or(input);
        if let Some((forge_type, name)) = s.split_once(':') {
            if let Ok(forge_type) = forge_type.parse() {
                return Self::new(forge_type, name).with_input(input);
            }
        }
        Self::new(ForgeType::Asdf, s).with_input(input)
    }
}
impl From<&String> for ForgeArg {
    fn from(s: &String) -> Self {
        Self::from(s.as_str())
    }
}

impl ForgeArg {
    pub fn new(forge_type: ForgeType, name: &str) -> Self {
        let name = unalias_forge(name).to_string();
        let id = match forge_type {
            ForgeType::Asdf => name.clone(),
            forge_type => format!("{}:{}", forge_type.as_ref(), name),
        };
        let pathname = regex!(r#"[/:]"#).replace_all(&id, "-").to_string();
        Self {
            input: name.clone(),
            name,
            forge_type,
            id,
            cache_path: dirs::CACHE.join(&pathname),
            installs_path: dirs::INSTALLS.join(&pathname),
            downloads_path: dirs::DOWNLOADS.join(&pathname),
        }
    }

    fn with_input(mut self, input: &str) -> Self {
        self.input = input.to_string();
        self
    }
}

impl Display for ForgeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: use self.input when we're sure that won't break anything
        write!(f, "{}", self.id)
    }
}

impl Debug for ForgeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"ForgeArg("{}")"#, self.id)
    }
}

impl PartialEq for ForgeArg {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ForgeArg {}

impl Hash for ForgeArg {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forge_arg() {
        let t = |s: &str, id, name, t| {
            let fa: ForgeArg = s.into();
            assert_str_eq!(fa.id, id);
            assert_str_eq!(fa.name, name);
            assert_eq!(fa.forge_type, t);
        };
        let asdf = |s, id, name| t(s, id, name, ForgeType::Asdf);
        let cargo = |s, id, name| t(s, id, name, ForgeType::Cargo);
        let npm = |s, id, name| t(s, id, name, ForgeType::Npm);

        asdf("asdf:node", "node", "node");
        asdf("node", "node", "node");
        asdf("", "", "");
        cargo("cargo:eza", "cargo:eza", "eza");
        npm("npm:@antfu/ni", "npm:@antfu/ni", "@antfu/ni");
        npm("npm:prettier", "npm:prettier", "prettier");
    }

    #[test]
    fn test_forge_arg_pathname() {
        let t = |s: &str, expected| {
            let fa: ForgeArg = s.into();
            let actual = fa.installs_path.to_string_lossy();
            let expected = dirs::INSTALLS.join(expected);
            assert_str_eq!(actual, expected.to_string_lossy());
        };
        t("asdf:node", "node");
        t("node", "node");
        t("", "");
        t("cargo:eza", "cargo-eza");
        t("npm:@antfu/ni", "npm-@antfu-ni");
        t("npm:prettier", "npm-prettier");
    }
}
