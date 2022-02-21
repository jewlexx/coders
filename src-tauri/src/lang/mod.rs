mod languages;
use languages::LANGUAGES;
use std::ops::Deref;

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
    let mut lang: Language = Language("Text", "text");

    for (k, v) in LANGUAGES.clone().iter_mut() {
        if k.is_match(extension.as_ref()) {
            lang = *v;
            break;
        }
    }

    Some(lang)
}
