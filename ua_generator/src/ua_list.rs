/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (Windows NT 6.3; Win64; x64; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 5.1; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2948.71 Safari/537.36",
    "Mozilla/5.0 (Android 6.0.1; Tablet; rv:50.0) Gecko/50.0 Firefox/50.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/55.0.2888.32 Safari/537.36",
    "Mozilla/5.0 (Android 5.1; Tablet; rv:46.0) Gecko/46.0 Firefox/46.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2637.57 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
