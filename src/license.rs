use serde::Deserialize;

// all structs are from https://api.github.com/licenses
#[derive(Debug, Deserialize)]
pub struct License {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Debug, Deserialize)]
pub struct LicenseContent {
    pub key: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<String>,
    pub conditions: Vec<String>,
    pub limitations: Vec<String>,
    pub body: String,
}

impl LicenseContent {
    // fetch license content
    pub fn fetch_license_content(url: &str) -> LicenseContent {
        match ureq::get(url).call() {
            Ok(res) => res
                .into_json::<LicenseContent>()
                .expect("failed to parse license content json"),
            Err(error) => panic!("Unable to fetch license content: {error}"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Licenses {
    pub license: Vec<License>,
}

impl Licenses {
    // fetch all licenses from github
    pub fn fetch_licenses() -> Licenses {
        let body: Vec<License> = match ureq::get("https://api.github.com/licenses").call() {
            Ok(res) => res.into_json().expect("failed to parse licenses json"),
            Err(error) => panic!("Unable to fetch licenses: {error}"),
        };

        Licenses { license: body }
    }

    pub fn get_license_names(&self) -> Vec<String> {
        self.license.iter().map(|l| l.name.to_string()).collect()
    }

    pub fn get_license_keys(&self) -> Vec<String> {
        self.license.iter().map(|l| l.key.to_string()).collect()
    }

    pub fn get_license_from_name(&self, name: &String) -> LicenseContent {
        let lic = &self.license;

        let result = lic
            .iter()
            .find(|l| &l.name == name)
            .map(|l| &l.url)
            .expect("no license found for name");

        LicenseContent::fetch_license_content(result)
    }

    pub fn get_license_from_key(&self, key: &String) -> LicenseContent {
        let lic = &self.license;

        let result = lic
            .iter()
            .find(|l| &l.key == key)
            .map(|l| &l.url)
            .expect("no license found for key");

        LicenseContent::fetch_license_content(result)
    }
}
