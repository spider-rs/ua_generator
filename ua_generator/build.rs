extern crate serde;
extern crate ureq;

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use ureq::get;

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ApiResult {
    /// user agent string id
    ua: String,
}

/// get the agent type for version os
pub fn get_agent(url: &str, token: &String) -> String {
    let agent: ApiResult = get(&url)
        .set("apikey", token)
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    agent.ua
}

/// build entry for setting required agents
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build_enabled = env::var("BUILD_ENABLED").map(|v| v == "1").unwrap_or(false);

    if build_enabled {
        // fetch the latest ua and parse to files.
        let token: String = match env::var("APILAYER_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("You need a valid https://apilayer.com/ API key to gather agents!");

                "".to_string()
            }
        }
        .to_string();

        let base_api = "https://api.apilayer.com/user_agent/generate";

        // windows
        let windows_ie_desktop_agent = format!("{base_api}?windows=true&tablet=true&mobile=true&mac=false&linux=false&ie=true&firefox=false&desktop=true&chrome=false&android=false");
        let windows_firefox_desktop_agent = format!("{base_api}?windows=true&tablet=true&mobile=true&mac=false&linux=false&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let windows_chrome_desktop_agent = format!("{base_api}?windows=true&tablet=true&mobile=true&mac=false&linux=false&ie=false&firefox=false&desktop=true&chrome=true&android=false");
        // mac
        let mac_firefox_desktop_agent = format!("{base_api}?windows=false&tablet=true&mobile=true&mac=true&linux=false&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let mac_chrome_desktop_agent = format!("{base_api}?windows=false&tablet=true&mobile=true&mac=true&linux=false&ie=false&firefox=false&desktop=true&chrome=true&android=false");
        // linux
        let linux_firefox_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=true&ie=false&firefox=true&desktop=true&chrome=false&android=false");
        let linux_chrome_desktop_agent = format!("{base_api}?windows=false&tablet=false&mobile=false&mac=false&linux=true&ie=false&firefox=false&desktop=true&chrome=true&android=false");
        // android
        let android_firefox_agent = format!("{base_api}?windows=false&tablet=true&mobile=false&mac=false&linux=false&ie=true&firefox=true&desktop=true&chrome=false&android=true");
        let android_chrome_agent = format!("{base_api}?windows=false&tablet=true&mobile=false&mac=false&linux=false&ie=true&firefox=true&desktop=true&chrome=false&android=true");

        // windows agents
        let windows_ie_desktop_agent: String = get_agent(&windows_ie_desktop_agent, &token);
        let windows_firefox_desktop_agent: String =
            get_agent(&windows_firefox_desktop_agent, &token);
        let windows_chrome_desktop_agent: String = get_agent(&windows_chrome_desktop_agent, &token);
        // mac agents
        let mac_firefox_desktop_agent: String = get_agent(&mac_firefox_desktop_agent, &token);
        let mac_chrome_desktop_agent: String = get_agent(&mac_chrome_desktop_agent, &token);
        // linux
        let linux_firefox_desktop_agent: String = get_agent(&linux_firefox_desktop_agent, &token);
        let linux_chrome_desktop_agent: String = get_agent(&linux_chrome_desktop_agent, &token);
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
            mac_chrome_desktop_agent,
            android_chrome_agent,
            linux_firefox_desktop_agent,
            linux_chrome_desktop_agent,
        );

        fs::write(&dest_path, agents).unwrap();
        println!("cargo:rerun-if-changed=build.rs");
    }

    Ok(())
}
