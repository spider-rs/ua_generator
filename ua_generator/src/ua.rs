pub use crate::chrome_linux_ua_list::STATIC_CHROME_LINUX_AGENTS;
pub use crate::chrome_mac_ua_list::STATIC_CHROME_MAC_AGENTS;
pub use crate::chrome_mobile_ua_list::STATIC_CHROME_MOBILE_AGENTS;
pub use crate::chrome_tablet_ua_list::STATIC_CHROME_TABLET_AGENTS;
pub use crate::chrome_ua_list::STATIC_CHROME_AGENTS;
pub use crate::chrome_windows_ua_list::STATIC_CHROME_WINDOWS_AGENTS;
pub use crate::firefox_linux_ua_list::STATIC_FIREFOX_LINUX_AGENTS;
pub use crate::firefox_mac_ua_list::STATIC_FIREFOX_MAC_AGENTS;
pub use crate::firefox_mobile_ua_list::STATIC_FIREFOX_MOBILE_AGENTS;
pub use crate::firefox_tablet_ua_list::STATIC_FIREFOX_TABLET_AGENTS;
pub use crate::firefox_ua_list::STATIC_FIREFOX_AGENTS;
pub use crate::firefox_windows_ua_list::STATIC_FIREFOX_WINDOWS_AGENTS;
pub use crate::safari_mac_ua_list::STATIC_SAFARI_MAC_AGENTS;
pub use crate::safari_mobile_ua_list::STATIC_SAFARI_MOBILE_AGENTS;
pub use crate::safari_tablet_ua_list::STATIC_SAFARI_TABLET_AGENTS;
pub use crate::safari_ua_list::STATIC_SAFARI_AGENTS;
pub use crate::ua_list::STATIC_AGENTS;

use fastrand::{self, Rng};
use std::collections::HashMap;
use std::rc::Rc;

/// Operating systems (mirrors build.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Os {
    /// Windows
    Windows,
    /// Mac
    Mac,
    /// Linux
    Linux,
    /// Android
    Android,
}

/// Device / form factor (mirrors build.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormFactor {
    /// Desktop
    Desktop,
    /// Mobile
    Mobile,
    /// Tablet
    Tablet,
}

/// Browser families (mirrors build.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Browser {
    /// Chrome
    Chrome,
    /// Firefox
    Firefox,
    /// IE
    Ie,
    /// Safari
    Safari,
}

/// Get a random UA from a static precompiled list.
pub fn spoof_ua() -> &'static str {
    pick_rand(None, STATIC_AGENTS)
}

/// Get a random chrome UA from a static precompiled list.
pub fn spoof_chrome_ua() -> &'static str {
    pick_rand(None, STATIC_CHROME_AGENTS)
}

/// Get a random chrome mac UA from a static precompiled list.
pub fn spoof_chrome_mac_ua() -> &'static str {
    pick_rand(None, STATIC_CHROME_MAC_AGENTS)
}

/// Get a random chrome linux UA from a static precompiled list.
pub fn spoof_chrome_linux_ua() -> &'static str {
    pick_rand(None, STATIC_CHROME_LINUX_AGENTS)
}

/// Get a random chrome mobile UA from a static precompiled list.
pub fn spoof_chrome_mobile_ua() -> &'static str {
    pick_rand(None, STATIC_CHROME_MOBILE_AGENTS)
}

/// Get a random chrome tablet UA from a static precompiled list.
pub fn spoof_chrome_tablet_ua() -> &'static str {
    pick_rand(None, STATIC_CHROME_TABLET_AGENTS)
}

/// Get a random UA from a static precompiled list.
pub fn spoof_ua_with_randomizer(thread_rng: &mut Rng) -> &'static str {
    pick_rand(Some(thread_rng), STATIC_AGENTS)
}

/// Get a random chrome UA from a static precompiled list.
pub fn spoof_chrome_ua_with_randomizer(thread_rng: &mut Rng) -> &'static str {
    pick_rand(Some(thread_rng), STATIC_CHROME_AGENTS)
}

