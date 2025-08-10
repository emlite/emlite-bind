use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntersectionObserverInit {
    inner: Any,
}
impl FromVal for IntersectionObserverInit {
    fn from_val(v: &Any) -> Self {
        IntersectionObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IntersectionObserverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IntersectionObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IntersectionObserverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IntersectionObserverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IntersectionObserverInit> for Any {
    fn from(s: IntersectionObserverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IntersectionObserverInit> for Any {
    fn from(s: &IntersectionObserverInit) -> Any {
        s.inner.clone()
    }
}

impl IntersectionObserverInit {
    pub fn root(&self) -> Any {
        self.inner.get("root").as_::<Any>()
    }

    pub fn set_root(&mut self, value: &Any) {
        self.inner.set("root", value);
    }
}
impl IntersectionObserverInit {
    pub fn root_margin(&self) -> JsString {
        self.inner.get("rootMargin").as_::<JsString>()
    }

    pub fn set_root_margin(&mut self, value: &JsString) {
        self.inner.set("rootMargin", value);
    }
}
impl IntersectionObserverInit {
    pub fn scroll_margin(&self) -> JsString {
        self.inner.get("scrollMargin").as_::<JsString>()
    }

    pub fn set_scroll_margin(&mut self, value: &JsString) {
        self.inner.set("scrollMargin", value);
    }
}
impl IntersectionObserverInit {
    pub fn threshold(&self) -> Any {
        self.inner.get("threshold").as_::<Any>()
    }

    pub fn set_threshold(&mut self, value: &Any) {
        self.inner.set("threshold", value);
    }
}
impl IntersectionObserverInit {
    pub fn delay(&self) -> i32 {
        self.inner.get("delay").as_::<i32>()
    }

    pub fn set_delay(&mut self, value: i32) {
        self.inner.set("delay", value);
    }
}
impl IntersectionObserverInit {
    pub fn track_visibility(&self) -> bool {
        self.inner.get("trackVisibility").as_::<bool>()
    }

    pub fn set_track_visibility(&mut self, value: bool) {
        self.inner.set("trackVisibility", value);
    }
}
