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