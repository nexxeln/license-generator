use std::{process::Command, fs};
use dialoguer::{console::Style, theme::ColorfulTheme, Input, Select};
use crate::license;

pub fn select(selections: &Vec<String>) -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a license")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].clone()
}

fn get_current_year() -> String {
    let year: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter the year")
        .default("2022".to_string())
        .interact_text()
        .unwrap();

    year
}

// try to get username from git config
fn get_username() -> Option<String> {
    let cmd = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg("user.name")
        .output()
        .expect("failed to get username");

    let result: Option<String> = match cmd.status.success() {
        true => Option::from(String::from_utf8_lossy(&cmd.stdout).to_string()),
        false => Option::from(String::from(None)),
    };

    result
}

fn get_name() -> String {
    let name: String = match get_username() {
        Some(name) => {
            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your name")
                .default(name)
                .interact_text()
                .unwrap();

            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Your name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}

