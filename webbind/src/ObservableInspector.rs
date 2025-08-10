use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ObservableInspector {
    inner: Any,
}
impl FromVal for ObservableInspector {
    fn from_val(v: &Any) -> Self {
        ObservableInspector { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ObservableInspector {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ObservableInspector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ObservableInspector {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ObservableInspector {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ObservableInspector> for Any {
    fn from(s: ObservableInspector) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ObservableInspector> for Any {
    fn from(s: &ObservableInspector) -> Any {
        s.inner.clone()
    }
}

impl ObservableInspector {
    pub fn next(&self) -> Function {
        self.inner.get("next").as_::<Function>()
    }

    pub fn set_next(&mut self, value: &Function) {
        self.inner.set("next", value);
    }
}
impl ObservableInspector {
    pub fn error(&self) -> Function {
        self.inner.get("error").as_::<Function>()
    }

    pub fn set_error(&mut self, value: &Function) {
        self.inner.set("error", value);
    }
}
impl ObservableInspector {
    pub fn complete(&self) -> Function {
        self.inner.get("complete").as_::<Function>()
    }

    pub fn set_complete(&mut self, value: &Function) {
        self.inner.set("complete", value);
    }
}
impl ObservableInspector {
    pub fn subscribe(&self) -> Function {
        self.inner.get("subscribe").as_::<Function>()
    }

    pub fn set_subscribe(&mut self, value: &Function) {
        self.inner.set("subscribe", value);
    }
}
impl ObservableInspector {
    pub fn abort(&self) -> Function {
        self.inner.get("abort").as_::<Function>()
    }

    pub fn set_abort(&mut self, value: &Function) {
        self.inner.set("abort", value);
    }
}
