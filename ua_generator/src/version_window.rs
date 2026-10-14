//! The browser major version window every generated list is held to.
//!
//! Modern anti-bot edges reject stale browser majors outright, and the real
//! world data the lists are built from lags the release train: a sample of
//! live traffic always carries users who have not restarted their browser
//! in months. So the build keeps only the entries whose major sits within a
//! short window of the newest major seen in the fetched data, per family.
//!
//! This file is compiled twice: as `ua_generator::version_window` for
//! consumers and tests, and by `build.rs` (through `#[path]`) at list
//! generation time, so the rule that writes the lists and the rule the tests
//! check are the same code. It therefore depends on nothing but `core`.

/// A browser family the window applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    /// Chrome, Chromium and the Chromium based Edge and Opera (`Chrome/N`,
    /// `CriOS/N`).
    Chrome,
    /// Firefox on every platform (`Firefox/N`, `FxiOS/N`).
    Firefox,
    /// Safari on macOS, iOS and iPadOS (`Version/N` next to `Safari/`).
    Safari,
}

impl Family {
    /// How many majors below the newest one stay in a list.
    ///
    /// Chrome and Firefox ship a major every four weeks, so the newest three
    /// majors cover the users who are current or one update behind. Safari
    /// ships one major a year with point releases in between, so the newest
    /// two majors mean the current and previous yearly release.
    pub const fn window(self) -> u32 {
        match self {
            Family::Chrome => 2,
            Family::Firefox => 2,
            Family::Safari => 1,
        }
    }

    /// Lower case name, for logs.
    pub const fn as_str(self) -> &'static str {
        match self {
            Family::Chrome => "chrome",
            Family::Firefox => "firefox",
            Family::Safari => "safari",
        }
    }
}

/// The newest major seen per family across one fetch.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Newest {
    /// Newest Chrome major, if any Chrome entry was seen.
    pub chrome: Option<u32>,
    /// Newest Firefox major, if any Firefox entry was seen.
    pub firefox: Option<u32>,
    /// Newest Safari major, if any Safari entry was seen.
    pub safari: Option<u32>,
}

impl Newest {
    /// The newest major recorded for `family`.
    pub fn get(&self, family: Family) -> Option<u32> {
        match family {
            Family::Chrome => self.chrome,
            Family::Firefox => self.firefox,
            Family::Safari => self.safari,
        }
    }

    /// Record one sighting, keeping the larger major.
    pub fn observe(&mut self, family: Family, major: u32) {
        let slot = match family {
            Family::Chrome => &mut self.chrome,
            Family::Firefox => &mut self.firefox,
            Family::Safari => &mut self.safari,
        };
        *slot = Some(slot.map_or(major, |m| m.max(major)));
    }

    /// The newest major per family across every string in `uas`.
    pub fn from_uas<'a, I>(uas: I) -> Newest
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut newest = Newest::default();
        for ua in uas {
            if let Some((family, major)) = family_major(ua) {
                newest.observe(family, major);
            }
        }
        newest
    }
}

/// The browser family and major version a user agent string claims.
///
/// Firefox wins over everything (an `FxiOS` string also carries `Safari/`),
/// then Chrome (a `CriOS` or Edge string also carries `Safari/`, and an
/// Edge string carries `Chrome/`), then Safari, which is the one that has a
/// `Version/N` token beside `Safari/`. Anything else, such as a bot or an
/// Internet Explorer string, is `None`.
pub fn family_major(ua: &str) -> Option<(Family, u32)> {
    if let Some(major) = token_major(ua, "Firefox/").or_else(|| token_major(ua, "FxiOS/")) {
        return Some((Family::Firefox, major));
    }
    if let Some(major) = token_major(ua, "Chrome/").or_else(|| token_major(ua, "CriOS/")) {
        return Some((Family::Chrome, major));
    }
    if ua.contains("Safari/") {
        if let Some(major) = token_major(ua, "Version/") {
            return Some((Family::Safari, major));
        }
    }
    None
}

/// The leading integer right after `token` in `ua`, if present.
fn token_major(ua: &str, token: &str) -> Option<u32> {
    let start = ua.find(token)? + token.len();
    let digits = ua[start..].bytes().take_while(u8::is_ascii_digit).count();
    if digits == 0 {
        return None;
    }
    ua[start..start + digits].parse().ok()
}

/// Whether `major` is within `family`'s window below `newest`.
pub const fn within_window(family: Family, newest: u32, major: u32) -> bool {
    major <= newest && major + family.window() >= newest
}

/// Whether `ua` is current against `newest`: it parses to a known family and
/// its major is within that family's window.
pub fn is_current(ua: &str, newest: &Newest) -> bool {
    match family_major(ua) {
        Some((family, major)) => match newest.get(family) {
            Some(n) => within_window(family, n, major),
            None => false,
        },
        None => false,
    }
}

/// The entries of `items` that pass the window, never empty when `items`
/// has at least one parseable entry.
///
/// First the global window: every entry whose major is current against
/// `newest`. When that keeps nothing, the list's own newest majors are used
/// instead, so a list the data source serves stale (a mobile list a few
/// majors behind the desktop ones) still ships its freshest entries rather
/// than nothing. When nothing in the list parses at all, the list is kept
/// as it came, since there is nothing to judge it by.
pub fn filter_current<'a, S: AsRef<str>>(items: &'a [S], newest: &Newest) -> Vec<&'a S> {
    let global: Vec<&S> = items
        .iter()
        .filter(|s| is_current(s.as_ref(), newest))
        .collect();
    if !global.is_empty() {
        return global;
    }
    let local = Newest::from_uas(items.iter().map(AsRef::as_ref));
    let fallback: Vec<&S> = items
        .iter()
        .filter(|s| is_current(s.as_ref(), &local))
        .collect();
    if !fallback.is_empty() {
        return fallback;
    }
    items.iter().collect()
}

