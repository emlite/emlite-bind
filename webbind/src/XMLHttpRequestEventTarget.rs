use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLHttpRequestEventTarget {
    inner: EventTarget,
}
impl FromVal for XMLHttpRequestEventTarget {
    fn from_val(v: &emlite::Val) -> Self {
        XMLHttpRequestEventTarget {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XMLHttpRequestEventTarget {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XMLHttpRequestEventTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XMLHttpRequestEventTarget {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XMLHttpRequestEventTarget {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<XMLHttpRequestEventTarget> for emlite::Val {
    fn from(s: XMLHttpRequestEventTarget) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&XMLHttpRequestEventTarget> for emlite::Val {
    fn from(s: &XMLHttpRequestEventTarget) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XMLHttpRequestEventTarget);

impl XMLHttpRequestEventTarget {
    pub fn onloadstart(&self) -> Any {
        self.inner.get("onloadstart").as_::<Any>()
    }

    pub fn set_onloadstart(&mut self, value: &Any) {
        self.inner.set("onloadstart", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn onprogress(&self) -> Any {
        self.inner.get("onprogress").as_::<Any>()
    }

    pub fn set_onprogress(&mut self, value: &Any) {
        self.inner.set("onprogress", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn onload(&self) -> Any {
        self.inner.get("onload").as_::<Any>()
    }

    pub fn set_onload(&mut self, value: &Any) {
        self.inner.set("onload", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn ontimeout(&self) -> Any {
        self.inner.get("ontimeout").as_::<Any>()
    }

    pub fn set_ontimeout(&mut self, value: &Any) {
        self.inner.set("ontimeout", value);
    }
}
impl XMLHttpRequestEventTarget {
    pub fn onloadend(&self) -> Any {
        self.inner.get("onloadend").as_::<Any>()
    }

    pub fn set_onloadend(&mut self, value: &Any) {
        self.inner.set("onloadend", value);
    }
}
