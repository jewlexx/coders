#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![warn(clippy::all)]

use std::ops::Deref;

mod languages;

use languages::LANGUAGES;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
pub struct Language<'a>(pub &'a str, pub &'a str);

impl<'a> Language<'a> {
    #[inline]
    pub fn name(&self) -> &'a str {
        self.0
    }

    #[inline]
    pub fn id(&self) -> &'a str {
        self.1
    }
}

impl<'a> Deref for Language<'a> {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.id()
    }
}

#[inline]
pub fn from_extension<S: AsRef<str>>(extension: S) -> Option<Language<'static>> {
    from_lowercase_extension(extension.as_ref().to_ascii_lowercase())
}

#[inline]
pub fn from_lowercase_extension<S: AsRef<str>>(extension: S) -> Option<Language<'static>> {
    LANGUAGES
        .binary_search_by_key(&extension.as_ref(), |&(ext, _)| ext)
        .ok()
        .map(|i| LANGUAGES[i].1)
}
