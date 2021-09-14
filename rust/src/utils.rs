use regex::Regex;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use subprocess::{Popen, PopenConfig, Redirection};

use chrono::{DateTime, Utc};
use chrono_tz::Asia::Seoul;
use chrono_tz::Tz;
pub enum ChromeType {
    GOOGLE,
    CHROMIUM,
    MSEDGE,
}

pub enum OSType {
    Windows,
    Mac,
    Linux,
}

impl OSType {
    pub fn to_string(&self) -> String {
        match self {
            OSType::Windows => "win".to_string(),
            OSType::Mac => "mac".to_string(),
            OSType::Linux => "linux".to_string(),
        }
    }
}

pub fn os_name() -> OSType {
    match env::consts::OS {
        "linux" => OSType::Linux,
        "windows" => OSType::Windows,
        "macos" => OSType::Mac,
        _ => OSType::Linux,
    }
}

pub fn os_architecture() -> String {
    let arch = env::consts::ARCH;
    if arch.ends_with("64") {
        if env::consts::OS == "windows" {
            return "32".to_string();
        }
        "64".to_string()
    } else {
        "32".to_string()
    }
}

pub fn os_type() -> String {
    let mut result: String = os_name().to_string();
    result.push_str(&os_architecture());
    result
}

pub fn chrome_version(browser_type: ChromeType) -> String {
    let re = Regex::new(r#"\d+\.\d+\.\d+"#).unwrap();

    let os = os_name();

    let cmd: Vec<&str> = match browser_type {
        ChromeType::GOOGLE => match os {
            OSType::Windows => {
                r#"reg query HKEY_CURRENT_USER\Software\Google\Chrome\BLBeacon /v version"#
            }
            OSType::Linux => r#"google-chrome --version 2>/dev/null || google-chrome-stable --version 2>/dev/null"#,
            OSType::Mac => {
                r#"/Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --version"#
            }
        },
        ChromeType::CHROMIUM => match os {
            OSType::Windows => {
                r#"reg query HKLM\SOFTWARE\Wow6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Google Chrome /v version"#
            }
            OSType::Linux => r#"chromium --version 2>/dev/null || chromium-browser --version 2>/dev/null"#,
            OSType::Mac => r#"/Applications/Chromium.app/Contents/MacOS/Chromium --version"#,
        },
        ChromeType::MSEDGE => match os {
            OSType::Windows => {
                r#"reg query HKEY_CURRENT_USER\SOFTWARE\Microsoft\Edge\BLBeacon /v version"#
            }
            OSType::Mac => {
                r#"/Applications/Microsoft\ Edge.app/Contents/MacOS/Microsoft\ Edge --version"#
            }
            _ => "None",
        },
    }.split(" ").map(|s| s).collect();

    // println!("cmd: {:?}", cmd);
    let mut proc = Popen::create(
        &cmd,
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::None,
            stderr: Redirection::None,
            ..Default::default()
        },
    )
    .unwrap();
    // Obtain the output from the standard streams.
    let (out, _) = proc.communicate(None).unwrap();
    let out = out.unwrap();
    if let Some(version) = re.find(&out) {
        version.as_str().to_string()
    } else {
        panic!(
            "Could not get version for Chrome with this command: {:?}",
            cmd
        )
    }
}

pub fn run_binary() {
    let mut proc = Popen::create(
        &[r#"E:\DEV\Code\Rust\seleniumrs\src\ajou_notice.exe"#],
        PopenConfig {
            stdout: Redirection::Pipe,
            stdin: Redirection::None,
            stderr: Redirection::None,
            ..Default::default()
        },
    )
    .unwrap();
    // Obtain the output from the standard streams.
    let (out, _) = proc.communicate(None).unwrap();
    let out = out.unwrap();
    println!("{:#}", out);
}

pub async fn get_chrome_latest_release_version() -> Result<String, reqwest::Error> {
    let version = chrome_version(ChromeType::GOOGLE);
    let url = "https://chromedriver.storage.googleapis.com/LATEST_RELEASE_".to_string() + &version;

    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}

pub async fn download_file(url: &str, path: &str) -> Result<File, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let res = client.get(url).send().await?;

    let path = Path::new(path);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    let content = res.bytes().await?;
    file.write_all(&content).unwrap();

    Ok(file)
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_version() {
        let version = chrome_version(ChromeType::GOOGLE);
        println!("Chrome version: {}", version);

        println!("OS Type: {}", os_type());

        println!(
            "Actual Release Version: {:?}",
            get_chrome_latest_release_version()
                .await
                .expect("Failed to get")
        );
    }
    // https://chromedriver.storage.googleapis.com/93.0.4577.15/chromedriver_win32.zip
    #[tokio::test]
    async fn test_download() {
        let mut latest_url = "https://chromedriver.storage.googleapis.com/".to_string();
        let file_path = format!("chromedriver_{}.zip", os_type());
        let version = get_chrome_latest_release_version()
            .await
            .expect("Failed to get");
        let file_name = format!("chromedriver_{}_{}.zip", &version, os_type());
        latest_url.push_str(&version);
        latest_url.push_str("/");
        latest_url.push_str(&file_path);

        println!("url: {}", latest_url);
        download_file(&latest_url, &file_name).await.unwrap();
    }

    #[test]
    fn test_one() {
        // run_binary();
        let seoul_now: DateTime<Tz> = Utc::now().with_timezone(&Seoul);
        println!("Seoul: {}", seoul_now);
    }
}
