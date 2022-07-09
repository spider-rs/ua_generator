pub use crate::ua_list::agents;

/// Get a random UA from the most popular list auto generated from [https://apilayer.com/marketplace/user_agent-api#documentation-tab].
pub fn spoof_ua() -> String {
    let agent_list = agents(); // list of user agents to choose from.

    extern "C" {
        /// C seed random.
        fn srand() -> u32;
        /// return random from seed.
        fn rand() -> u32;
    }

    // basic roll dice randomize.
    let agent = unsafe {
        srand();
        let random = rand().to_string();
        let random = random.chars().rev().nth(0).unwrap();
        let random: u32 = random.to_digit(10).unwrap();

        if random >= 9 {
            8
        } else {
            random
        }
    };

    agent_list[agent as usize].to_owned()
}

#[test]
/// test the spoof returns a valid user agent
fn spoof_ua_test() {
    let ua = spoof_ua();

    assert_eq!(ua.is_empty(), false);
}
