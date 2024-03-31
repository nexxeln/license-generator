mod helpers;
mod license;

use clap::Parser;
use license::Licenses;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    license_name: Option<String>,

    #[arg(short, long)]
    skip_prompt: Option<bool>,
}

fn main() {
    let licenses = Licenses::fetch_licenses();
    let valid_license_names = &licenses.get_license_keys();

    let args = Args::parse();
    let license_name = &args.license_name.unwrap_or(String::from("")).to_lowercase();

    let license_content = if valid_license_names.contains(license_name) {
        licenses.get_license_from_key(license_name)
    } else {
        let license = helpers::select(&licenses.get_license_names());

        licenses.get_license_from_name(&license)
    };

    helpers::fill_content(&license_content, args.skip_prompt.unwrap_or(false));
}
