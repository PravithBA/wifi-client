use inquire::InquireError;
use inquire::{Password, Select};
use std::process::Command;

fn main() {
    println!("Scanning for wifi...");
    let get_ssid_output = Command::new("nmcli")
        .arg("-t")
        .arg("-f")
        .arg("SSID")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("`nmcli` returned an error while scanning for wifi networks");
    let get_ssid_output_string = match get_ssid_output.status.success() {
        true => String::from_utf8_lossy(&get_ssid_output.stdout).to_string(),
        false => {
            panic!("{}", String::from_utf8_lossy(&get_ssid_output.stderr))
        }
    };
    if get_ssid_output_string.len() == 0 {
        panic!("`nmcli` returned an error while scanning for wifi networks")
    }
    let mut wifi_ssids: Vec<String> = get_ssid_output_string
        .split("\n")
        .map(|s| s.to_string())
        .collect();
    wifi_ssids.remove(wifi_ssids.len() - 1);
    let wifi_ssid_options: Vec<&str> = wifi_ssids.iter().map(|s| &**s).collect();
    let ssid_answer: Result<&str, InquireError> =
        Select::new("Select the SSID you want to connect", wifi_ssid_options).prompt();
    let selected_ssid = match ssid_answer {
        Ok(choice) => choice,
        Err(_) => panic!("There was an error, please try again"),
    };
    let password_answer =
        Password::new(format!("Please enter the password for '{}'", selected_ssid).as_str())
            .without_confirmation()
            .prompt();
    let password_answer = match password_answer {
        Ok(password) => password,
        Err(_) => panic!("There was an error, please try again"),
    };
    let connect_ssid_output = Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("connect")
        .arg(format!("{}", selected_ssid))
        .arg("password")
        .arg(format!("{}", password_answer))
        .output();
    let connect_ssid_output = match connect_ssid_output {
        Ok(ok) => ok,
        Err(error) => {
            println!("Panicked: {}", error);
            panic!("Error: {error}");
        }
    };
    println!("{}", String::from_utf8_lossy(&connect_ssid_output.stderr));
    let connect_ssid_output_string = match connect_ssid_output.status.success() {
        true => String::from_utf8_lossy(&connect_ssid_output.stdout).to_string(),
        false => {
            panic!("{}", String::from_utf8_lossy(&connect_ssid_output.stderr))
        }
    };
    println!("{}", connect_ssid_output_string);
}
