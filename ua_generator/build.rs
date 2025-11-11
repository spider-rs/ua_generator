#![cfg_attr(not(feature = "fetch"), allow(unused))]

#[cfg(feature = "fetch")]
mod ua_build {
    use std::borrow::Cow;
    use std::fmt::Write as _;
    use std::fs;
    use std::path::{Path, PathBuf};

    use dotenvy::{dotenv, var};
    use serde::{Deserialize, Serialize};
    use toml::Value as TomlValue;
    use ureq::Agent;

    // =========================
    // Typed surface (all params)
    // =========================

    /// Operating systems supported by the remote API.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Os {
        Windows,
        Mac,
        Linux,
        Android,
    }

    /// Device / form factor flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FormFactor {
        Desktop,
        Mobile,
        Tablet,
    }

    /// Browser families supported by the remote API.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Browser {
        Chrome,
        Firefox,
        Ie,
        Safari,
    }

    /// Request mode: single item, full list, or N samples.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum FetchMode {
        Single,
        List,
        Sampled(usize),
    }

    /// Fully typed UA query covering *all* boolean parameters your API expects.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TypedQuery {
        pub os: Option<Os>,
        pub form_factor: Option<FormFactor>,
        pub browser: Option<Browser>,
        pub mode: FetchMode,
    }

    impl TypedQuery {
        pub fn new(
            os: Option<Os>,
            form_factor: Option<FormFactor>,
            browser: Option<Browser>,
            mode: FetchMode,
        ) -> Self {
            Self {
                os,
                form_factor,
                browser,
                mode,
            }
        }

        /// Convert to key/value boolean pairs (matching your existing query keys).
        pub fn to_pairs(&self) -> Vec<(&'static str, &'static str)> {
            let mut pairs = Vec::with_capacity(12);

            let (windows, mac, linux, android_os) = match self.os {
                Some(Os::Windows) => (true, false, false, false),
                Some(Os::Mac) => (false, true, false, false),
                Some(Os::Linux) => (false, false, true, false),
                Some(Os::Android) => (false, false, false, true),
                None => (false, false, false, false),
            };

            let (desktop, mobile, tablet) = match self.form_factor {
                Some(FormFactor::Desktop) => (true, false, false),
                Some(FormFactor::Mobile) => (false, true, false),
                Some(FormFactor::Tablet) => (false, false, true),
                None => (false, false, false),
            };

            let (ie, firefox, chrome, safari) = match self.browser {
                Some(Browser::Ie) => (true, false, false, false),
                Some(Browser::Firefox) => (false, true, false, false),
                Some(Browser::Chrome) => (false, false, true, false),
                Some(Browser::Safari) => (false, false, false, true),
                None => (false, false, false, false),
            };

            let list_flag = matches!(self.mode, FetchMode::List);

            pairs.push(("windows", yes_no(windows)));
            pairs.push(("tablet", yes_no(tablet)));
            pairs.push(("mobile", yes_no(mobile)));
            pairs.push(("mac", yes_no(mac)));
            pairs.push(("linux", yes_no(linux)));
            pairs.push(("ie", yes_no(ie)));
            pairs.push(("firefox", yes_no(firefox)));
            pairs.push(("desktop", yes_no(desktop)));
            pairs.push(("chrome", yes_no(chrome)));
            pairs.push(("safari", yes_no(safari)));
            pairs.push(("android", yes_no(android_os)));
            pairs.push(("list", yes_no(list_flag)));

            pairs
        }

        /// `?k=v&...`
        pub fn to_query_string(&self) -> String {
            let mut s = String::from("?");
            for (i, (k, v)) in self.to_pairs().into_iter().enumerate() {
                if i > 0 {
                    s.push('&');
                }
                s.push_str(k);
                s.push('=');
                s.push_str(v);
            }
            s
        }
    }

    const fn yes_no(b: bool) -> &'static str {
        if b {
            "true"
        } else {
            "false"
        }
    }

    /// Where to write and what const name to export (mirrors your current files).
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct OutputSpec {
        pub rel_src_path: &'static str,
        pub const_name: &'static str,
        pub doc: &'static str,
    }

    /// A build “job”: what to fetch and where to write it.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct BuildGroup {
        pub query: TypedQuery,
        pub output: OutputSpec,
    }

    // =========================
    // Output specs (all files)
    // =========================

    pub mod outputs {
        use super::OutputSpec;

        pub const STATIC_AGENTS: OutputSpec = OutputSpec {
            rel_src_path: "ua_list.rs",
            const_name: "STATIC_AGENTS",
            doc: "static list of agents pre-built",
        };

        pub const CHROME_WINDOWS: OutputSpec = OutputSpec {
            rel_src_path: "chrome_windows_ua_list.rs",
            const_name: "STATIC_CHROME_WINDOWS_AGENTS",
            doc: "static list of agents pre-built windows",
        };

        pub const CHROME_MAC: OutputSpec = OutputSpec {
            rel_src_path: "chrome_mac_ua_list.rs",
            const_name: "STATIC_CHROME_MAC_AGENTS",
            doc: "static list of agents pre-built mac",
        };

        pub const CHROME_LINUX: OutputSpec = OutputSpec {
            rel_src_path: "chrome_linux_ua_list.rs",
            const_name: "STATIC_CHROME_LINUX_AGENTS",
            doc: "static list of agents pre-built linux",
        };

        pub const CHROME_ALL: OutputSpec = OutputSpec {
            rel_src_path: "chrome_ua_list.rs",
            const_name: "STATIC_CHROME_AGENTS",
            doc: "List of real Chrome User-Agents.",
        };

        pub const CHROME_MOBILE: OutputSpec = OutputSpec {
            rel_src_path: "chrome_mobile_ua_list.rs",
            const_name: "STATIC_CHROME_MOBILE_AGENTS",
            doc: "List of real chrome mobile User-Agents.",
        };

        pub const CHROME_TABLET: OutputSpec = OutputSpec {
            rel_src_path: "chrome_tablet_ua_list.rs",
            const_name: "STATIC_CHROME_TABLET_AGENTS",
            doc: "List of real chrome tablet User-Agents.",
        };

        pub const FIREFOX_ALL: OutputSpec = OutputSpec {
            rel_src_path: "firefox_ua_list.rs",
            const_name: "STATIC_FIREFOX_AGENTS",
            doc: "List of real Firefox User-Agents.",
        };

        pub const FIREFOX_WINDOWS: OutputSpec = OutputSpec {
            rel_src_path: "firefox_windows_ua_list.rs",
            const_name: "STATIC_FIREFOX_WINDOWS_AGENTS",
            doc: "static list of firefox agents pre-built windows",
        };

        pub const FIREFOX_MAC: OutputSpec = OutputSpec {
            rel_src_path: "firefox_mac_ua_list.rs",
            const_name: "STATIC_FIREFOX_MAC_AGENTS",
            doc: "static list of firefox agents pre-built mac",
        };

        pub const FIREFOX_LINUX: OutputSpec = OutputSpec {
            rel_src_path: "firefox_linux_ua_list.rs",
            const_name: "STATIC_FIREFOX_LINUX_AGENTS",
            doc: "static list of firefox agents pre-built linux",
        };

        pub const FIREFOX_MOBILE: OutputSpec = OutputSpec {
            rel_src_path: "firefox_mobile_ua_list.rs",
            const_name: "STATIC_FIREFOX_MOBILE_AGENTS",
            doc: "List of real firefox mobile User-Agents.",
        };

        pub const FIREFOX_TABLET: OutputSpec = OutputSpec {
            rel_src_path: "firefox_tablet_ua_list.rs",
            const_name: "STATIC_FIREFOX_TABLET_AGENTS",
            doc: "List of real firefox tablet User-Agents.",
        };

        pub const SAFARI_ALL: OutputSpec = OutputSpec {
            rel_src_path: "safari_ua_list.rs",
            const_name: "STATIC_SAFARI_AGENTS",
            doc: "List of real Safari User-Agents.",
        };

        pub const SAFARI_MAC: OutputSpec = OutputSpec {
            rel_src_path: "safari_mac_ua_list.rs",
            const_name: "STATIC_SAFARI_MAC_AGENTS",
            doc: "static list of safari agents pre-built mac (desktop)",
        };

        pub const SAFARI_MOBILE: OutputSpec = OutputSpec {
            rel_src_path: "safari_mobile_ua_list.rs",
            const_name: "STATIC_SAFARI_MOBILE_AGENTS",
            doc: "List of real Safari mobile User-Agents (iOS).",
        };

        pub const SAFARI_TABLET: OutputSpec = OutputSpec {
            rel_src_path: "safari_tablet_ua_list.rs",
            const_name: "STATIC_SAFARI_TABLET_AGENTS",
            doc: "List of real Safari tablet User-Agents (iPadOS).",
        };
    }

    // =======================================
    // Named groups matching what you generate
    // =======================================

    pub mod groups {
        use super::outputs::*;
        use super::*;

        /// Your 11-entry mixed STATIC_AGENTS recipe as typed queries.
        pub fn static_agents_recipe() -> Vec<TypedQuery> {
            vec![
                // --- Windows (desktop)
                TypedQuery::new(
                    Some(Os::Windows),
                    Some(FormFactor::Desktop),
                    Some(Browser::Ie),
                    FetchMode::Single,
                ),
                TypedQuery::new(
                    Some(Os::Windows),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(2),
                ),
                TypedQuery::new(
                    Some(Os::Windows),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(2),
                ),
                // --- macOS (desktop)
                TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Safari),
                    FetchMode::Sampled(2),
                ),
                TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(2),
                ),
                TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(1),
                ),
                // --- Linux (desktop)
                TypedQuery::new(
                    Some(Os::Linux),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(2),
                ),
                TypedQuery::new(
                    Some(Os::Linux),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(2),
                ),
                // --- Android (mobile)
                TypedQuery::new(
                    Some(Os::Android),
                    Some(FormFactor::Mobile),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(2),
                ),
                TypedQuery::new(
                    Some(Os::Android),
                    Some(FormFactor::Mobile),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(1),
                ),
                // --- Android (tablet) – useful for tablet profiles that aren’t iPadOS
                TypedQuery::new(
                    Some(Os::Android),
                    Some(FormFactor::Tablet),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(1),
                ),
                // --- iOS / iPadOS Safari (cross-OS lists on API; we key off form factor)
                TypedQuery::new(
                    None,
                    Some(FormFactor::Mobile),
                    Some(Browser::Safari),
                    FetchMode::Sampled(2),
                ), // iPhone
                TypedQuery::new(
                    None,
                    Some(FormFactor::Tablet),
                    Some(Browser::Safari),
                    FetchMode::Sampled(2),
                ), // iPad
            ]
        }

        // Firefox desktop x9 (sampled like Chrome)
        pub fn firefox_windows() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Windows),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(9),
                ),
                output: outputs::FIREFOX_WINDOWS,
            }
        }
        pub fn firefox_mac() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(9),
                ),
                output: outputs::FIREFOX_MAC,
            }
        }
        pub fn firefox_linux() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Linux),
                    Some(FormFactor::Desktop),
                    Some(Browser::Firefox),
                    FetchMode::Sampled(9),
                ),
                output: outputs::FIREFOX_LINUX,
            }
        }

        // Firefox lists via list=true (all / mobile / tablet)
        pub fn firefox_all_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(None, None, Some(Browser::Firefox), FetchMode::List),
                output: outputs::FIREFOX_ALL,
            }
        }
        pub fn firefox_mobile_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Mobile),
                    Some(Browser::Firefox),
                    FetchMode::List,
                ),
                output: outputs::FIREFOX_MOBILE,
            }
        }
        pub fn firefox_tablet_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Tablet),
                    Some(Browser::Firefox),
                    FetchMode::List,
                ),
                output: outputs::FIREFOX_TABLET,
            }
        }

        /// Chrome Windows x9.
        pub fn chrome_windows() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Windows),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(9),
                ),
                output: CHROME_WINDOWS,
            }
        }

        /// Chrome Mac x9.
        pub fn chrome_mac() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(9),
                ),
                output: CHROME_MAC,
            }
        }

        /// Chrome Linux x9.
        pub fn chrome_linux() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Linux),
                    Some(FormFactor::Desktop),
                    Some(Browser::Chrome),
                    FetchMode::Sampled(9),
                ),
                output: CHROME_LINUX,
            }
        }

        pub fn safari_mac() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    Some(Os::Mac),
                    Some(FormFactor::Desktop),
                    Some(Browser::Safari),
                    FetchMode::Sampled(9), // desktop Mac x9
                ),
                output: outputs::SAFARI_MAC,
            }
        }

        pub fn safari_all_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(None, None, Some(Browser::Safari), FetchMode::List),
                output: outputs::SAFARI_ALL,
            }
        }

        pub fn safari_mobile_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Mobile),
                    Some(Browser::Safari),
                    FetchMode::List,
                ),
                output: outputs::SAFARI_MOBILE,
            }
        }

        pub fn safari_tablet_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Tablet),
                    Some(Browser::Safari),
                    FetchMode::List,
                ),
                output: outputs::SAFARI_TABLET,
            }
        }

        /// Chrome (all) with list=true.
        pub fn chrome_all_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(None, None, Some(Browser::Chrome), FetchMode::List),
                output: outputs::CHROME_ALL,
            }
        }

        /// Chrome mobile with list=true.
        pub fn chrome_mobile_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Mobile),
                    Some(Browser::Chrome),
                    FetchMode::List,
                ),
                output: outputs::CHROME_MOBILE,
            }
        }

        /// Chrome tablet with list=true.
        pub fn chrome_tablet_list() -> BuildGroup {
            BuildGroup {
                query: TypedQuery::new(
                    None,
                    Some(FormFactor::Tablet),
                    Some(Browser::Chrome),
                    FetchMode::List,
                ),
                output: outputs::CHROME_TABLET,
            }
        }
    }

    // ======================
    // API + writer utilities
    // ======================

    #[derive(Deserialize, Serialize, Debug, Default)]
    struct ApiResult {
        agent: String,
    }

    struct ApiClient {
        base_api: String,
        token: String,
        http: Agent,
    }

    impl ApiClient {
        fn new(base_api: impl Into<String>, token: impl Into<String>) -> Self {
            Self {
                base_api: base_api.into(),
                token: token.into(),
                http: ureq::builder().build(),
            }
        }

        fn url(&self, q: &TypedQuery) -> String {
            format!("{}{}", self.base_api, q.to_query_string())
        }

        fn get_one(&self, q: &TypedQuery) -> Result<String, String> {
            let url = self.url(q);
            self.http
                .get(&url)
                .set("apikey", &self.token)
                .set("user-agent", "spider-rs")
                .call()
                .map_err(|e| format!("request failed: {url} => {e:?}"))?
                .into_json::<ApiResult>()
                .map_err(|e| format!("parse failed for {url}: {e}"))
                .map(|r| r.agent)
        }

        fn get_list(&self, q: &TypedQuery) -> Result<Vec<String>, String> {
            let url = self.url(q);
            self.http
                .get(&url)
                .set("apikey", &self.token)
                .set("user-agent", "spider-rs")
                .call()
                .map_err(|e| format!("request failed: {url} => {e:?}"))?
                .into_json::<Vec<ApiResult>>()
                .map_err(|e| format!("parse failed for {url}: {e}"))
                .map(|v| v.into_iter().map(|r| r.agent).collect())
        }

        fn get_n(&self, q: &TypedQuery, n: usize) -> Result<Vec<String>, String> {
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                out.push(self.get_one(q)?);
            }
            Ok(out)
        }
    }

    fn fetch_by_query(client: &ApiClient, q: &TypedQuery) -> Result<Vec<String>, String> {
        match q.mode {
            FetchMode::Single => client.get_one(q).map(|s| vec![s]),
            FetchMode::List => client.get_list(q),
            FetchMode::Sampled(n) => client.get_n(q, n),
        }
    }

    fn write_const_str_slice(
        out_path: impl AsRef<Path>,
        const_name: &str,
        items: &[String],
        doc: &str,
    ) -> Result<(), String> {
        let mut buf = String::new();

        if !doc.trim().is_empty() {
            let _ = writeln!(buf, "/// {}", doc.trim());
        }
        let _ = writeln!(buf, "pub const {}: &[&str] = &[", const_name);
        for item in items {
            let escaped = escape_str(item);
            let _ = writeln!(buf, "    \"{escaped}\",");
        }
        let _ = writeln!(buf, "];");

        fs::write(out_path, buf).map_err(|e| format!("write const file failed: {e}"))
    }

    fn escape_str(s: &str) -> Cow<'_, str> {
        if s.contains('\\') || s.contains('"') {
            let mut out = String::with_capacity(s.len() + 8);
            for ch in s.chars() {
                match ch {
                    '\\' => out.push_str("\\\\"),
                    '"' => out.push_str("\\\""),
                    _ => out.push(ch),
                }
            }
            Cow::Owned(out)
        } else {
            Cow::Borrowed(s)
        }
    }

    fn src(path: &str) -> PathBuf {
        Path::new("./src").join(path)
    }

    // =====================
    // Cargo.toml versioning
    // =====================
    fn manifest_path_from_env() -> PathBuf {
        if let Ok(p) = std::env::var("BUMP_MANIFEST") {
            return PathBuf::from(p);
        }
        let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());
        Path::new(&dir).join("Cargo.toml")
    }

    /// Bump the minor version.
    fn bump_minor_version() {
        // 1) Open crate manifest
        let crate_manifest = manifest_path_from_env();
        let Ok(text) = fs::read_to_string(&crate_manifest) else {
            eprintln!("warn: failed to read Cargo.toml for version bump");
            return;
        };

        let Ok(mut doc): Result<TomlValue, _> = text.parse() else {
            eprintln!("warn: failed to parse Cargo.toml");
            return;
        };

        let Some(pkg) = doc.get_mut("package") else {
            eprintln!("warn: missing [package] in Cargo.toml");
            return;
        };

        let Some(ver_val) = pkg.get_mut("version") else {
            eprintln!("warn: missing package.version in Cargo.toml");
            return;
        };

        let Some(ver_str) = ver_val.as_str() else {
            eprintln!("warn: package.version is not a string");
            return;
        };

        let new_ver = increment_minor(ver_str);
        *ver_val = TomlValue::String(new_ver.clone());

        if let Err(e) = fs::write(&crate_manifest, toml::to_string(&doc).unwrap_or(text)) {
            eprintln!("warn: failed writing bumped Cargo.toml: {e}");
        } else {
            println!("Bumped version to: {new_ver}");
        }
    }

    fn increment_minor(version: &str) -> String {
        let mut parts: Vec<String> = version.split('.').map(|f| f.to_string()).collect();
        if parts.len() < 2 {
            return "0.1.0".to_string();
        }
        if let Ok(minor) = parts[1].parse::<u64>() {
            parts[1] = (minor + 1).to_string();
            if parts.len() >= 3 {
                parts[2] = "0".to_string();
            } else {
                parts.push("0".to_string());
            }
            parts.join(".")
        } else {
            version.to_string()
        }
    }

    // =========
    // Entrypoint
    // =========

    pub fn run() -> Result<(), String> {
        dotenv().ok();

        // Keep your envs and defaults identical.
        let build_enabled = var("BUILD_ENABLED")
            .map(|v| matches!(v.as_str(), "1" | "true" | "TRUE"))
            .unwrap_or(false);

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-env-changed=BUILD_ENABLED");
        println!("cargo:rerun-if-env-changed=API_URL");
        println!("cargo:rerun-if-env-changed=APILAYER_KEY");

        if !build_enabled {
            return Ok(());
        }

        let base_api = var("API_URL")
            .unwrap_or_else(|_| "https://api.spider.cloud/data/user_agents".to_string());

        let token =
            var("APILAYER_KEY").map_err(|_| "empty APILAYER_KEY token for the API.".to_string())?;

        let client = ApiClient::new(base_api, token);

        use groups::*;

        // ---- STATIC_AGENTS (exactly 11 items) ----
        let mut static_items: Vec<String> = Vec::new();
        for q in static_agents_recipe() {
            let mut chunk = fetch_by_query(&client, &q)?;
            static_items.append(&mut chunk);
        }

        {
            let spec = outputs::STATIC_AGENTS;
            write_const_str_slice(
                src(spec.rel_src_path),
                spec.const_name,
                &static_items,
                spec.doc,
            )?;
        }

        // ---- Chrome desktop x9 per-OS ----
        for grp in [
            chrome_windows(),
            chrome_mac(),
            chrome_linux(),
            groups::firefox_windows(),
            groups::firefox_mac(),
            groups::firefox_linux(),
            groups::safari_mac(),
        ] {
            let items = fetch_by_query(&client, &grp.query)?;
            write_const_str_slice(
                src(grp.output.rel_src_path),
                grp.output.const_name,
                &items,
                grp.output.doc,
            )?;
        }

        // ---- Chrome lists: all / mobile / tablet ----
        for grp in [
            chrome_all_list(),
            chrome_mobile_list(),
            chrome_tablet_list(),
            groups::firefox_all_list(),
            groups::firefox_mobile_list(),
            groups::firefox_tablet_list(),
            groups::safari_all_list(),
            groups::safari_mobile_list(),
            groups::safari_tablet_list(),
        ] {
            let items = fetch_by_query(&client, &grp.query)?;
            write_const_str_slice(
                src(grp.output.rel_src_path),
                grp.output.const_name,
                &items,
                grp.output.doc,
            )?;
            // mirror your original behavior: bump after list files
            bump_minor_version();
        }

        Ok(())
    }
}

#[cfg(feature = "fetch")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = ua_build::run() {
        eprintln!("error: {e}");
        return Err(Box::<dyn std::error::Error + Send + Sync>::from(e));
    }
    Ok(())
}

#[cfg(not(feature = "fetch"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
