//! Helpers and types for creating barcodes.
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
/// types of available 1D barcodes.
/// Docs here are taken from the full XML spec: <https://files.support.epson.com/pdf/pos/bulk/epos-print_xml_um_en_revi.pdf>
/// Binary data can also be specified with \xnn and \\ to print a backslash
pub enum BarcodeType {
    #[serde(rename = "upc_a")]
    /// When an 11-digit number is specified, a check digit is automatically added. When a 12-digit number is specified, the 12th digit is processed as a check digit but the check digit is not validated.
    UpcA,
    #[serde(rename = "upc_e")]
    /// Specify 0 as the first digit. Specify the manufacturer code in the digits 2 to 6. Specify (right-align) the item code in the digits 7 to 11. 
    /// he number of item code digits varies depending on the manufacturer code. Specify 0s in empty digits. Same check digit rules apply as upc_a.
    UpcE,
    #[serde(rename = "ean13")]
    /// When an 12-digit number is specified, a check digit is automatically added. When a 13-digit number is specified, the 12th digit is processed as a check digit but the check digit is not validated.
    EAN13,
    #[serde(rename = "jan13")]
    /// When an 12-digit number is specified, a check digit is automatically added. When a 13-digit number is specified, the 12th digit is processed as a check digit but the check digit is not validated.
    JAN13,
    #[serde(rename = "EAN8")]
    /// When a 7-digit number is specified, a check digit is automatically added. When an 8-digit number is specified, the 8th digit is processed as a check digit but the check digit is not validated.
    EAN8,
    #[serde(rename = "JAN8")]
    /// When a 7-digit number is specified, a check digit is automatically added. When an 8-digit number is specified, the 8th digit is processed as a check digit but the check digit is not validated.
    JAN8,
    #[serde(rename = "code39")]
    /// When the first character is *, the character is processed as the start character. In other cases, a start character is automatically added.
    Code39,
    #[serde(rename = "itf")]
    /// Start and stop codes are automatically added. Check digits are not added or validated.
    ITF,
    #[serde(rename = "codabar")]
    /// Specify a start and stop character (A to D, a to d). Check digits are not added or validated.
    CodaBar,
    #[serde(rename = "code93")]
    /// Start and stop characters are automatically added. A check digit is automatically calculated and added.
    Code93,
    #[serde(rename = "code128")]
    /// Specify a start character (CODE A, CODE B, CODE C).
    /// A stop character is automatically added
    /// A check digit is automatically calculated and added.
    /// To encode each of the following characters, specify two characters starting with the character `{`:
    /// FNC1: {1
    /// FNC2: {2
    /// FNC3: {3
    /// FNC4: {4
    /// CODE A: {A
    /// CODE B: {B
    /// CODE C: {C
    /// SHIFT: {S
    /// {: {{
    Code128,
    #[serde(rename = "gs1_128")]
    /// A start character, FNC1, a check digit, and a stop character are automatically added
    /// To automatically calculate and add a check digit for an application identifier (AI) and the subsequent data, specify the character "*" in the position of the check digit.
    /// You can enclose an application identifier (AI) in parentheses. The parentheses are used as HRI print characters and are not encoded as data
    /// You can insert spaces between an application identifier (AI) and data. The spaces are used as HRI print characters and are not encoded as data.
    /// To encode each of the following characters, specify two characters starting with the character "{":
    /// FNC1: {1
    /// FNC3: {3
    /// (: {(
    /// ): {)
    /// *: {*
    /// {: {{
    GS1_128,
    #[serde(rename = "gs1_databar_omnidirectional")]
    /// Specify a 13-digit global trade item number (GTIN) not including an application identifier (AI) or a check digit.
    GS1DatabarOmnidirectional,
    #[serde(rename = "gs1_databar_truncated")]
    /// Specify a 13-digit global trade item number (GTIN) not including an application identifier (AI) or a check digit.
    GS1DatabarTruncated,
    #[serde(rename = "gs1_databar_limited")]
    /// Specify a 13-digit global trade item number (GTIN) not including an application identifier (AI) or a check digit.
    GS1DatabarLimited,
    #[serde(rename = "gs1_databar_expanded")]
    /// You can enclose an application identifier (AI) in parentheses. The parentheses are used as HRI print characters and are not encoded as data
    /// To encode each of the following characters, specify two characters starting with the character "{"
    /// FNC1: {1
    /// (: {(
    /// ): {)
    Gs1DatabarExpanded
}

