mod helpers;
mod license;

use license::{License, LicenseContent};

fn main() {
    // fetch licenses
    let licenses = License::fetch();

    //  select menu
    let license_name: String = helpers::select(&licenses.iter().map(|l: &License| l.name.clone()).collect());

    // get selected license
    let selected_license: &License = licenses.iter().find(|l| l.name == license_name).unwrap();

    // get license content
    let license_content = LicenseContent::from(selected_license);

    // fill content
    helpers::fill_content(&license_content);
}
