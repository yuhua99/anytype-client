use std::{collections::BTreeMap, fmt};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Unknown fields preserved for forward-compatible API responses.
pub type ExtraFields = BTreeMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "format", rename_all = "snake_case")]
pub enum Icon {
    Emoji {
        emoji: String,
    },
    File {
        file: String,
    },
    #[serde(rename = "icon")]
    Named {
        name: String,
        color: IconColor,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub enum IconColor {
    Grey,
    Yellow,
    Orange,
    Red,
    Pink,
    Purple,
    Blue,
    Ice,
    Teal,
    Lime,
    /// Fallback for colors not modeled by the CLI.
    /// Response-side only; not accepted as CLI input.
    #[serde(other)]
    #[value(skip)]
    Unknown,
}

impl fmt::Display for IconColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Grey => "grey",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Red => "red",
            Self::Pink => "pink",
            Self::Purple => "purple",
            Self::Blue => "blue",
            Self::Ice => "ice",
            Self::Teal => "teal",
            Self::Lime => "lime",
            Self::Unknown => "unknown",
        };
        f.write_str(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: Vec<T>,
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub total: Option<i64>,
    pub has_more: Option<bool>,
}
