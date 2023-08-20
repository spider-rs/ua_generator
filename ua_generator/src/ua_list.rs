/// static list of agents pre-built
pub const STATIC_AGENTS: &'static [&'static str; 9] = &[
    "Mozilla/5.0 (Windows NT 6.2; Trident/7.0; rv:11.0) like Gecko",
    "Mozilla/5.0 (Windows NT 6.2; Win64; x64; rv:45.0) Gecko/20100101 Firefox/45.0",
    "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/50.0.2696.86 Safari/537.36",
    "Mozilla/5.0 (Android 5.1; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.9; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_8_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/49.0.2659.46 Safari/537.36",
    "Mozilla/5.0 (Android 4.4.2; Tablet; rv:49.0) Gecko/49.0 Firefox/49.0",
    "Mozilla/5.0 (X11; Linux i686 on x86_64; rv:50.0) Gecko/20100101 Firefox/50.0",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/54.0.2856.22 Safari/537.36"
];  

/// user agent list
pub fn agents() -> [&'static str; 9] {
    STATIC_AGENTS.to_owned()
}
