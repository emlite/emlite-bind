use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSMatrixComponentOptions {
    inner: Any,
}
impl FromVal for CSSMatrixComponentOptions {
    fn from_val(v: &Any) -> Self {
        CSSMatrixComponentOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSMatrixComponentOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSMatrixComponentOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSMatrixComponentOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSMatrixComponentOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSMatrixComponentOptions> for Any {
    fn from(s: CSSMatrixComponentOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSMatrixComponentOptions> for Any {
    fn from(s: &CSSMatrixComponentOptions) -> Any {
        s.inner.clone()
    }
}

impl CSSMatrixComponentOptions {
    pub fn is2_d(&self) -> bool {
        self.inner.get("is2D").as_::<bool>()
    }

    pub fn set_is2_d(&mut self, value: bool) {
        self.inner.set("is2D", value);
    }
}
