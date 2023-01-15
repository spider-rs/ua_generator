/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 5.1; WOW64; Trident/6.0)",
    "Mozilla/5.0 (Windows NT 6.2; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Windows NT 6.2) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2813.13 Safari/537.36",
    "Mozilla/5.0 (Android 5.0.2; Tablet; rv:45.0) Gecko/45.0 Firefox/45.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:48.0) Gecko/20100101 Firefox/48.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2966.48 Safari/537.36",
    "Mozilla/5.0 (Android 5.1; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686 on x86_64; rv:48.0) Gecko/20100101 Firefox/48.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/53.0.2815.20 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