/// Get a random chrome mac UA from a static precompiled list.
pub fn spoof_chrome_mac_ua_with_randomizer(thread_rng: &mut Rng) -> &'static str {
    pick_rand(Some(thread_rng), STATIC_CHROME_MAC_AGENTS)
}

/// Get a random chrome linux UA from a static precompiled list.
pub fn spoof_chrome_linux_ua_with_randomizer(thread_rng: &mut Rng) -> &'static str {
    pick_rand(Some(thread_rng), STATIC_CHROME_LINUX_AGENTS)
}

/// Slices for each generated family (zero-copy, no alloc).
#[inline]
pub fn chrome_agents() -> &'static [&'static str] {
    STATIC_CHROME_AGENTS
}

#[inline]
/// Chrome windows agents.
pub fn chrome_windows_agents() -> &'static [&'static str] {
    STATIC_CHROME_WINDOWS_AGENTS
}

#[inline]
/// Chrome mac agents.
pub fn chrome_mac_agents() -> &'static [&'static str] {
    STATIC_CHROME_MAC_AGENTS
}

#[inline]
/// Chrome linux agents.
pub fn chrome_linux_agents() -> &'static [&'static str] {
    STATIC_CHROME_LINUX_AGENTS
}

#[inline]
/// Chrome mobile agents.
pub fn chrome_mobile_agents() -> &'static [&'static str] {
    STATIC_CHROME_MOBILE_AGENTS
}

#[inline]
/// Chrome tablet agents.
pub fn chrome_tablet_agents() -> &'static [&'static str] {
    STATIC_CHROME_TABLET_AGENTS
}

/// Get a random firefox UA from a static precompiled list.
pub fn spoof_firefox_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_AGENTS)
}

/// Get a random firefox windows UA.
pub fn spoof_firefox_windows_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_WINDOWS_AGENTS)
}

/// Get a random firefox mac UA.
pub fn spoof_firefox_mac_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_MAC_AGENTS)
}

/// Get a random firefox linux UA.
pub fn spoof_firefox_linux_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_LINUX_AGENTS)
}

/// Get a random firefox mobile UA.
pub fn spoof_firefox_mobile_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_MOBILE_AGENTS)
}

/// Get a random firefox tablet UA.
pub fn spoof_firefox_tablet_ua() -> &'static str {
    pick_rand(None, STATIC_FIREFOX_TABLET_AGENTS)
}

/// Random Firefox UA using the provided RNG (deterministic with seeded RNG).
pub fn spoof_firefox_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_AGENTS)
}

/// Random Firefox Windows (desktop) UA using the provided RNG.
pub fn spoof_firefox_windows_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_WINDOWS_AGENTS)
}

/// Random Firefox macOS (desktop) UA using the provided RNG.
pub fn spoof_firefox_mac_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_MAC_AGENTS)
}

/// Random Firefox Linux (desktop) UA using the provided RNG.
pub fn spoof_firefox_linux_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_LINUX_AGENTS)
}

/// Random Firefox Mobile UA using the provided RNG.
pub fn spoof_firefox_mobile_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_MOBILE_AGENTS)
}

/// Random Firefox Tablet UA using the provided RNG.
pub fn spoof_firefox_tablet_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_FIREFOX_TABLET_AGENTS)
}

#[inline]
/// All Firefox UAs (mixed OS/form factors).
pub fn firefox_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_AGENTS
}

#[inline]
/// Firefox Windows desktop UAs.
pub fn firefox_windows_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_WINDOWS_AGENTS
}

#[inline]
/// Firefox macOS desktop UAs.
pub fn firefox_mac_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_MAC_AGENTS
}

#[inline]
/// Firefox Linux desktop UAs.
pub fn firefox_linux_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_LINUX_AGENTS
}

#[inline]
/// Firefox Mobile UAs.
pub fn firefox_mobile_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_MOBILE_AGENTS
}

#[inline]
/// Firefox Tablet UAs.
pub fn firefox_tablet_agents() -> &'static [&'static str] {
    STATIC_FIREFOX_TABLET_AGENTS
}