#[derive(Deserialize, Serialize, Debug)]
/// types of available 2D barcodes.
/// Docs here are taken from the full XML spec: <https://files.support.epson.com/pdf/pos/bulk/epos-print_xml_um_en_revi.pdf>
pub enum SymbolType {
    #[serde(rename = "pdf417_standard")]
    /// The data area can contain up to 928 code words in a maximum of 90 rows, each of which can contain up to 30 code words.
    PDF417,
    #[serde(rename = "pdf417_truncated")]
    PDF415Trunc,
    #[serde(rename = "qrcode_model_1")]
    /// QRCode model 1
    QRCode1,
    #[serde(rename = "qrcode_model_2")]
    /// QRCode model 2
    QRCode2,
    #[serde(rename = "maxicode_mode_2")]
    /// Mode 2: Formatted data containing a structured Carrier Message with a numeric postal code. 
    /// When the first piece of data is [)>\ x1e01\x1dyy (where yy is a two-digit number), this is processed as the message
    /// header, and the subsequent data is processed as the primary message. In other cases, from the first piece of data, data is processed as
    /// the primary message.
    /// 
    /// Normally, specify the primary message in the following format: 
    /// Postal code (1- to 9-digit number) GS:(\x1d) ISO country code (1- to 3-digit number) GS:(\x1d) 
    /// Service class code (1- to 3-digit number)
    /// 
    /// Example: 908063840\x1d850\x1d001\x1d\x04
    MaxiCodeMode2,
    #[serde(rename = "maxicode_mode_3")]
    /// Mode 3: Formatted data containing a structured Carrier Message with an alphanumeric postal code.
    /// When the first piece of data is [)>\ x1e01\x1dyy (where yy is a two-digit number), this is processed as the message
    /// header, and the subsequent data is processed as the primary message. In other cases, from the first piece of data, data is processed as
    /// the primary message.
    /// 
    /// Normally,  specify the primary message in the following format:
    /// Postal code (1 to 6 pieces of data convertible by Code Set A)
    ///GS:(\x1d) ISO country code (1- to 3-digit number) GS:(\x1d) Service
    ///class code (1- to 3-digit number)
    MaxiCodeMode3,
    #[serde(rename = "maxicode_mode_4")]
    /// Mode 4: Unformatted data with Standard Error Correction.
    MaxiCodeMode4,
    #[serde(rename = "maxicode_mode_5")]
    /// Unformatted data with Enhanced Error Correction.
    MaxiCodeMode5,
    #[serde(rename = "maxicode_mode_6")]
    /// Used for programming hardware devices.
    MaxiCodeMode6,
    #[serde(rename = "gs1_databar_stacked")]
    /// Specify a 13-digit global trade item number (GTIN) not including an application identifier (AI) or a check digit.
    GS1DatabarStacked,
    #[serde(rename = "gs1_databar_stacked_omnidirectional")]
    GS1DatabarStackedOmnidirectional,
    #[serde(rename = "gs1_databar_expanded_stacked")]
    /// You can enclose an application identifier (AI) in parentheses. The
    /// parentheses are used as HRI print characters and are not encoded as data.
    GS1DatabarExpandedStacked,
    #[serde(rename = "azteccode_fullrange")]
    AztecCodeFullRange,
    #[serde(rename = "azteccode_compact")]
    AztecCodeCompact,
    #[serde(rename = "datamatrix_square")]
    DatamatrixSquare,
    #[serde(rename = "datamatrix_rectangle_8")]
    DatamatrixRectangle8,
    #[serde(rename = "datamatrix_rectangle_12")]
    DatamatrixRectangle12,
    #[serde(rename = "datamatrix_rectangle_16")]
    DatamatrixRectangle16,
}

#[derive(Deserialize, Serialize, Debug)]
/// Specifies the HRI position for a barcode
pub enum HRI {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "above")]
    Above,
    #[serde(rename = "below")]
    Below,
    #[serde(rename = "both")]
    Both
}

#[derive(Deserialize, Serialize, Debug)]
/// Error correction levels
/// variants level_{0-8} are used by PDF417,
/// while level_{l-h} are used by QRCode.
/// Aztec codes will take a pure integer value ranging from 5-95
pub enum ErrorCorrectionLevel {
    #[serde(rename = "level_0")]
    Level0,
    #[serde(rename = "level_1")]
    Level1,
    #[serde(rename = "level_2")]
    Level2,
    #[serde(rename = "level_3")]
    Level3,
    #[serde(rename = "level_4")]
    Level4,
    #[serde(rename = "level_5")]
    Level5,
    #[serde(rename = "level_6")]
    Level6,
    #[serde(rename = "level_7")]
    Level7,
    #[serde(rename = "level_8")]
    Level8,
    #[serde(rename = "level_l")]
    LevelL,
    #[serde(rename = "level_m")]
    LevelM,
    #[serde(rename = "level_q")]
    LevelQ,
    #[serde(rename = "level_h")]
    LevelH,
    #[serde(rename = "default")]
    Default,
    Int(u32)
}