use std::fmt::{self, Display};

#[derive(Default)]
pub struct WifiData {
    pub in_use: bool,
    pub bssid: String,
    pub ssid: String,
    pub mode: String,
    pub channel: String,
    pub rate: String,
    pub signal: String,
    pub bars: String,
    pub security: String,
}

impl WifiData {
    pub fn new(
        in_use: bool,
        bssid: String,
        ssid: String,
        mode: String,
        channel: String,
        rate: String,
        signal: String,
        bars: String,
        security: String,
    ) -> WifiData {
        WifiData {
            in_use,
            bssid,
            ssid,
            mode,
            channel,
            rate,
            signal,
            bars,
            security,
        }
    }
}

impl Display for WifiData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SSID: \"{}\"", self.ssid)
    }
}
