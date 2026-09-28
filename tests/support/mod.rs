//! Helpers shared by the integration tests.

// Every test crate compiles this module on its own and uses a different subset.
#![allow(dead_code)]

use std::{fmt::Debug, path::Path};

use pretty_assertions::assert_eq;
use serde::{Serialize, de::DeserializeOwned};

/// Reads `tests/fixtures/<name>`.
pub fn fixture(name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Deserializes `tests/fixtures/<name>` into `T`.
pub fn from_fixture<T: DeserializeOwned>(name: &str) -> T {
    serde_json::from_str(&fixture(name)).unwrap_or_else(|e| panic!("{name}: {e}"))
}

/// Asserts that `value` survives serialize → deserialize unchanged.
pub fn assert_round_trip<T>(value: &T)
where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
{
    let json = serde_json::to_string(value).expect("serialize");
    let back: T = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(&back, value);
}
