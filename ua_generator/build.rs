
#[cfg(feature = "fetch")]
use std::fs;
#[cfg(feature = "fetch")]
use std::path::Path;

#[cfg(feature = "fetch")]
#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct ApiResult {
    /// user agent string id
    agent: String,
}

#[cfg(feature = "fetch")]
/// bump the minor
fn increment_version(version: &str) -> String {
    let mut parts: Vec<String> = version.split('.').map(String::from).collect(); // Collect into Vec<String> to handle owned strings
    if let Some(last) = parts.last_mut() {
        if let Ok(num) = last.parse::<u32>() {
            *last = (num + 1).to_string(); // Create a new owned String and assign that
        }
    }
    parts.join(".")
}

#[cfg(feature = "fetch")]
fn bump_version_in_cargo_toml() -> Result<(), Box<dyn std::error::Error>> {
    use serde_json::Value;
    use std::io::Read;
    let cargo_toml_path = Path::new("Cargo.toml");

    let mut cargo_toml_content = String::new();

    crate::fs::OpenOptions::new()
        .read(true)
        .open(&cargo_toml_path)?
        .read_to_string(&mut cargo_toml_content)?;

    if !cargo_toml_content.is_empty() {
        let mut parsed_toml: Value = cargo_toml_content.parse()?;

        if let Some(version) = parsed_toml
            .get_mut("package")
            .and_then(|pkg| pkg.get_mut("version"))
        {
            if let Some(version_str) = version.as_str() {
                let new_version = increment_version(version_str);
                *version = Value::String(new_version.clone());
                println!("Bumped version to: {}", new_version);
            }
        }

        let new_cargo_toml_content = toml::to_string(&parsed_toml)?;
        fs::write(cargo_toml_path, new_cargo_toml_content)?;
    }

    Ok(())
}

/// get the agent type for version os
#[cfg(feature = "fetch")]
pub fn get_agent(url: &str, token: &String) -> String {
    use ureq::get;

    match get(&url)
        .set("apikey", token)
        .set("user-agent", "spider-rs")
        .call()
    {
        Ok(req) => {
            let req: ApiResult = req
                .into_json()
                .expect("Authorization not granted! Make sure to set a valid API key.");

            req.agent
        }
        Err(e) => {
            panic!("{:?}. Please check your API key", e)
        }
    }
}

#[cfg(not(feature = "fetch"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

