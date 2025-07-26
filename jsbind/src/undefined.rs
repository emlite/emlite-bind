use crate::utils::*;

/// The struct merely carries the underlying `emlite::Val` so it can be
/// passed through JS APIs; there is only one meaningful instance,
/// exposed as [`Undefined::VALUE`].
#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct Undefined {
    inner: emlite::Val,
}

impl Undefined {
    /// The canonical `undefined` value.
    ///
    /// Because `undefined` has no runtime state, storing a single
    /// constant is sufficient and avoids repeated `Val::undefined()`
    /// calls.
    pub const VALUE: Undefined = Undefined {
        inner: emlite::Val::undefined(),
    };

    /// Always returns `false` — `undefined` is never `null`.
    #[inline]
    pub fn is_null(&self) -> bool {
        false
    }

    /// Always returns `true`.
    #[inline]
    pub fn is_undefined(&self) -> bool {
        true
    }
}

bind!(Undefined);

impl crate::prelude::DynCast for Undefined {
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
    #[inline]
    fn unchecked_from_val_mut(v: &mut emlite::Val) -> &mut Self {
        unsafe { &mut *(v as *mut emlite::Val as *mut Self) }
    }
}
