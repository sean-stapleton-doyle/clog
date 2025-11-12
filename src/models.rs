use std::collections::BTreeMap;

use chrono::NaiveDate;

#[derive(Debug)]
pub struct Changelog {
    pub preamble: String,
    pub versions: Vec<VersionEntry>,
    pub links: BTreeMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub struct VersionEntry {
    pub version: String,
    pub date: Option<NaiveDate>,
    pub preamble: Option<String>,
    pub yanked: bool,
    pub changes: BTreeMap<ChangeType, Vec<String>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum ChangeType {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

impl std::str::FromStr for ChangeType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "added" => Ok(ChangeType::Added),
            "changed" => Ok(ChangeType::Changed),
            "deprecated" => Ok(ChangeType::Deprecated),
            "removed" => Ok(ChangeType::Removed),
            "fixed" => Ok(ChangeType::Fixed),
            "security" => Ok(ChangeType::Security),
            _ => Err("Unknown change type"),
        }
    }
}