/// Random Safari UA.
pub fn spoof_safari_ua() -> &'static str {
    pick_rand(None, STATIC_SAFARI_AGENTS)
}
/// Random Safari macOS (desktop) UA.
pub fn spoof_safari_mac_ua() -> &'static str {
    pick_rand(None, STATIC_SAFARI_MAC_AGENTS)
}
/// Random Safari Mobile (iOS) UA.
pub fn spoof_safari_mobile_ua() -> &'static str {
    pick_rand(None, STATIC_SAFARI_MOBILE_AGENTS)
}
/// Random Safari Tablet (iPadOS) UA.
pub fn spoof_safari_tablet_ua() -> &'static str {
    pick_rand(None, STATIC_SAFARI_TABLET_AGENTS)
}

/// Random Safari UA using the provided RNG.
pub fn spoof_safari_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_SAFARI_AGENTS)
}

/// Random Safari Mac UA using the provided RNG.
pub fn spoof_safari_mac_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_SAFARI_MAC_AGENTS)
}

/// Random Safari Mobile UA using the provided RNG.
pub fn spoof_safari_mobile_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_SAFARI_MOBILE_AGENTS)
}

/// Random Safari Tablet UA using the provided RNG.
pub fn spoof_safari_tablet_ua_with_randomizer(rng: &mut Rng) -> &'static str {
    pick_rand(Some(rng), STATIC_SAFARI_TABLET_AGENTS)
}

#[inline]
/// All Safari UAs (mixed OS/form factors).
pub fn safari_agents() -> &'static [&'static str] {
    STATIC_SAFARI_AGENTS
}

#[inline]
/// All Safari Mac UAs (mixed OS/form factors).
pub fn safari_mac_agents() -> &'static [&'static str] {
    STATIC_SAFARI_MAC_AGENTS
}

#[inline]
/// All Safari Mobile UAs (mixed OS/form factors).
pub fn safari_mobile_agents() -> &'static [&'static str] {
    STATIC_SAFARI_MOBILE_AGENTS
}

#[inline]
/// All Safari Tablet UAs (mixed OS/form factors).
pub fn safari_tablet_agents() -> &'static [&'static str] {
    STATIC_SAFARI_TABLET_AGENTS
}

#[inline]
/// Mixed static agents.
pub fn mixed_static_agents() -> &'static [&'static str] {
    STATIC_AGENTS
}

