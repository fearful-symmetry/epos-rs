//! Helper types for formatting text and objects.
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
/// Set alignment for an element
pub enum Align {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "right")]
    Right
}

#[derive(Deserialize, Serialize, Debug)]
/// Feed position settings
pub enum FeedPos {
    #[serde(rename = "peeling")]
    /// Paper feed to the peeling position
    Peeling,
    #[serde(rename = "cutting")]
    /// Paper feed to the cutting position
    Cutting,
    #[serde(rename = "current_tof")]
    /// Paper feed to the head position of current label
    CurrentTof,
    #[serde(rename = "next_tof")]
    /// Paper feed to the head position of next label
    NextTof
}


#[derive(Deserialize, Serialize, Debug)]
/// Specifies the type of paper cut to perform
pub enum CutType {
    #[serde(rename = "no_feed")]
    /// Cut without feeding
    NoFeed,
    #[serde(rename = "feed")]
    /// Feed, then cut
    Feed,
    #[serde(rename = "reserve")]
    /// Print until the cut position
    Reserve
}


#[derive(Deserialize, Serialize, Debug)]
/// Set the language used in the text line
pub enum Lang {
    #[serde(rename = "de")]
    De,
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "en")]
    En,
    #[serde(rename = "it")]
    It,
    #[serde(rename = "es")]
    Es,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ja-jp")]
    JaJp,
    #[serde(rename = "ko")]
    Ko,
    #[serde(rename = "ko-kr")]
    KoKr,
    #[serde(rename = "zh-hans")]
    ZhHans,
    #[serde(rename = "zh-cn")]
    ZhCn,
    #[serde(rename = "zh-hant")]
    ZhHant,
    #[serde(rename = "zh-tw")]
    ZhTw,
    Other(String)
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
/// Set the font
pub enum Font {
    #[serde(rename = "font_a")]
    FontA,
    #[serde(rename = "font_b")]
    FontB,
    #[serde(rename = "font_c")]
    FontC,
    #[serde(rename = "font_d")]
    FontD,
    #[serde(rename = "font_e")]
    FontE
}

#[derive(Deserialize, Serialize, Debug)]
/// Set the color
/// Obviously not available on thermal printers
pub enum Color {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "color_1")]
    Color1,
    #[serde(rename = "color_2")]
    Color2,
    #[serde(rename = "color_3")]
    Color3,
    #[serde(rename = "color_4")]
    Color4
}
