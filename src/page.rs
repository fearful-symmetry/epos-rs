//! Types that are exclusive to page mode.
use serde::{Deserialize, Serialize};

use crate::formatters::Style;

pub trait PageItem: Serialize {}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="area")]
pub struct Area {
        /// Start point for the print area
        #[serde(rename = "@x")]
        pub x: u16,
        /// End point for the print area
        #[serde(rename = "@y")]
        pub y: u16,
        /// Total Print area width
        #[serde(rename = "@width")]
        pub width: u16,
        /// Total print area height
        #[serde(rename = "@height")]
        pub height: u16,
}

impl PageItem for Area {}



#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename ="rectangle")]
pub struct Rectangle {
    /// Specifies the horizontal draw start position in units of dots.
    #[serde(rename = "@x1")]
    pub x1: u16,
    /// Specifies the vertical draw start position in units of dots.
    #[serde(rename = "@y1")]
    pub y1: u16,
    /// Specifies the horizontal draw end position in units of dots.
    #[serde(rename = "@x2")]
    pub x2:u16,
    /// Specifies the vertical draw end position in units of dots.
    #[serde(rename = "@y2")]
    pub y2: u16,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<Style>    
}

impl PageItem for Rectangle{}

#[cfg(test)]
mod tests {
    use super::{Area, Rectangle};

    #[test]
    fn test_rectangle() {
        let test = Rectangle{x1: 0, y1: 1, x2: 2, y2: 3, style: Some(crate::formatters::Style::Medium)};
        let out = quick_xml::se::to_string(&test).unwrap();   
        assert_eq!(out, String::from(r#"<rectangle x1="0" y1="1" x2="2" y2="3" style="medium"/>"#));
    }

    #[test]
    fn test_area() {
        let test = Area{x: 100, y: 100, width: 200, height: 400};
        let out = quick_xml::se::to_string(&test).unwrap();
        assert_eq!(out, String::from(r#"<area x="100" y="100" width="200" height="400"/>"#));
    }
}