/// Generic chooser using the typed enums used in build.rs.
/// If a precise list for (os, form, browser) isn’t generated at build time,
/// we fall back in this order: OS+Chrome desktop list → all Chrome list → mixed static list.
pub fn spoof_by(
    os: Option<Os>,
    form: Option<FormFactor>,
    browser: Option<Browser>,
    rng: Option<&mut Rng>,
) -> &'static str {
    match (os, form, browser) {
        // --- Chrome  ---
        (Some(Os::Windows), Some(FormFactor::Desktop), Some(Browser::Chrome)) => {
            pick_rand(rng, STATIC_CHROME_WINDOWS_AGENTS)
        }
        (Some(Os::Mac), Some(FormFactor::Desktop), Some(Browser::Chrome)) => {
            pick_rand(rng, STATIC_CHROME_MAC_AGENTS)
        }
        (Some(Os::Linux), Some(FormFactor::Desktop), Some(Browser::Chrome)) => {
            pick_rand(rng, STATIC_CHROME_LINUX_AGENTS)
        }
        (_, Some(FormFactor::Mobile), Some(Browser::Chrome)) => {
            pick_rand(rng, STATIC_CHROME_MOBILE_AGENTS)
        }
        (_, Some(FormFactor::Tablet), Some(Browser::Chrome)) => {
            pick_rand(rng, STATIC_CHROME_TABLET_AGENTS)
        }
        (_, _, Some(Browser::Chrome)) => pick_rand(rng, STATIC_CHROME_AGENTS),

        // --- Firefox ---
        (Some(Os::Windows), Some(FormFactor::Desktop), Some(Browser::Firefox)) => {
            pick_rand(rng, STATIC_FIREFOX_WINDOWS_AGENTS)
        }
        (Some(Os::Mac), Some(FormFactor::Desktop), Some(Browser::Firefox)) => {
            pick_rand(rng, STATIC_FIREFOX_MAC_AGENTS)
        }
        (Some(Os::Linux), Some(FormFactor::Desktop), Some(Browser::Firefox)) => {
            pick_rand(rng, STATIC_FIREFOX_LINUX_AGENTS)
        }
        (_, Some(FormFactor::Mobile), Some(Browser::Firefox)) => {
            pick_rand(rng, STATIC_FIREFOX_MOBILE_AGENTS)
        }
        (_, Some(FormFactor::Tablet), Some(Browser::Firefox)) => {
            pick_rand(rng, STATIC_FIREFOX_TABLET_AGENTS)
        }
        (_, _, Some(Browser::Firefox)) => pick_rand(rng, STATIC_FIREFOX_AGENTS),

        // --- Safari ---
        // Desktop macOS
        (Some(Os::Mac), Some(FormFactor::Desktop), Some(Browser::Safari)) => {
            pick_rand(rng, STATIC_SAFARI_MAC_AGENTS)
        }
        // Mobile / Tablet (cross-OS lists since Safari mobile = iOS family)
        (_, Some(FormFactor::Mobile), Some(Browser::Safari)) => {
            pick_rand(rng, STATIC_SAFARI_MOBILE_AGENTS)
        }
        (_, Some(FormFactor::Tablet), Some(Browser::Safari)) => {
            pick_rand(rng, STATIC_SAFARI_TABLET_AGENTS)
        }
        // Any Safari fallback → global Safari list
        (_, _, Some(Browser::Safari)) => pick_rand(rng, STATIC_SAFARI_AGENTS),

        // --- FormFactor match only ---
        (_, Some(FormFactor::Desktop), _) => pick_rand_multi(
            rng,
            &[
                STATIC_CHROME_WINDOWS_AGENTS,
                STATIC_CHROME_MAC_AGENTS,
                STATIC_CHROME_LINUX_AGENTS,
                STATIC_FIREFOX_WINDOWS_AGENTS,
                STATIC_FIREFOX_MAC_AGENTS,
                STATIC_FIREFOX_LINUX_AGENTS,
                STATIC_SAFARI_MAC_AGENTS,
            ],
        ),
        (_, Some(FormFactor::Mobile), _) => pick_rand_multi(
            rng,
            &[
                STATIC_CHROME_MOBILE_AGENTS,
                STATIC_FIREFOX_MOBILE_AGENTS,
                STATIC_SAFARI_MOBILE_AGENTS,
            ],
        ),
        (_, Some(FormFactor::Tablet), _) => pick_rand_multi(
            rng,
            &[
                STATIC_CHROME_TABLET_AGENTS,
                STATIC_FIREFOX_TABLET_AGENTS,
                STATIC_SAFARI_TABLET_AGENTS,
            ],
        ),

        // --- Fall Back ---
        // IE: until IE lists are generated, fall back to mixed static
        _ => pick_rand(rng, STATIC_AGENTS),
    }
}

/* -------------------------
All-agents aggregate
------------------------- */

/// Returns a combined vector containing all user-agents from various predefined static categories.
pub fn all_static_agents() -> &'static Vec<&'static str> {
    static AGENTS: std::sync::OnceLock<Vec<&'static str>> = std::sync::OnceLock::new();
    AGENTS.get_or_init(|| {
        STATIC_AGENTS
            .iter()
            // Chrome groups
            .chain(STATIC_CHROME_AGENTS.iter())
            .chain(STATIC_CHROME_MAC_AGENTS.iter())
            .chain(STATIC_CHROME_LINUX_AGENTS.iter())
            .chain(STATIC_CHROME_MOBILE_AGENTS.iter())
            .chain(STATIC_CHROME_TABLET_AGENTS.iter())
            // Firefox groups
            .chain(STATIC_FIREFOX_AGENTS.iter())
            .chain(STATIC_FIREFOX_WINDOWS_AGENTS.iter())
            .chain(STATIC_FIREFOX_MAC_AGENTS.iter())
            .chain(STATIC_FIREFOX_LINUX_AGENTS.iter())
            .chain(STATIC_FIREFOX_MOBILE_AGENTS.iter())
            .chain(STATIC_FIREFOX_TABLET_AGENTS.iter())
            // Safari groups
            .chain(STATIC_SAFARI_AGENTS.iter())
            .chain(STATIC_SAFARI_MAC_AGENTS.iter())
            .chain(STATIC_SAFARI_MOBILE_AGENTS.iter())
            .chain(STATIC_SAFARI_TABLET_AGENTS.iter())
            .copied()
            .collect()
    })
}

