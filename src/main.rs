use inquire::InquireError;
use inquire::{Password, Select};
use std::error::Error;
use std::process::{self, Command};
mod error;
use error::CommandError;
use spinners::{Spinner, Spinners};

fn main() {
    let mut sp = Spinner::new(Spinners::Aesthetic, "Scanning for wifi".into());
    let wifi_ssids = scan_ssids();
    let wifi_ssids = match wifi_ssids {
        Ok(ok) => ok,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };
    sp.stop();
    println!("");
    loop {
        let wifi_ssid_options: Vec<&str> = wifi_ssids.iter().map(|s| &**s).collect();
        let ssid_answer: Result<&str, InquireError> =
            Select::new("Select the SSID you want to connect", wifi_ssid_options).prompt();
        let selected_ssid = match ssid_answer {
            Ok(choice) => choice,
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };
        let password_answer =
            Password::new(format!("Please enter the password for \"{}\"", selected_ssid).as_str())
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
            format!("Connecting to \"{}\"", selected_ssid).into(),
        );
        let connect_ssid_result =
            connect_ssid(&selected_ssid.to_string(), Option::Some(password_answer));
        sp.stop();
        match connect_ssid_result {
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

fn scan_ssids() -> Result<Vec<String>, Box<dyn Error>> {
    // Rescan before getting the ssids
    Command::new("nmcli")
        .arg("dev")
        .arg("wifi")
        .arg("rescan")
        .output()?;
    let get_ssid_output = Command::new("nmcli")
        .arg("-t")
        .arg("-f")
        .arg("SSID")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output();
    let connect_ssid_output = get_ssid_output?.to_owned();
    let connect_ssid_output_string = match connect_ssid_output.status.success() {
        true => {
            let get_ssid_output_string =
                String::from_utf8_lossy(&connect_ssid_output.stdout).to_string();
            let mut wifi_ssids: Vec<String> = get_ssid_output_string
                .split("\n")
                .map(|s| s.to_string())
                .collect();
            wifi_ssids.remove(wifi_ssids.len() - 1);
            Ok(wifi_ssids)
        }
        false => Err(CommandError::new(
            String::from_utf8_lossy(&connect_ssid_output.stderr).to_string(),
        )
        .into()),
    };
    connect_ssid_output_string
}

fn connect_ssid(ssid: &String, password: Option<String>) -> Result<String, Box<dyn Error>> {
    let connect_ssid_output = match password {
        Some(password) => Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("connect")
            .arg(format!("{}", ssid))
            .arg("password")
            .arg(format!("{}", password))
            .output(),
        None => Command::new("nmcli")
            .arg("dev")
            .arg("wifi")
            .arg("connect")
            .arg(format!("{}", ssid))
            .output(),
    };
    let connect_ssid_output = connect_ssid_output?.to_owned();
    let connect_ssid_output_string = match connect_ssid_output.status.success() {
        true => Ok(String::from_utf8_lossy(&connect_ssid_output.stdout).to_string()),
        false => Err(CommandError::new(
            String::from_utf8_lossy(&connect_ssid_output.stderr).to_string(),
        )
        .into()),
    };
    connect_ssid_output_string
}
