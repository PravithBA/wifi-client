use inquire::InquireError;
use inquire::Select;
use std::process::Command;

fn main() {
    println!("Scanning for wifi...");
    let output = Command::new("nmcli")
        .arg("-t")
        .arg("-f")
        .arg("SSID")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("`nmcli` returned an error while scanning for wifi networks");
    let output_string = String::from_utf8_lossy(&output.stdout).to_string();
    let wifi_ssids: Vec<String> = output_string
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    let wifi_ssid_options: Vec<&str> = wifi_ssids.iter().map(|s| &**s).collect();
    let answer: Result<&str, InquireError> =
        Select::new("Select the SSID you want to connect", wifi_ssid_options).prompt();
    let selected_ssid: &str;
    match answer {
        Ok(choice) => selected_ssid = choice,
        Err(_) => panic!("There was an error, please try again"),
    }
    println!("Your selected SSID is: `{}`", selected_ssid);
}
