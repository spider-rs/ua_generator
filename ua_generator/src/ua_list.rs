/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 8.0; Windows NT 6.2; Win64; x64; Trident/4.0)",
    "Mozilla/5.0 (Windows NT 6.3; rv:48.0) Gecko/20100101 Firefox/48.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2911.47 Safari/537.36",
    "Mozilla/5.0 (Android 4.4.3; Tablet; rv:47.0) Gecko/47.0 Firefox/47.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/52.0.2748.35 Safari/537.36",
    "Mozilla/5.0 (Android 5.1; Tablet; rv:50.0) Gecko/50.0 Firefox/50.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2820.34 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
