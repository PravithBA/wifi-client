use inquire::InquireError;
use inquire::{Password, Select};
use spinners::{Spinner, Spinners};
use std::error::Error;
use std::process::{self, Command};
mod error;
use error::CommandError;
mod wifi_data;
use wifi_data::WifiData;

fn main() {
    loop {
        let mut sp = Spinner::new(Spinners::Aesthetic, "Scanning for wifi".into());
        let wifis = get_nearby_wifi();
        let wifi_options = match wifis {
            Ok(ok) => ok,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
        sp.stop();
        println!("");
        let wifi_answer: Result<WifiData, InquireError> =
            Select::new("Select the SSID you want to connect", wifi_options).prompt();

        let selected_wifi = match wifi_answer {
            Ok(choice) => choice,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
        let password_answer =
            Password::new(format!("Please enter the password for \"{}\"", selected_wifi).as_str())
                .without_confirmation()
                .prompt();
        let password_answer = match password_answer {
            Ok(password) => password,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
        let mut sp = Spinner::new(
            Spinners::Aesthetic,
            format!("Connecting to \"{}\"", selected_wifi).into(),
        );
        let connect_wifi_result = connect_wifi(&selected_wifi, Option::Some(password_answer));
        sp.stop();
        match connect_wifi_result {
            Ok(ok) => {
                println!("{}", ok);
                process::exit(1)
            }
            Err(error) => {
                eprintln!("{}", error);
                continue;
            }
        }
    }
}

fn split_without_escape_character(string: &str, split_string: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut current = String::new();
    let mut escaped = false;

    for c in string.chars() {
        if c == split_string && !escaped {
            result.push(current.clone());
            current.clear();
        } else {
            current.push(c);
            escaped = c == '\\' && !escaped;
        }
    }

    result.push(current);
    result
}

fn get_nearby_wifi() -> Result<Vec<WifiData>, Box<dyn Error>> {
    // Rescan before getting the wifis
    Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("rescan")
        .output()?;
    let get_wifi_output = Command::new("nmcli")
        .arg("-t")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output();
    let connect_wifi_output = get_wifi_output?.to_owned();
    let connect_wifi_output_string: Result<Vec<String>, CommandError> =
        match connect_wifi_output.status.success() {
            true => {
                let get_wifi_output_string =
                    String::from_utf8_lossy(&connect_wifi_output.stdout).to_string();
                let mut wifi_strings: Vec<String> = get_wifi_output_string
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect();
                wifi_strings.remove(wifi_strings.len() - 1);
                Ok(wifi_strings)
            }
            false => Err(CommandError::new(
                String::from_utf8_lossy(&connect_wifi_output.stderr).to_string(),
            )
            .into()),
        };
    let connect_wifi_output_string = connect_wifi_output_string?;
    let split_string: char = ':';
    let wifis: Vec<WifiData> = connect_wifi_output_string
        .into_iter()
        .map(|wifi_string| -> WifiData {
            let wifi_split_string = split_without_escape_character(&wifi_string, split_string);
            WifiData::new(
                if wifi_split_string[0] == "*" {
                    true
                } else {
                    false
                },
                wifi_split_string[1].to_owned(),
                wifi_split_string[2].to_owned(),
                wifi_split_string[3].to_owned(),
                wifi_split_string[4].to_owned(),
                wifi_split_string[5].to_owned(),
                wifi_split_string[6].to_owned(),
                wifi_split_string[7].to_owned(),
                wifi_split_string[8].to_owned(),
            )
        })
        .collect();
    Ok(wifis)
}

fn connect_wifi(wifi: &WifiData, password: Option<String>) -> Result<String, Box<dyn Error>> {
    let connect_wifi_output = match password {
        Some(password) => Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("connect")
            .arg(format!("{}", wifi.ssid))
            .arg("password")
            .arg(format!("{}", password))
            .output(),
        None => Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("connect")
            .arg(format!("{}", wifi.ssid))
            .output(),
    };
    let connect_wifi_output = connect_wifi_output?.to_owned();
    let connect_wifi_output_string = match connect_wifi_output.status.success() {
        true => Ok(String::from_utf8_lossy(&connect_wifi_output.stdout).to_string()),
        false => Err(CommandError::new(
            String::from_utf8_lossy(&connect_wifi_output.stderr).to_string(),
        )
        .into()),
    };
    connect_wifi_output_string
}
