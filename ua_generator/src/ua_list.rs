/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 10.0; Win64; x64; Trident/6.0)",
    "Mozilla/5.0 (Windows NT 6.3; WOW64; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2880.13 Safari/537.36",
    "Mozilla/5.0 (Android 5.1.1; Tablet; rv:48.0) Gecko/48.0 Firefox/48.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:51.0) Gecko/20100101 Firefox/51.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2921.17 Safari/537.36",
    "Mozilla/5.0 (Android 5.0.1; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686 on x86_64; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.58 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