/// build entry for setting required agents
#[cfg(feature = "fetch")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use dotenvy::{dotenv, var};
    use ureq::get;

    dotenv().ok();

    let build_enabled = var("BUILD_ENABLED")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    if build_enabled {
        let base_api = var("API_URL").unwrap_or("https://api.spider.cloud/data/user_agents".into());
        // fetch the latest ua and parse to files.
        let token: String = match var("APILAYER_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("You need a valid {} API key to gather agents!", base_api);
                "".to_string()
            }
        }
        .to_string();

        if token.is_empty() {
            panic!("empty APILAYER_KEY token for the API.");
        }

        // windows
        let windows_ie_desktop_agent = format!("{base_api}?windows=true&tablet=false&mobile=false&mac=false&linux=false&ie=true&firefox=false&desktop=true&chrome=false&android=false");
        let windows_firefox_desktop_agent = format!("{base_api}?windows=true&tablet=false&mobile=false&mac=false&linux=false&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let windows_chrome_desktop_agent = format!("{base_api}?windows=true&tablet=false&mobile=false&mac=false&linux=false&ie=false&firefox=false&desktop=true&chrome=true&android=false");
        // mac
        let mac_firefox_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=true&linux=false&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let mac_chrome_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=true&linux=false&ie=false&firefox=false&desktop=true&chrome=true&android=false");

        // linux
        let linux_firefox_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=true&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let linux_chrome_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=true&ie=false&firefox=false&desktop=true&chrome=true&android=false");
        // android
        let android_firefox_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=false&ie=true&firefox=true&desktop=true&chrome=false&android=true");
        let android_chrome_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=false&ie=true&firefox=true&desktop=true&chrome=false&android=true");

        // windows agents
        let windows_ie_desktop_agent: String = get_agent(&windows_ie_desktop_agent, &token);
        let windows_firefox_desktop_agent: String =
            get_agent(&windows_firefox_desktop_agent, &token);
        let windows_chrome_desktop_agent: String = get_agent(&windows_chrome_desktop_agent, &token);
        // mac agents
        let mac_firefox_desktop_agent: String = get_agent(&mac_firefox_desktop_agent, &token);

        // mac chrome
        let mac_chrome_desktop_agent1: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent2: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent3: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent4: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent5: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent6: String = get_agent(&mac_chrome_desktop_agent, &token);
        let mac_chrome_desktop_agent7: String = get_agent(&mac_chrome_desktop_agent, &token);

        // linux
        let linux_firefox_desktop_agent: String = get_agent(&linux_firefox_desktop_agent, &token);

        let linux_chrome_desktop_agent1: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent2: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent3: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent4: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent5: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent6: String = get_agent(&linux_chrome_desktop_agent, &token);
        let linux_chrome_desktop_agent7: String = get_agent(&linux_chrome_desktop_agent, &token);

        // android agents
        let android_firefox_agent: String = get_agent(&android_firefox_agent, &token);
        let android_chrome_agent: String = get_agent(&android_chrome_agent, &token);

        let dest_path = Path::new(&"./src").join("ua_list.rs");

        let agents = format!(
            r#"/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {{
    STATIC_AGENTS.to_owned()
}}
"#,
            windows_ie_desktop_agent,
            windows_firefox_desktop_agent,
            windows_chrome_desktop_agent,
            android_firefox_agent,
            mac_firefox_desktop_agent,
            mac_chrome_desktop_agent1,
            android_chrome_agent,
            linux_firefox_desktop_agent,
            linux_chrome_desktop_agent1,
        );

        fs::write(&dest_path, agents).unwrap();

        let dest_path = Path::new(&"./src").join("chrome_mac_ua_list.rs");

        let agents = format!(
            r#"/// static list of agents pre-built
pub const STATIC_CHROME_MAC_AGENTS: &'static [&'static str; 9] = &[
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}"
];  

/// chrome mac user agent list
pub fn chrome_mac_agents() -> [&'static str; 9] {{
    STATIC_CHROME_MAC_AGENTS.to_owned()
}}
"#,
            mac_chrome_desktop_agent1,
            mac_chrome_desktop_agent2,
            mac_chrome_desktop_agent3,
            mac_chrome_desktop_agent4,
            mac_chrome_desktop_agent5,
            mac_chrome_desktop_agent6,
            mac_chrome_desktop_agent7,
            mac_chrome_desktop_agent1,
            mac_chrome_desktop_agent4
        );

        fs::write(&dest_path, agents).unwrap();

        let dest_path = Path::new(&"./src").join("chrome_linux_ua_list.rs");

        let agents = format!(
            r#"/// static list of agents pre-built
pub const STATIC_CHROME_LINUX_AGENTS: &'static [&'static str; 9] = &[
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}",
    "{}"
];  

