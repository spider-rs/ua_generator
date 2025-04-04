pub use crate::chrome_linux_ua_list::STATIC_CHROME_LINUX_AGENTS;
pub use crate::chrome_mac_ua_list::STATIC_CHROME_MAC_AGENTS;
pub use crate::chrome_ua_list::STATIC_CHROME_AGENTS;

pub use crate::ua_list::STATIC_AGENTS;

use fastrand;
use std::collections::HashMap;
use std::rc::Rc;

/// Get a random UA from a static precompiled list.
pub fn spoof_ua() -> &'static str {
    STATIC_AGENTS[fastrand::usize(..STATIC_AGENTS.len())]
}

/// Get a random chrome UA from a static precompiled list.
pub fn spoof_chrome_ua() -> &'static str {
    STATIC_CHROME_AGENTS[fastrand::usize(..STATIC_CHROME_AGENTS.len())]
}

/// Get a random chrome mac UA from a static precompiled list.
pub fn spoof_chrome_mac_ua() -> &'static str {
    STATIC_CHROME_MAC_AGENTS[fastrand::usize(..STATIC_CHROME_MAC_AGENTS.len())]
}

/// Get a random chrome linux UA from a static precompiled list.
pub fn spoof_chrome_linux_ua() -> &'static str {
    STATIC_CHROME_LINUX_AGENTS[fastrand::usize(..STATIC_CHROME_LINUX_AGENTS.len())]
}

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

    /// Returns a random user agent from the dynamic list.
    pub fn spoof(&self) -> &str {
        if self.list.is_empty() {
            spoof_ua()
        } else {
            &self.list[fastrand::usize(..self.list.len())]
        }
    }
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
                &ua_instance
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
}
