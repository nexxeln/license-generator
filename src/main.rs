mod helpers;
mod license;

use clap::Parser;
use license::LicenseContent;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    license_name: Option<String>,

    #[arg(short, long)]
    skip_prompt: Option<bool>,
}

fn main() {
    let licenses = license::Licenses::fetch_licenses();
    let valid_license_names = &licenses.get_license_keys();

    let args: Args = Args::parse();
    let license_name = &args.license_name.unwrap_or(String::from("")).to_lowercase();
    let license_content: LicenseContent;

    if valid_license_names.contains(&license_name) {
        license_content = licenses.get_license_from_key(&license_name);
    } else {
        let license = helpers::select(&licenses.get_license_names());
        license_content = licenses.get_license_from_name(&license);
    }

    helpers::fill_content(&license_content, args.skip_prompt.unwrap_or(false));
}
