use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ReadableStreamIteratorOptions {
    inner: Any,
}
impl FromVal for ReadableStreamIteratorOptions {
    fn from_val(v: &Any) -> Self {
        ReadableStreamIteratorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ReadableStreamIteratorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ReadableStreamIteratorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ReadableStreamIteratorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ReadableStreamIteratorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ReadableStreamIteratorOptions> for Any {
    fn from(s: ReadableStreamIteratorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ReadableStreamIteratorOptions> for Any {
    fn from(s: &ReadableStreamIteratorOptions) -> Any {
        s.inner.clone()
    }
}

impl ReadableStreamIteratorOptions {
    pub fn prevent_cancel(&self) -> bool {
        self.inner.get("preventCancel").as_::<bool>()
    }

    pub fn set_prevent_cancel(&mut self, value: bool) {
        self.inner.set("preventCancel", value);
    }
}
