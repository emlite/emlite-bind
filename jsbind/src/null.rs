use crate::utils::bind;

/// Only a single instance has semantic meaning—see [`Null::VALUE`].
#[derive(Clone, Debug)]
pub struct Null {
    inner: emlite::Val,
}

impl Null {
    /// Canonical singleton for the JS `null` value.
    ///
    /// Storing one constant avoids repeated calls to
    /// `emlite::Val::null()` and communicates intent at the type level.
    pub const VALUE: Null = Null {
        inner: emlite::Val::null(),
    };

    /// Always returns `true` – this wrapper is, by definition, `null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        true
    }
}

bind!(Null);
