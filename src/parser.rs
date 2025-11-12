use crate::models::{ChangeType, Changelog, VersionEntry};
use chrono::NaiveDate;
use pest::Parser;
use pest::iterators::Pair;
use std::collections::BTreeMap;
use std::str::FromStr;

#[derive(pest_derive::Parser)]
#[grammar = "changelog.pest"]
pub struct ChangelogParser;

pub fn parse_changelog(input: &str) -> Result<Changelog, pest::error::Error<Rule>> {
    let pairs = ChangelogParser::parse(Rule::file, input)?;

    let mut changelog = Changelog {
        preamble: String::new(),
        versions: Vec::new(),
        links: BTreeMap::new(),
    };

    if let Some(file_pair) = pairs.peek() {
        for pair in file_pair.into_inner() {
            match pair.as_rule() {
                Rule::top_level_heading => {
                    changelog.preamble.push_str(pair.as_str());
                }
                Rule::preamble => {
                    changelog.preamble.push_str(pair.as_str());
                }
                Rule::versions => {
                    for version_pair in pair.into_inner() {
                        changelog.versions.push(build_version_entry(version_pair));
                    }
                }
                Rule::links => {
                    for link_pair in pair.into_inner() {
                        let (version, url) = build_link_definition(link_pair);
                        changelog.links.insert(version, url);
                    }
                }
                Rule::EOI => {}
                _ => {
                    unreachable!("Unexpected top-level rule: {:?}", pair.as_rule())
                }
            }
        }
    }

    Ok(changelog)
}

fn build_version_entry(pair: Pair<Rule>) -> VersionEntry {
    let mut entry = VersionEntry {
        version: String::new(),
        date: None,
        preamble: None,
        yanked: false,
        changes: BTreeMap::new(),
    };

    let mut preamble_lines = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::version_header => {
                for header_part in inner.into_inner() {
                    match header_part.as_rule() {
                        Rule::version => {
                            entry.version = header_part.as_str().to_string();
                        }
                        Rule::date => {
                            let date_str = header_part.as_str();
                            entry.date =
                                Some(NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap());
                        }
                        Rule::yanked => {
                            entry.yanked = true;
                        }
                        _ => unreachable!("Unexpected header part: {:?}", header_part.as_rule()),
                    }
                }
            }
            Rule::version_preamble => {
                preamble_lines.push(inner.as_str());
            }
            Rule::change_sections => {
                for section_pair in inner.into_inner() {
                    let (change_type, items) = build_change_section(section_pair);
                    entry.changes.insert(change_type, items);
                }
            }
            _ => unreachable!("Unexpected version entry part: {:?}", inner.as_rule()),
        }
    }

    if !preamble_lines.is_empty() {
        entry.preamble = Some(preamble_lines.join(""));
    }

    entry
}

fn build_change_section(pair: Pair<Rule>) -> (ChangeType, Vec<String>) {
    let mut change_type = None;
    let mut items = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::change_header => {
                for header_part in inner.into_inner() {
                    if let Rule::change_type = header_part.as_rule() {
                        change_type = Some(ChangeType::from_str(header_part.as_str()).unwrap());
                    }
                }
            }
            Rule::change_items => {
                for item in inner.into_inner() {
                    items.push(item.as_str().trim().to_string());
                }
            }
            _ => unreachable!("Unexpected change section part: {:?}", inner.as_rule()),
        }
    }

    (change_type.unwrap(), items)
}

fn build_link_definition(pair: Pair<Rule>) -> (String, String) {
    let mut version = String::new();
    let mut url = String::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::link_version => {
                version = inner.into_inner().next().unwrap().as_str().to_string();
            }
            Rule::link_url => {
                url = inner.as_str().to_string();
            }
            _ => unreachable!("Unexpected link definition part: {:?}", inner.as_rule()),
        }
    }

    (version, url)
}
