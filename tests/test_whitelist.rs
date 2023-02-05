use std::collections::HashSet;
use std::iter::FromIterator;

use configure_semantic_release_assets::SemanticReleaseManifest;

const SEMANTIC_RELEASE_CONFIG: &'static str = r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release-cargo/semantic-release-cargo",
    [
      "semantic-release-major-tag",
      {
        "customTags": [
          "v${major}",
          "v${major}.${minor}"
        ]
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": [
          {
            "path": ".semantic-release-action_rust/dist/x86_64-unknown-linux-musl/BINARY_NAME-x86_64-unknown-linux-musl",
            "label": "x86_64-unknown-linux-musl"
          },
          {
            "path": ".semantic-release-action_rust/dist/x86_64-unknown-linux-gnu/BINARY_NAME-x86_64-unknown-linux-gnu",
            "label": "x86_64-unknown-linux-gnu"
          },
          {
            "path": ".semantic-release-action_rust/dist/i686-unknown-linux-musl/BINARY_NAME-i686-unknown-linux-musl",
            "label": "i686-unknown-linux-musl"
          },
          {
            "path": ".semantic-release-action_rust/dist/i686-unknown-linux-gnu/BINARY_NAME-i686-unknown-linux-gnu",
            "label": "i686-unknown-linux-gnu"
          },
          {
            "path": ".semantic-release-action_rust/dist/x86_64-apple-darwin/BINARY_NAME-x86_64-apple-darwin",
            "label": "x86_64-apple-darwin"
          },
          {
            "path": ".semantic-release-action_rust/dist/aarch64-unknown-linux-musl/BINARY_NAME-aarch64-unknown-linux-musl",
            "label": "aarch64-unknown-linux-musl"
          },
          {
            "path": ".semantic-release-action_rust/dist/aarch64-unknown-linux-gnu/BINARY_NAME-aarch64-unknown-linux-gnu",
            "label": "aarch64-unknown-linux-gnu"
          },
          {
            "path": ".semantic-release-action_rust/dist/aarch64-apple-darwin/BINARY_NAME-aarch64-apple-darwin",
            "label": "aarch64-apple-darwin"
          },
          {
            "path": ".semantic-release-action_rust/dist/SHA256SUMS.txt",
            "label": "SHA256SUMS.txt"
          }
        ]
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md",
          "Cargo.toml",
          "Cargo.lock"
        ]
      }
    ]
  ]
}
"#;

fn check(initial: &str, whitelist: &str, expected: &str) {
    let mut manifest = SemanticReleaseManifest::parse_from_string(initial).unwrap();
    manifest.apply_whitelist(HashSet::from_iter(
        whitelist.split_whitespace().map(|s| s.to_owned()),
    ));
    assert_eq!(expected.trim(), manifest.to_string())
}

#[test]
fn should_remove_all_assets_when_whitelist_is_empty() {
    check(
        SEMANTIC_RELEASE_CONFIG,
        "",
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release-cargo/semantic-release-cargo",
    [
      "semantic-release-major-tag",
      {
        "customTags": [
          "v${major}",
          "v${major}.${minor}"
        ]
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": []
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md",
          "Cargo.toml",
          "Cargo.lock"
        ]
      }
    ]
  ]
}
    "#,
    )
}

#[test]
fn should_not_change_file_when_whitelist_matches_contents() {
    check(
        SEMANTIC_RELEASE_CONFIG,
        "x86_64-unknown-linux-musl x86_64-unknown-linux-gnu i686-unknown-linux-musl i686-unknown-linux-gnu x86_64-apple-darwin aarch64-unknown-linux-musl aarch64-unknown-linux-gnu aarch64-apple-darwin SHA256SUMS.txt",
        SEMANTIC_RELEASE_CONFIG,
    )
}

#[test]
fn should_work_with_leading_and_trailing_whitespace() {
    check(
        SEMANTIC_RELEASE_CONFIG,
        " aarch64-apple-darwin ",
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release-cargo/semantic-release-cargo",
    [
      "semantic-release-major-tag",
      {
        "customTags": [
          "v${major}",
          "v${major}.${minor}"
        ]
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": [
          {
            "path": ".semantic-release-action_rust/dist/aarch64-apple-darwin/BINARY_NAME-aarch64-apple-darwin",
            "label": "aarch64-apple-darwin"
          }
        ]
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md",
          "Cargo.toml",
          "Cargo.lock"
        ]
      }
    ]
  ]
}
    "#,
    )
}

#[test]
fn should_work_with_a_whitelist_of_multiple_items() {
    check(
        SEMANTIC_RELEASE_CONFIG,
        r#"
aarch64-apple-darwin
aarch64-unknown-linux-gnu
SHA256SUMS.txt
        "#,
        r#"
{
  "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    "@semantic-release-cargo/semantic-release-cargo",
    [
      "semantic-release-major-tag",
      {
        "customTags": [
          "v${major}",
          "v${major}.${minor}"
        ]
      }
    ],
    [
      "@semantic-release/github",
      {
        "assets": [
          {
            "path": ".semantic-release-action_rust/dist/aarch64-unknown-linux-gnu/BINARY_NAME-aarch64-unknown-linux-gnu",
            "label": "aarch64-unknown-linux-gnu"
          },
          {
            "path": ".semantic-release-action_rust/dist/aarch64-apple-darwin/BINARY_NAME-aarch64-apple-darwin",
            "label": "aarch64-apple-darwin"
          },
          {
            "path": ".semantic-release-action_rust/dist/SHA256SUMS.txt",
            "label": "SHA256SUMS.txt"
          }
        ]
      }
    ],
    [
      "@semantic-release/git",
      {
        "assets": [
          "CHANGELOG.md",
          "Cargo.toml",
          "Cargo.lock"
        ]
      }
    ]
  ]
}
    "#,
    )
}
