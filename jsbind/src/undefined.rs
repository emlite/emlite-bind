use crate::utils::bind;

/// The struct merely carries the underlying `emlite::Val` so it can be
/// passed through JS APIs; there is only one meaningful instance,
/// exposed as [`Undefined::VALUE`].
#[derive(Clone)]
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
