mod license;
mod helpers;

fn main() {
    // fetch licenses
    let licenses = license::Licenses::fetch_licenses();

    //  select menu
    let license = helpers::select(&licenses.get_license_names());

    // get license content
    let license_content = &licenses.get_license_from_name(&license);

    // fill content
    helpers::fill_content(license_content);
}
