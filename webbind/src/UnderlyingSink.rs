use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnderlyingSink {
    inner: Any,
}
impl FromVal for UnderlyingSink {
    fn from_val(v: &Any) -> Self {
        UnderlyingSink { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for UnderlyingSink {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for UnderlyingSink {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for UnderlyingSink {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for UnderlyingSink {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<UnderlyingSink> for Any {
    fn from(s: UnderlyingSink) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&UnderlyingSink> for Any {
    fn from(s: &UnderlyingSink) -> Any {
        s.inner.clone()
    }
}

impl UnderlyingSink {
    pub fn start(&self) -> Function {
        self.inner.get("start").as_::<Function>()
    }

    pub fn set_start(&mut self, value: &Function) {
        self.inner.set("start", value);
    }
}
impl UnderlyingSink {
    pub fn write(&self) -> Function {
        self.inner.get("write").as_::<Function>()
    }

    pub fn set_write(&mut self, value: &Function) {
        self.inner.set("write", value);
    }
}
impl UnderlyingSink {
    pub fn close(&self) -> Function {
        self.inner.get("close").as_::<Function>()
    }

    pub fn set_close(&mut self, value: &Function) {
        self.inner.set("close", value);
    }
}
impl UnderlyingSink {
    pub fn abort(&self) -> Function {
        self.inner.get("abort").as_::<Function>()
    }

    pub fn set_abort(&mut self, value: &Function) {
        self.inner.set("abort", value);
    }
}
impl UnderlyingSink {
    pub fn type_(&self) -> Any {
        self.inner.get("type").as_::<Any>()
    }

    pub fn set_type_(&mut self, value: &Any) {
        self.inner.set("type", value);
    }
}
