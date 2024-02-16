//! Helpers for parsing status codes.

use serde::{Deserialize, Serialize};

type StatusCode = u32;

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct PrinterStatus {
    /// No response from the TM printer
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_response: bool,
    /// Printing is successfully completed
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub success: bool,
    /// Status of the 3rd pin of the drawer kick-out connector = "H"
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub drawer_kick_out_connector: bool,
    /// Off line status from remaining battery
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub battery_offline_status: bool,
    /// Offline
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub offline: bool,
    /// The cover is open
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub cover_open: bool,
    /// Paper is being fed by a paper feed switch operation
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub paper_feed_operation: bool,
    /// Waiting to be brought back online
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub waiting_online: bool,
    /// The paper feed switch is being pressed
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub paper_feed_switch_pressed: bool,
    /// A mechanical error occurred
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub mechanical_error: bool,
    /// An autocutter error occurred
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub autocutter_error: bool,
    /// An unrecoverable error occurred
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub unrecoverable: bool,
    /// Automatically recoverable error occurred
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub recoverable: bool,
    /// No paper in roll paper near end sensor
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_paper_roll_near_end_sensor: bool,
    /// No paper in roll paper ens sensor
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_paper_in_roll_paper_end_sensor: bool,
    /// A buzzer is on (only for applicable devices)
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub buzzer_on: bool,
    /// Waiting period for removal of label (only for applicable devices)
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub label_wait_removal: bool,
    /// No paper in label peeling sensor (only for applicable devices)
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_paper_in_peel_sensor: bool,
    /// The spooler has stopped (unused)
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub spooler_stopped: bool,
}

impl std::fmt::Display for PrinterStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = serde_json::to_string(self).unwrap();
        write!(f, "{}", str)
    }
}

impl From<StatusCode> for PrinterStatus {
    fn from(value: StatusCode) -> Self {
        PrinterStatus {
            no_response: (value & 0x00000001) != 0,
            success: (value & 0x00000002) != 0,
            drawer_kick_out_connector: (value & 0x00000004) != 0,
            battery_offline_status: (value & 0x00000004) != 0,
            offline: (value & 0x00000008) != 0,
            cover_open: (value & 0x00000020) != 0,
            paper_feed_operation: (value & 0x00000040) != 0,
            waiting_online: (value & 0x00000100) != 0,
            paper_feed_switch_pressed: (value & 0x00000200) != 0,
            mechanical_error: (value & 0x00000400) != 0,
            autocutter_error: (value & 0x00000800) != 0,
            unrecoverable: (value & 0x00002000) != 0,
            recoverable: (value & 0x00004000) != 0,
            no_paper_roll_near_end_sensor: (value & 0x00020000) != 0,
            no_paper_in_roll_paper_end_sensor: (value & 0x00080000) != 0,
            buzzer_on: (value & 0x01000000) != 0,
            label_wait_removal: (value & 0x01000000) != 0,
            no_paper_in_peel_sensor: (value & 0x40000000) != 0,
            spooler_stopped: (value & 0x80000000) != 0,
        }
    }
}


#[derive(Deserialize, Serialize, Debug, Default)]
/// A status response from ePOS's SOAP API
pub struct Response {
    /// SOAP namespace.
    #[serde(rename = "@xmlns")]
    pub ns: String,
    #[serde(rename = "@success")]
    /// Was the last print successful?
    pub success: bool,
    #[serde(rename = "@code")]
    /// String code, in cases of failure
    pub code: String,
    #[serde(rename = "@status")]
    /// A 32-bit bitmask of current status bits set
    /// Convert to a more readable struct by
    /// `let out: PrinterStatus = status.into()`
    pub status: StatusCode,
    /// Battery status code
    #[serde(rename = "@battery")]
    pub battery: u32
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let codes: PrinterStatus = self.status.into();
        write!(f, "status: '{}' codes: {}", self.code, codes)
    }
}


#[cfg(test)]
mod tests {
    use super::{PrinterStatus, Response};

    #[test]
    fn test_status_bitmask() {
        let example = 251658262;
        let status = Response{status: example, ..Default::default()};
        let res: PrinterStatus = status.status.into();
        let codes = PrinterStatus {
            success: true,
            drawer_kick_out_connector: true,
            battery_offline_status: true,
            buzzer_on: true,
            label_wait_removal: true,
            ..Default::default()
        };
        println!("status: {}", res);
        assert_eq!(codes, res);
    }

    #[test]
    fn test_status_to_string(){
        let example = 251658262;
        let status = Response{status: example, ..Default::default()};
        let out: PrinterStatus = status.status.into();
        assert_eq!(r#"{"success":true,"drawer_kick_out_connector":true,"battery_offline_status":true,"buzzer_on":true,"label_wait_removal":true}"#, out.to_string())
    }
}