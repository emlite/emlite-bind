use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AddEventListenerOptions {
    inner: Any,
}
impl FromVal for AddEventListenerOptions {
    fn from_val(v: &Any) -> Self {
        AddEventListenerOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AddEventListenerOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AddEventListenerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AddEventListenerOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AddEventListenerOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AddEventListenerOptions> for Any {
    fn from(s: AddEventListenerOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AddEventListenerOptions> for Any {
    fn from(s: &AddEventListenerOptions) -> Any {
        s.inner.clone()
    }
}

impl AddEventListenerOptions {
    pub fn passive(&self) -> bool {
        self.inner.get("passive").as_::<bool>()
    }

    pub fn set_passive(&mut self, value: bool) {
        self.inner.set("passive", value);
    }
}
impl AddEventListenerOptions {
    pub fn once(&self) -> bool {
        self.inner.get("once").as_::<bool>()
    }

    pub fn set_once(&mut self, value: bool) {
        self.inner.set("once", value);
    }
}
impl AddEventListenerOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
