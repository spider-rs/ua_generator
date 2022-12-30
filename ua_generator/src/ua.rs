pub use crate::ua_list::STATIC_AGENTS;
use fastrand;

/// Get a random UA from the most popular list pre-compiled generated from <https://apilayer.com/marketplace/user_agent-api#documentation-tab>.
pub fn spoof_ua() -> &'static str {
    let agent_list = STATIC_AGENTS; // list of user agents to choose from.
    let i = fastrand::usize(..agent_list.len());

    agent_list[i]
}

#[test]
/// test the spoof returns a valid user agent
fn spoof_ua_test() {
    let ua = spoof_ua();

    assert_eq!(ua.is_empty(), false);
}
