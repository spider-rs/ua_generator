/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:47.0) Gecko/20100101 Firefox/47.0",
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2908.21 Safari/537.36",
    "Mozilla/5.0 (Android 5.1.1; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:51.0) Gecko/20100101 Firefox/51.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2881.92 Safari/537.36",
    "Mozilla/5.0 (Android 5.0.1; Tablet; rv:46.0) Gecko/46.0 Firefox/46.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686 on x86_64; rv:48.0) Gecko/20100101 Firefox/48.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2740.65 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
