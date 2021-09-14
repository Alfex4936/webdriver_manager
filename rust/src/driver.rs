use crate::utils::ChromeType;

trait Driver {
    fn get_name(&self) -> &String;
    fn get_url(&self) -> &String;
    fn get_version(&self) -> &String;
    fn get_latest_release_version(&self) -> &String;
}

struct ChromeDriver {
    name: String,
    url: String,
    version: String,
    latest_release_url: String,

    chrome_type: ChromeType,
    browser_version: String,
}

impl Driver for ChromeDriver {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_url(&self) -> &String {
        &self.url
    }
    fn get_version(&self) -> &String {
        &self.version
    }
    fn get_latest_release_version(&self) -> &String {
        &self.latest_release_url
    }
}
