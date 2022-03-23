use serde::Deserialize;
use ureq::serde_json::Value::String;

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
    pub fn about(url: &String) -> LicenseContent {
        let license_content: LicenseContent = match ureq::get(&url).call() {
            Ok(res) => res.into_json().unwrap(),
            Err(error) => panic!("Unable to fetch license content: {}", error)
        };

        license_content
    }
}

#[derive(Debug, Deserialize)]
pub struct Licenses {
    pub license: Vec<License>
}

impl Licenses {
    pub fn fetch() -> Licenses {
        let response: Vec<License> = match ureq::get("https://api.github.com/licenses").call() {
            Ok(res) => res.into_json().unwrap(),
            Err(error) => panic!("Unable to get licenses: {}", error)
        };

        Licenses {license: response}
    }

    pub fn get_license_names(&self) -> Vec<String> {
        self.license.iter().map(|l| String::from(&l.name)).collect()
    }

    pub fn get_license_from_name(&self, name: &String) -> LicenseContent {
        let lisc = &self.license;

        let result = lisc
            .into_iter()
            .filter(|l| l.name == name.clone())
            .map(|l| l.url.clone())
            .collect();

        LicenseContent::about(&result)
    }
}

