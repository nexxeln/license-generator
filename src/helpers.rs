use crate::license;
use chrono::{Datelike, Local};
use dialoguer::{console::Style, theme::ColorfulTheme, FuzzySelect, Input};
use license::LicenseContent;
use std::{fs, io, process::Command};

// main logic to fill the license with name and year
pub fn fill_content(license: &LicenseContent, skip_prompt: bool) {
    let name = get_name(skip_prompt);
    let year = get_year(skip_prompt);

    // replacing content
    let body = license
        .body
        .replace("[year]", &year)
        .replace("[yyyy]", &year)
        .replace("[fullname]", &name)
        .replace("[name of copyright owner]", &name)
        .replace("<year>", &year)
        .replace("<name of author>", &name);

    // write file according to replaced content
    match write_file("LICENSE", &body) {
        Ok(_) => println!(
            "{}",
            Style::new()
                .for_stderr()
                .green()
                .apply_to("✔ License created successfully\n  Please take a look at it and make changes if necessary")
        ),
        Err(error) => println!(
            "{} {}",
            Style::new()
                .for_stderr()
                .red()
                .apply_to("✘ An error occurred"),
            error
        ),
    };
}

// select license
pub fn select(selections: &Vec<String>) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a license")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    selections[selection].clone()
}

// try to get username from global git config
fn get_git_username() -> Option<String> {
    let cmd = Command::new("git")
        .arg("config")
        .arg("--global")
        .arg("--get")
        .arg("user.name")
        .output()
        .ok()?;

    let res: Option<String> = match cmd.status.success() {
        true => Option::from(String::from_utf8_lossy(&cmd.stdout).to_string()),
        false => Option::from(None),
    };

    res
}

// get name from user
fn get_name(skip_prompt: bool) -> String {
    let name: String = match get_git_username() {
        Some(mut name) => {
            // removing trailing newline (cross platform way)
            if name.ends_with("\n") {
                name.pop();

                if name.ends_with("\r") {
                    name.pop();
                }
            }

            if !skip_prompt {
                return Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Enter your name")
                    .default(name.clone())
                    .interact_text()
                    .unwrap();
            }

            name
        }
        None => {
            let input: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Name")
                .interact_text()
                .unwrap();

            input
        }
    };

    name
}

// get year from user
fn get_year(skip_prompt: bool) -> String {
    let current_year = Local::now().year();

    if skip_prompt {
        return current_year.to_string();
    }

    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter year")
        .default(current_year.to_string())
        .interact_text()
        .unwrap()
}

// write license file
fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    let result = match !fs::metadata(path).is_ok() {
        false => {
            let path: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "LICENSE already exists, enter a new name else the content will be overridden!",
                )
                .default(path.to_string())
                .interact_text()
                .unwrap();

            fs::write(path, content)
        }
        true => fs::write(path, content),
    };

    result
}