/// Whether `list` is what [`filter_current`] would have produced: every
/// entry parses, and the entries are all current against `newest` or, when
/// none is, all current against the list's own newest majors.
pub fn list_conforms<S: AsRef<str>>(list: &[S], newest: &Newest) -> bool {
    if list.is_empty() {
        return false;
    }
    if list.iter().all(|s| is_current(s.as_ref(), newest)) {
        return true;
    }
    let local = Newest::from_uas(list.iter().map(AsRef::as_ref));
    list.iter().all(|s| is_current(s.as_ref(), &local))
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIN_CHROME_154: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";
    const WIN_EDGE_153: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/153.0.0.0 Safari/537.36 Edg/153.0.0.0";
    const IOS_CHROME_131: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 18_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) CriOS/131.0.6778.73 Mobile/15E148 Safari/604.1";
    const MAC_SAFARI_26: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/26.5.2 Safari/605.1.15";
    const MAC_SAFARI_18: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.6 Safari/605.1.15";
    const LINUX_FIREFOX_155: &str =
        "Mozilla/5.0 (X11; Linux x86_64; rv:155.0) Gecko/20100101 Firefox/155.0";
    const IOS_FIREFOX_130: &str = "Mozilla/5.0 (iPhone; CPU iPhone OS 17_6 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) FxiOS/130.0 Mobile/15E148 Safari/605.1.15";
    const IE_11: &str = "Mozilla/5.0 (Windows NT 10.0; WOW64; Trident/7.0; rv:11.0) like Gecko";

    #[test]
    fn parses_family_and_major() {
        assert_eq!(family_major(WIN_CHROME_154), Some((Family::Chrome, 154)));
        assert_eq!(family_major(WIN_EDGE_153), Some((Family::Chrome, 153)));
        assert_eq!(family_major(IOS_CHROME_131), Some((Family::Chrome, 131)));
        assert_eq!(family_major(MAC_SAFARI_26), Some((Family::Safari, 26)));
        assert_eq!(
            family_major(LINUX_FIREFOX_155),
            Some((Family::Firefox, 155))
        );
        assert_eq!(family_major(IOS_FIREFOX_130), Some((Family::Firefox, 130)));
        assert_eq!(family_major(IE_11), None);
        assert_eq!(family_major(""), None);
        assert_eq!(family_major("Chrome/"), None);
    }

    #[test]
    fn window_edges() {
        assert!(within_window(Family::Chrome, 155, 155));
        assert!(within_window(Family::Chrome, 155, 153));
        assert!(!within_window(Family::Chrome, 155, 152));
        assert!(!within_window(Family::Chrome, 155, 156));
        assert!(within_window(Family::Safari, 27, 26));
        assert!(!within_window(Family::Safari, 27, 25));
        assert!(within_window(Family::Firefox, 1, 0));
    }

    #[test]
    fn newest_per_family() {
        let newest = Newest::from_uas([
            WIN_CHROME_154,
            WIN_EDGE_153,
            IOS_CHROME_131,
            MAC_SAFARI_26,
            MAC_SAFARI_18,
            LINUX_FIREFOX_155,
            IOS_FIREFOX_130,
            IE_11,
        ]);
        assert_eq!(
            newest,
            Newest {
                chrome: Some(154),
                firefox: Some(155),
                safari: Some(26)
            }
        );
    }

    #[test]
    fn filter_keeps_the_window_and_drops_the_rest() {
        let newest = Newest {
            chrome: Some(155),
            firefox: Some(157),
            safari: Some(27),
        };
        let items = [
            WIN_CHROME_154,
            WIN_EDGE_153,
            IOS_CHROME_131,
            MAC_SAFARI_26,
            MAC_SAFARI_18,
            LINUX_FIREFOX_155,
            IOS_FIREFOX_130,
            IE_11,
        ];
        let kept: Vec<&str> = filter_current(&items, &newest)
            .into_iter()
            .copied()
            .collect();
        assert_eq!(
            kept,
            vec![
                WIN_CHROME_154,
                WIN_EDGE_153,
                MAC_SAFARI_26,
                LINUX_FIREFOX_155
            ]
        );
        assert!(list_conforms(&kept, &newest));
        assert!(!list_conforms(&items, &newest));
    }

    #[test]
    fn stale_list_falls_back_to_its_own_newest() {
        let newest = Newest {
            chrome: Some(155),
            ..Default::default()
        };
        let old_129 = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Mobile Safari/537.36";
        let old_124 = "Mozilla/5.0 (Linux; Android 10; K) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Mobile Safari/537.36";
        let items = [old_124, IOS_CHROME_131, old_129];
        let kept: Vec<&str> = filter_current(&items, &newest)
            .into_iter()
            .copied()
            .collect();
        assert_eq!(kept, vec![IOS_CHROME_131, old_129]);
        assert!(list_conforms(&kept, &newest));
        assert!(!list_conforms(&items, &newest));
    }

    #[test]
    fn unparseable_list_is_kept_whole() {
        let newest = Newest {
            chrome: Some(155),
            ..Default::default()
        };
        let items = [IE_11];
        let kept: Vec<&str> = filter_current(&items, &newest)
            .into_iter()
            .copied()
            .collect();
        assert_eq!(kept, vec![IE_11]);
        assert!(!list_conforms(&items, &newest));
        assert!(!list_conforms::<&str>(&[], &newest));
    }
}