/// chrome linux user agent list
pub fn chrome_linux_agents() -> [&'static str; 9] {{
    STATIC_CHROME_LINUX_AGENTS.to_owned()
}}
"#,
            linux_chrome_desktop_agent1,
            linux_chrome_desktop_agent2,
            linux_chrome_desktop_agent3,
            linux_chrome_desktop_agent4,
            linux_chrome_desktop_agent5,
            linux_chrome_desktop_agent6,
            linux_chrome_desktop_agent7,
            linux_chrome_desktop_agent1,
            linux_chrome_desktop_agent4
        );

        fs::write(&dest_path, agents).unwrap();

        // Build a list of valid chrome user-agents to use for chrome only browsers.
        let chrome_agent_list: Vec<ApiResult> =
            match get(&format!("{base_api}?chrome=true&list=true"))
                .set("apikey", &token)
                .set("user-agent", "spider-rs")
                .call()
            {
                Ok(req) => {
                    let req: Vec<ApiResult> = req
                        .into_json()
                        .expect("Authorization not granted! Make sure to set a valid API key.");
                    req
                }
                Err(e) => {
                    panic!("{:?}", e)
                }
            };
        let dest_path = Path::new(&"./src").join("chrome_ua_list.rs");
        let mut chrome_devices = format!(
            r#"/// List of real Chrome User-Agents.
pub const STATIC_CHROME_AGENTS: &'static [&'static str; {}] = &[
"#,
            chrome_agent_list.len()
        );

        for device in chrome_agent_list {
            chrome_devices.push_str(&format!("    \"{}\",\n", device.agent));
        }

        chrome_devices.push_str("];");

        if let Ok(_) = fs::write(dest_path, chrome_devices) {
            let _ = bump_version_in_cargo_toml();
        }

        // Build a list of valid chrome user-agents to use for chrome only browsers.
        let chrome_agent_list: Vec<ApiResult> = match get(&format!(
            "{base_api}?chrome=true&desktop=false&tablet=false&mobile=true&list=true"
        ))
        .set("apikey", &token)
        .set("user-agent", "spider-rs")
        .call()
        {
            Ok(req) => {
                let req: Vec<ApiResult> = req
                    .into_json()
                    .expect("Authorization not granted! Make sure to set a valid API key.");
                req
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        };
        let dest_path = Path::new(&"./src").join("chrome_mobile_ua_list.rs");
        let mut chrome_devices = format!(
            r#"/// List of real chrome mobile User-Agents.
pub const STATIC_CHROME_MOBILE_AGENTS: &'static [&'static str; {}] = &[
"#,
            chrome_agent_list.len()
        );

        for device in chrome_agent_list {
            chrome_devices.push_str(&format!("    \"{}\",\n", device.agent));
        }

        chrome_devices.push_str("];");

        if let Ok(_) = fs::write(dest_path, chrome_devices) {
            let _ = bump_version_in_cargo_toml();
        }

        // Build a list of valid chrome user-agents to use for chrome only browsers.
        let chrome_agent_list: Vec<ApiResult> =
            match get(&format!("{base_api}?chrome=true&list=true"))
                .set("apikey", &token)
                .set("user-agent", "spider-rs")
                .call()
            {
                Ok(req) => {
                    let req: Vec<ApiResult> = req
                        .into_json()
                        .expect("Authorization not granted! Make sure to set a valid API key.");
                    req
                }
                Err(e) => {
                    panic!("{:?}", e)
                }
            };
        let dest_path = Path::new(&"./src").join("chrome_ua_list.rs");
        let mut chrome_devices = format!(
            r#"/// List of real Chrome User-Agents.
pub const STATIC_CHROME_AGENTS: &'static [&'static str; {}] = &[
"#,
            chrome_agent_list.len()
        );

        for device in chrome_agent_list {
            chrome_devices.push_str(&format!("    \"{}\",\n", device.agent));
        }

        chrome_devices.push_str("];");

        if let Ok(_) = fs::write(dest_path, chrome_devices) {
            let _ = bump_version_in_cargo_toml();
        }

        // Build a list of valid chrome user-agents to use for chrome only browsers.
        let chrome_agent_list: Vec<ApiResult> = match get(&format!(
            "{base_api}?chrome=true&desktop=false&tablet=true&mobile=false&list=true"
        ))
        .set("apikey", &token)
        .set("user-agent", "spider-rs")
        .call()
        {
            Ok(req) => {
                let req: Vec<ApiResult> = req
                    .into_json()
                    .expect("Authorization not granted! Make sure to set a valid API key.");
                req
            }
            Err(e) => {
                panic!("{:?}", e)
            }
        };
        let dest_path = Path::new(&"./src").join("chrome_tablet_ua_list.rs");
        let mut chrome_devices = format!(
            r#"/// List of real chrome tablet User-Agents.
pub const STATIC_CHROME_TABLET_AGENTS: &'static [&'static str; {}] = &[
"#,
            chrome_agent_list.len()
        );

        for device in chrome_agent_list {
            chrome_devices.push_str(&format!("    \"{}\",\n", device.agent));
        }

        chrome_devices.push_str("];");

        if let Ok(_) = fs::write(dest_path, chrome_devices) {
            let _ = bump_version_in_cargo_toml();
        }

        println!("cargo:rerun-if-changed=build.rs");
    }

    Ok(())
}
