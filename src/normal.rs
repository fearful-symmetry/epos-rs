use serde::{Deserialize, Serialize};

use crate::formatters::{CutType, Style};

pub trait NormalItem: Serialize {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename ="cut")]
/// Cut the paper.
pub struct  Cut {
    #[serde(rename = "@type")]
    pub cut_type: CutType
}

impl NormalItem for Cut {}

/// Draw a horizontal line
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="hline")]
pub struct Hline {
    /// Specifies the horizontal draw start position in units of dots.
    #[serde(rename = "@x1")]
    pub x1: u16,
    /// Specifies the horizontal draw end position in units of dots.
    #[serde(rename = "@x2")]
    pub x2: u16,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>
}

impl NormalItem for Hline {}
