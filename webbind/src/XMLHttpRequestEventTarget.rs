use super::*;

/// The XMLHttpRequestEventTarget class.
/// [`XMLHttpRequestEventTarget`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XMLHttpRequestEventTarget {
    inner: EventTarget,
}

impl FromVal for XMLHttpRequestEventTarget {
    fn from_val(v: &Any) -> Self {
        XMLHttpRequestEventTarget {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for XMLHttpRequestEventTarget {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XMLHttpRequestEventTarget {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XMLHttpRequestEventTarget> for Any {
    fn from(s: XMLHttpRequestEventTarget) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XMLHttpRequestEventTarget> for Any {
    fn from(s: &XMLHttpRequestEventTarget) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(XMLHttpRequestEventTarget);

impl XMLHttpRequestEventTarget {
    /// Getter of the `onloadstart` attribute.
    /// [`XMLHttpRequestEventTarget.onloadstart`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)
    pub fn onloadstart(&self) -> Any {
        self.inner.get("onloadstart").as_::<Any>()
    }

    /// Setter of the `onloadstart` attribute.
    /// [`XMLHttpRequestEventTarget.onloadstart`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadstart)
    pub fn set_onloadstart(&mut self, value: &Any) {
        self.inner.set("onloadstart", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `onprogress` attribute.
    /// [`XMLHttpRequestEventTarget.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)
    pub fn onprogress(&self) -> Any {
        self.inner.get("onprogress").as_::<Any>()
    }

    /// Setter of the `onprogress` attribute.
    /// [`XMLHttpRequestEventTarget.onprogress`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onprogress)
    pub fn set_onprogress(&mut self, value: &Any) {
        self.inner.set("onprogress", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `onabort` attribute.
    /// [`XMLHttpRequestEventTarget.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`XMLHttpRequestEventTarget.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `onerror` attribute.
    /// [`XMLHttpRequestEventTarget.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`XMLHttpRequestEventTarget.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `onload` attribute.
    /// [`XMLHttpRequestEventTarget.onload`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)
    pub fn onload(&self) -> Any {
        self.inner.get("onload").as_::<Any>()
    }

    /// Setter of the `onload` attribute.
    /// [`XMLHttpRequestEventTarget.onload`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onload)
    pub fn set_onload(&mut self, value: &Any) {
        self.inner.set("onload", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `ontimeout` attribute.
    /// [`XMLHttpRequestEventTarget.ontimeout`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)
    pub fn ontimeout(&self) -> Any {
        self.inner.get("ontimeout").as_::<Any>()
    }

    /// Setter of the `ontimeout` attribute.
    /// [`XMLHttpRequestEventTarget.ontimeout`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/ontimeout)
    pub fn set_ontimeout(&mut self, value: &Any) {
        self.inner.set("ontimeout", value);
    }
}
impl XMLHttpRequestEventTarget {
    /// Getter of the `onloadend` attribute.
    /// [`XMLHttpRequestEventTarget.onloadend`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)
    pub fn onloadend(&self) -> Any {
        self.inner.get("onloadend").as_::<Any>()
    }

    /// Setter of the `onloadend` attribute.
    /// [`XMLHttpRequestEventTarget.onloadend`](https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequestEventTarget/onloadend)
    pub fn set_onloadend(&mut self, value: &Any) {
        self.inner.set("onloadend", value);
    }
}
