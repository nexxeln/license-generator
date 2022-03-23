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