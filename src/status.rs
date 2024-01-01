//! Helpers for parsing status codes.
use serde::{Deserialize, Serialize};

const ERROR_BITMAPS: [(&'static str, u32); 19] = [
    // No response from the TM printer
    ("TATUS_NO_RESPONSE", 0x00000001),
    // Printing is successfully completed
    ("PRINT_SUCCESS", 0x00000002),
    // Status of the 3rd pin of the drawer kick-out connector = "H"
    ("DRAWER_KICK_OUT_CONNECTOR", 0x00000004),
    // Off line status from remaining battery
    ("BATT_OFFLINE_STATUS", 0x00000004),
    // Offline
    ("OFFLINE", 0x00000008),
    // The cover is open
    ("COVER_OPEN", 0x00000020),
    // Paper is being fed by a paper feed switch operation
    ("PAPER_FEED_OPERATION", 0x00000040),
    // Waiting to be brought back online
    ("WAITING_TO_BE_ONLINE", 0x00000100),
    // The paper feed switch is being pressed (ON)
    ("PAPER_FEED_SWITCH_PRESSED", 0x00000200),
    // A mechanical error occurred
    ("MECHANICAL_ERROR", 0x00000400),
    // An autocutter error occurred
    ("AUTOCUTTER_ERROR", 0x00000800),
    // An unrecoverable error occurred
    ("UNRECOVERABLE", 0x00002000),
    // An automatically recoverable error occurred
    ("RECOVERABLE", 0x00004000),
    // No paper in roll paper near end sensor
    ("NO_PAPER_ROLL_NEAR_END_SENSOR", 0x00020000),
    // No paper in roll paper end sensor
    ("NO_PAPER_ROLL_END_SENSOR", 0x00080000),
    // A buzzer is on
    ("BUZZER_ON", 0x01000000),
    // Waiting period for removal of label
    ("LABEL_WAIT_REMOVAL", 0x01000000),
    // No paper in label peeling sensor
    ("NO_PAPER_IN_PEEL_SENSOR", 0x40000000),
    // The spooler has stopped
    ("SPOOLER_STOPPED", 0x80000000),
];

#[derive(Deserialize, Serialize, Debug, Default)]
/// The raw response from ePOS's SOAP API
pub struct Response {
    #[serde(rename = "$text")]
    pub body: Option<String>,
    #[serde(rename = "@xmlns")]
    pub ns: String,
    #[serde(rename = "@success")]
    pub success: bool,
    #[serde(rename = "@code")]
    pub code: String,
    #[serde(rename = "@status")]
    /// A 32-bit bitmask of current status bits set
    pub status: u32,
    #[serde(rename = "@battery")]
    pub battery: u32
}

impl Response {
    pub fn status_codes(&self) -> Vec<String> {
        let mut list: Vec<String> = Vec::new();
        for (err_name, code) in ERROR_BITMAPS {
            if self.status & code != 0 {
                list.push(err_name.to_string())
            }
        }
        list
    }
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status: {} codes: {:?}", self.code, self.status_codes())
    }
}

#[cfg(test)]
mod tests {
    use super::Response;

    #[test]
    fn test_status() {
        let example = 251658262;
        let status = Response{status: example, ..Default::default()};
        let res = status.status_codes();
        let codes = vec!["PRINT_SUCCESS".to_string(), "DRAWER_KICK_OUT_CONNECTOR".to_string(), "BATT_OFFLINE_STATUS".to_string(), "BUZZER_ON".to_string(), "LABEL_WAIT_REMOVAL".to_string()];
        assert_eq!(codes, res);
        println!("status: {:?}", res);
    }
}