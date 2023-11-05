/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 9.0; Windows NT 6.3; WOW64; Trident/5.0)",
    "Mozilla/5.0 (Windows NT 5.1; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2957.59 Safari/537.36",
    "Mozilla/5.0 (Android 4.4.2; Tablet; rv:48.0) Gecko/48.0 Firefox/48.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2843.90 Safari/537.36",
    "Mozilla/5.0 (Android 4.4; Tablet; rv:50.0) Gecko/50.0 Firefox/50.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:47.0) Gecko/20100101 Firefox/47.0",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2724.61 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
