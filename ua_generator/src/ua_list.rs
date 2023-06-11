/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (Windows NT 6.2; WOW64; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 5.1; WOW64; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Windows NT 6.3; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2787.24 Safari/537.36",
    "Mozilla/5.0 (Android 5.0; Tablet; rv:50.0) Gecko/50.0 Firefox/50.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:51.0) Gecko/20100101 Firefox/51.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_5) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2816.91 Safari/537.36",
    "Mozilla/5.0 (Android 4.4; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:51.0) Gecko/20100101 Firefox/51.0",
    "Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2741.24 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
