
/// Get a random UA from the most popular list auto generated from [https://techblog.willshouse.com/2012/01/03/most-common-user-agents/].
pub fn spoof_ua() -> String {
    let moz_base = "Mozilla/5.0";
    let agent_xshared = format!("{} (Windows NT 10.0; Win64; x64", moz_base);
    let chrome_version = |v| {
        format!("{}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{} Safari/537.36", agent_xshared, v)
    };
    let chrome_100 = "100.0.4896.127";
    let chrome_101 = "101.0.4951.54";
    let firefox_100 = format!("{}; rv:100.0) Gecko/20100101 Firefox/100.0", agent_xshared);
    let safari_100 = format!("{} (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/15.4 Safari/605.1.15", moz_base);
    let agent_list = vec![chrome_version(chrome_100), chrome_version(chrome_101), firefox_100, safari_100]; // list of user agents to choose from.

    let mut agent = 0;

    extern "C" {
        /// C seed random.
        fn srand() -> u32;
        /// return random from seed.
        fn rand() -> u32;
    }
    
    // basic roll dice randomize.
    unsafe {
        srand();
        let random = rand().to_string();
        let random = random.chars().rev().nth(0).unwrap();
        let random: u32 = random.to_digit(10).unwrap();

        if random > 8 {
            agent = 3;
        } else if random > 6 {
            agent = 2;
        } else if random > 3 {
            agent = 1;
        };
    }

    agent_list[agent].to_owned()
}


#[test]
fn spoof_ua_test() {
    let ua = spoof_ua();
    
    assert_eq!(
        ua.is_empty(),
        false
    );
}