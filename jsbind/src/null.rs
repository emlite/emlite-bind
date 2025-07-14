use crate::utils::*;

/// Only a single instance has semantic meaning—see [`Null::VALUE`].
#[derive(Clone, Debug)]
#[repr(transparent)]
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

impl crate::prelude::DynCast for Null {
    #[inline]
    fn instanceof(_val: &emlite::Val) -> bool {
        false
    }
    #[inline]
    fn unchecked_from_val(v: emlite::Val) -> Self {
        v.as_::<Self>() // zero-cost new-type cast
    }
    #[inline]
    fn unchecked_from_val_ref(v: &emlite::Val) -> &Self {
        unsafe { &*(v as *const emlite::Val as *const Self) }
    }
}
