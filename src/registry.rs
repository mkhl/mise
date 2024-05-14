use once_cell::sync::Lazy;
use std::collections::BTreeMap;

pub static REGISTRY: Lazy<BTreeMap<&'static str, &'static str>> =
    Lazy::new(|| BTreeMap::from([("ubi", "cargo:ubi")]));

pub fn get(s: &str) -> Option<&str> {
    REGISTRY.get(s).copied()
}