/// Returns a random user-agent from all predefined static categories.
/// Generally you do not want to use this unless you can handle any agent.
pub fn spoof_random_agent(thread_rng: &mut Rng) -> &'static str {
    let agents = all_static_agents();
    agents[thread_rng.usize(..agents.len())]
}

/* -------------------------
Dynamic list container
------------------------- */

/// Structure to manage a dynamic list of User-Agents, with quick lookup capabilities.
#[derive(Default, Clone)]
pub struct UserAgents {
    /// A list of User-Agents used at runtime.
    list: Vec<Rc<String>>,
    /// Map from User-Agent strings to their positions in `list`.
    list_map: HashMap<Rc<String>, usize>,
}

impl UserAgents {
    /// Creates a new `UserAgents` from a list.
    pub fn new<I>(iter: I) -> UserAgents
    where
        I: IntoIterator<Item = String>,
    {
        let mut list = Vec::new();
        let mut list_map = HashMap::new();

        for (i, item) in iter.into_iter().enumerate() {
            let rc_item = Rc::new(item);
            list.push(Rc::clone(&rc_item));
            list_map.insert(rc_item, i);
        }

        UserAgents { list, list_map }
    }

    /// Gets a reference to the list of user agents.
    pub fn get_agent_list(&self) -> &Vec<Rc<String>> {
        &self.list
    }

    /// Gets a reference to the map of user agents and their positions.
    pub fn get_agent_map(&self) -> &HashMap<Rc<String>, usize> {
        &self.list_map
    }

    /// Adds a new user agent by name.
    pub fn add_agent(&mut self, agent: String) {
        let rc_agent = Rc::new(agent);
        if !self.list_map.contains_key(&rc_agent) {
            let index = self.list.len();
            self.list.push(Rc::clone(&rc_agent));
            self.list_map.insert(rc_agent, index);
        }
    }

    /// Removes a user agent from the list by its name.
    pub fn remove_agent(&mut self, agent_name: &str) {
        let key_to_remove = self.list_map.iter().find_map(|(key, _)| {
            if key.as_str() == agent_name {
                Some(key.clone())
            } else {
                None
            }
        });

        if let Some(key) = key_to_remove {
            if let Some(&index) = self.list_map.get(&key) {
                self.list.remove(index);
                self.list_map.remove(&key);
                for value in self.list_map.values_mut() {
                    if *value > index {
                        *value -= 1;
                    }
                }
            }
        }
    }

    /// Returns a random user agent from the dynamic list (falls back to static).
    pub fn spoof(&self) -> &str {
        if self.list.is_empty() {
            spoof_ua()
        } else {
            &self.list[fastrand::usize(..self.list.len())]
        }
    }

    /// Returns a random user agent from the dynamic list (falls back to static).
    pub fn spoof_with_randomizer(&self, thread_rng: &mut Rng) -> &str {
        if self.list.is_empty() {
            spoof_ua()
        } else {
            &self.list[thread_rng.usize(..self.list.len())]
        }
    }
}

/* -------------------------
Internal helpers
------------------------- */

