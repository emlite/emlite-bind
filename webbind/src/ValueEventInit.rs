use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ValueEventInit {
    inner: Any,
}
impl FromVal for ValueEventInit {
    fn from_val(v: &Any) -> Self {
        ValueEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ValueEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ValueEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ValueEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ValueEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ValueEventInit> for Any {
    fn from(s: ValueEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ValueEventInit> for Any {
    fn from(s: &ValueEventInit) -> Any {
        s.inner.clone()
    }
}

impl ValueEventInit {
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
