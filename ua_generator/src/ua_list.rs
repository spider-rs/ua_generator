/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (compatible; MSIE 10.0; Windows NT 10.0; Win64; x64; Trident/6.0)",
    "Mozilla/5.0 (Windows NT 6.2; WOW64; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Windows NT 6.3; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2844.41 Safari/537.36",
    "Mozilla/5.0 (Android 5.0.1; Tablet; rv:46.0) Gecko/46.0 Firefox/46.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.11; rv:46.0) Gecko/20100101 Firefox/46.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_11_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2644.90 Safari/537.36",
    "Mozilla/5.0 (Android 4.4.3; Tablet; rv:47.0) Gecko/47.0 Firefox/47.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2638.85 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