#[inline]
fn pick_rand<'a>(rng: Option<&mut Rng>, candidates: &'a [&'a str]) -> &'a str {
    if candidates.is_empty() {
        return "";
    }
    match rng {
        Some(r) => candidates[r.usize(..candidates.len())],
        None => candidates[fastrand::usize(..candidates.len())],
    }
}

/// Pick randomly across multiple candidate slices without allocating.
#[inline]
fn pick_rand_multi<'a>(rng: Option<&mut Rng>, groups: &[&'a [&'a str]]) -> &'a str {
    let total: usize = groups.iter().map(|g| g.len()).sum();
    if total == 0 {
        return "";
    }

    let mut idx = match rng {
        Some(r) => r.usize(..total),
        None => fastrand::usize(..total),
    };

    for g in groups {
        if idx < g.len() {
            return g[idx];
        }
        idx -= g.len();
    }

    // unreachable if `total` computed correctly
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Test that `spoof_ua` returns a non-empty string.
    fn test_spoof_ua() {
        let ua = spoof_ua();
        assert!(!ua.is_empty());
    }

    #[test]
    /// Test creating `UserAgents` and obtaining a user agent successfully.
    fn test_user_agents() {
        let agents = vec![
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".to_string(),
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0.3 Safari/605.1.15".to_string(),
        ];
        let ua_instance = UserAgents::new(agents.clone());
        let random_ua = ua_instance.spoof();
        assert!(agents.contains(&random_ua.to_string()));
    }

    #[test]
    /// Test add and remove of a user agent.
    fn test_add_and_remove_user_agent() {
        let mut ua_instance = UserAgents::new(vec!["Agent1".to_string(), "Agent2".to_string()]);

        assert_eq!(ua_instance.list.len(), 2);
        assert_eq!(ua_instance.list_map.len(), 2);

        ua_instance.add_agent("Agent3".to_string());
        assert_eq!(ua_instance.list.len(), 3);
        assert_eq!(ua_instance.list_map.len(), 3);

        let agent3_rc = ua_instance
            .get_agent_list()
            .iter()
            .find(|rc| rc.as_str() == "Agent3")
            .expect("Agent3 should be in the list.");

        assert!(
            Rc::ptr_eq(
                agent3_rc,
                ua_instance
                    .list_map
                    .iter()
                    .find_map(|(rc, _)| if **rc == "Agent3" { Some(rc) } else { None })
                    .unwrap()
            ),
            "Agent3 should point to the same Rc instance in both map and list."
        );

        ua_instance.remove_agent("Agent1");
        assert_eq!(ua_instance.list.len(), 2);
        assert_eq!(ua_instance.list_map.len(), 2);
        assert!(!ua_instance
            .list_map
            .contains_key(&Rc::new("Agent1".to_string())));

        assert!(
            !ua_instance
                .get_agent_list()
                .iter()
                .any(|rc| rc.as_str() == "Agent1"),
            "Agent1 should be removed from the list."
        );

        assert_eq!(
            ua_instance.list_map.get(&Rc::new("Agent3".to_string())),
            Some(&1)
        );
    }

    #[test]
    fn test_spoof_by_fallbacks() {
        // Should pull from the per-OS desktop lists when available
        let _ = spoof_by(
            Some(Os::Windows),
            Some(FormFactor::Desktop),
            Some(Browser::Chrome),
            None,
        );
        let _ = spoof_by(
            Some(Os::Mac),
            Some(FormFactor::Desktop),
            Some(Browser::Chrome),
            None,
        );
        let _ = spoof_by(
            Some(Os::Linux),
            Some(FormFactor::Desktop),
            Some(Browser::Chrome),
            None,
        );

        // Mobile/tablet lists exist cross-OS
        let _ = spoof_by(None, Some(FormFactor::Mobile), Some(Browser::Chrome), None);
        let _ = spoof_by(None, Some(FormFactor::Tablet), Some(Browser::Chrome), None);

        // Firefox/IE fall back to STATIC_AGENTS unless you add specific lists later
        let _ = spoof_by(
            Some(Os::Windows),
            Some(FormFactor::Desktop),
            Some(Browser::Firefox),
            None,
        );
        let _ = spoof_by(
            Some(Os::Windows),
            Some(FormFactor::Desktop),
            Some(Browser::Ie),
            None,
        );

        // FormFactor-only
        let _ = spoof_by(None, Some(FormFactor::Desktop), None, None);
        let _ = spoof_by(None, Some(FormFactor::Mobile), None, None);
        let _ = spoof_by(None, Some(FormFactor::Tablet), None, None);
    }
}
