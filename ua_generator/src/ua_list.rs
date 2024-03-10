/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 8.0; Windows NT 10.0; WOW64; Trident/4.0)",
    "Mozilla/5.0 (Windows NT 6.2; Win64; x64; rv:48.0) Gecko/20100101 Firefox/48.0",
    "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2702.70 Safari/537.36",
    "Mozilla/5.0 (Android 5.0; Tablet; rv:51.0) Gecko/51.0 Firefox/51.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2706.68 Safari/537.36",
    "Mozilla/5.0 (Android 4.4.4; Tablet; rv:45.0) Gecko/45.0 Firefox/45.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:49.0) Gecko/20100101 Firefox/49.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2624.66 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
