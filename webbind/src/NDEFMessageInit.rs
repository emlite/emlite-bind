use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFMessageInit {
    inner: Any,
}
impl FromVal for NDEFMessageInit {
    fn from_val(v: &Any) -> Self {
        NDEFMessageInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFMessageInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFMessageInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NDEFMessageInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NDEFMessageInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NDEFMessageInit> for Any {
    fn from(s: NDEFMessageInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NDEFMessageInit> for Any {
    fn from(s: &NDEFMessageInit) -> Any {
        s.inner.clone()
    }
}

impl NDEFMessageInit {
    pub fn records(&self) -> TypedArray<NDEFRecordInit> {
        self.inner
            .get("records")
            .as_::<TypedArray<NDEFRecordInit>>()
    }

    pub fn set_records(&mut self, value: &TypedArray<NDEFRecordInit>) {
        self.inner.set("records", value);
    }
}
