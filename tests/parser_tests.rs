use clog::models::ChangeType;
use clog::parser::parse_changelog;

#[test]
fn test_parse_simple_changelog() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01

### Added

- New feature A
- New feature B

### Fixed

- Bug fix 1
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok(), "Failed to parse: {:?}", result.err());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions.len(), 1);

    let version = &changelog.versions[0];
    assert_eq!(version.version, "1.0.0");
    assert_eq!(version.date.unwrap().to_string(), "2023-01-01");
    assert_eq!(version.yanked, false);

    assert!(version.changes.contains_key(&ChangeType::Added));
    assert!(version.changes.contains_key(&ChangeType::Fixed));

    let added = version.changes.get(&ChangeType::Added).unwrap();
    assert_eq!(added.len(), 2);
    assert!(added[0].contains("New feature A"));
    assert!(added[1].contains("New feature B"));

    let fixed = version.changes.get(&ChangeType::Fixed).unwrap();
    assert_eq!(fixed.len(), 1);
    assert!(fixed[0].contains("Bug fix 1"));
}

#[test]
fn test_parse_unreleased_section() {
    let input = r#"# Changelog

## [Unreleased]

### Added

- Upcoming feature
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions.len(), 1);
    assert_eq!(changelog.versions[0].version, "Unreleased");
    assert_eq!(changelog.versions[0].date, None);
}

#[test]
fn test_parse_yanked_version() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01 [YANKED]

### Added

- Feature that was yanked
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions.len(), 1);
    assert_eq!(changelog.versions[0].yanked, true);
}

#[test]
fn test_parse_multiple_versions() {
    let input = r#"# Changelog

## [2.0.0] - 2023-02-01

### Added

- Feature in 2.0.0

## [1.0.0] - 2023-01-01

### Added

- Feature in 1.0.0
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions.len(), 2);
    assert_eq!(changelog.versions[0].version, "2.0.0");
    assert_eq!(changelog.versions[1].version, "1.0.0");
}

#[test]
fn test_parse_all_change_types() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01

### Added

- New feature

### Changed

- Updated feature

### Deprecated

- Old feature

### Removed

- Deleted feature

### Fixed

- Bug fix

### Security

- Security patch
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    let version = &changelog.versions[0];

    assert!(version.changes.contains_key(&ChangeType::Added));
    assert!(version.changes.contains_key(&ChangeType::Changed));
    assert!(version.changes.contains_key(&ChangeType::Deprecated));
    assert!(version.changes.contains_key(&ChangeType::Removed));
    assert!(version.changes.contains_key(&ChangeType::Fixed));
    assert!(version.changes.contains_key(&ChangeType::Security));
}

#[test]
fn test_parse_with_preamble() {
    let input = r#"# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog.

## [1.0.0] - 2023-01-01

### Added

- Feature
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert!(changelog.preamble.contains("notable changes"));
    assert!(changelog.preamble.contains("Keep a Changelog"));
}

#[test]
fn test_parse_with_links() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01

### Added

- Feature

[1.0.0]: https://github.com/example/repo/releases/tag/v1.0.0
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.links.len(), 1);
    assert!(changelog.links.contains_key("1.0.0"));
    assert_eq!(
        changelog.links.get("1.0.0").unwrap(),
        "https://github.com/example/repo/releases/tag/v1.0.0"
    );
}

#[test]
fn test_parse_multiline_list_item() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01

### Added

- This is a long feature description
  that spans multiple lines
  with proper indentation
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    let added = changelog.versions[0]
        .changes
        .get(&ChangeType::Added)
        .unwrap();
    assert!(added[0].contains("multiple lines"));
}

#[test]
fn test_parse_version_without_date() {
    let input = r#"# Changelog

## [1.0.0]

### Added

- Feature
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions[0].date, None);
}

#[test]
fn test_parse_empty_sections_between_changes() {
    let input = r#"# Changelog

## [1.0.0] - 2023-01-01

### Added

- Feature A

### Changed

- Changed B
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions[0].changes.len(), 2);
}

#[test]
fn test_parse_semver_with_prerelease() {
    let input = r#"# Changelog

## [1.0.0-beta.1] - 2023-01-01

### Added

- Beta feature
"#;

    let result = parse_changelog(input);
    assert!(result.is_ok());

    let changelog = result.unwrap();
    assert_eq!(changelog.versions[0].version, "1.0.0-beta.1");
}
