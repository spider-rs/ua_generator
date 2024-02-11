/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.1; WOW64; Trident/5.0)",
    "Mozilla/5.0 (Windows NT 6.2; WOW64; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2708.26 Safari/537.36",
    "Mozilla/5.0 (Android 5.1; Tablet; rv:51.0) Gecko/51.0 Firefox/51.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.10; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2864.17 Safari/537.36",
    "Mozilla/5.0 (Android 5.0; Tablet; rv:51.0) Gecko/51.0 Firefox/51.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2770.54 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
