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

type Licenses = Vec<License>;

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
    pub fn fetch(url: &String) -> LicenseContent {
        let license: LicenseContent = match ureq::get(&url).call() {
            Ok(res) => res.into_json().unwrap(),
            Err(error) => panic!("Unable to fetch license content: {}", error),
        };

        license
    }
}

impl From<&String> for LicenseContent {
    fn from (url: &String) -> LicenseContent {
        LicenseContent::fetch(&url)
    }

}

impl From<&License> for LicenseContent {
    fn from (license: &License) -> LicenseContent {
        LicenseContent::fetch(&license.url)
    }

}

impl License {
    // fetch all licenses from github
    pub fn fetch() -> Licenses {
        match ureq::get("https://api.github.com/licenses").call() {
            Ok(res) => res.into_json().unwrap(),
            Err(error) => panic!("Unable to fetch licenses: {}", error),
        }
    }
}
