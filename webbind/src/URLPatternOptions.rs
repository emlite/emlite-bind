use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct URLPatternOptions {
    inner: Any,
}
impl FromVal for URLPatternOptions {
    fn from_val(v: &Any) -> Self {
        URLPatternOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for URLPatternOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for URLPatternOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for URLPatternOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for URLPatternOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<URLPatternOptions> for Any {
    fn from(s: URLPatternOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&URLPatternOptions> for Any {
    fn from(s: &URLPatternOptions) -> Any {
        s.inner.clone()
    }
}

impl URLPatternOptions {
    pub fn ignore_case(&self) -> bool {
        self.inner.get("ignoreCase").as_::<bool>()
    }

    pub fn set_ignore_case(&mut self, value: bool) {
        self.inner.set("ignoreCase", value);
    }
}
