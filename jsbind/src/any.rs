/// Any is just a wrapper around `emlite::Val`
pub use emlite::Val as Any;

impl crate::prelude::DynCast for Any {
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
