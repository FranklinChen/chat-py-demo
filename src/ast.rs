//! CHAT AST

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A CHAT transcript.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub struct Chat<'a> {
    /// All the top level lines.
    #[serde(borrow)]
    pub tops: Vec<Top<'a>>,
}

impl<'a> fmt::Display for Chat<'a> {
    /// Output one line for each top-level item.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for top in &self.tops {
            write!(f, "{top}\n")?;
        }
        Ok(())
    }
}

/// Top-level line.
#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq, Eq)]
pub enum Top<'a> {
    /// @...
    Header(#[serde(borrow)] &'a str),

    /// *...
    MainTier(#[serde(borrow)] &'a str),

    /// %...
    DependentTier(#[serde(borrow)] &'a str),

    /// An error!
    UnrecognizedTier(#[serde(borrow)] &'a str),
}

impl<'a> fmt::Display for Top<'a> {
    /// Output string representation of the variant.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Top::Header(s) => write!(f, "{s}"),
            Top::MainTier(s) => write!(f, "{s}"),
            Top::DependentTier(s) => write!(f, "{s}"),
            Top::UnrecognizedTier(s) => write!(f, "{s}"),
        }
    }
}
