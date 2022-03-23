mod license;
mod helpers;

fn main() {
    let licenses = license::Licenses::fetch();
    let license = helpers::select(&licenses.get_license_names());
    let _license = &licenses.get_license_from_name(&license);

    helpers::fill_license(_license);
}
