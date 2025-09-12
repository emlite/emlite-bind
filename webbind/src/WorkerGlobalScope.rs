use super::*;

/// The WorkerGlobalScope class.
/// [`WorkerGlobalScope`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerGlobalScope {
    inner: EventTarget,
}

impl FromVal for WorkerGlobalScope {
    fn from_val(v: &Any) -> Self {
        WorkerGlobalScope {
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

impl core::ops::Deref for WorkerGlobalScope {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkerGlobalScope {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkerGlobalScope {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<WorkerGlobalScope> for Any {
    fn from(s: WorkerGlobalScope) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkerGlobalScope> for Any {
    fn from(s: &WorkerGlobalScope) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkerGlobalScope);

impl WorkerGlobalScope {
    /// Getter of the `self` attribute.
    /// [`WorkerGlobalScope.self`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/self)
    pub fn self_(&self) -> WorkerGlobalScope {
        self.inner.get("self").as_::<WorkerGlobalScope>()
    }
}
impl WorkerGlobalScope {
    /// Getter of the `location` attribute.
    /// [`WorkerGlobalScope.location`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/location)
    pub fn location(&self) -> WorkerLocation {
        self.inner.get("location").as_::<WorkerLocation>()
    }
}
impl WorkerGlobalScope {
    /// Getter of the `navigator` attribute.
    /// [`WorkerGlobalScope.navigator`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/navigator)
    pub fn navigator(&self) -> WorkerNavigator {
        self.inner.get("navigator").as_::<WorkerNavigator>()
    }
}
impl WorkerGlobalScope {
    /// Getter of the `onerror` attribute.
    /// [`WorkerGlobalScope.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    /// Setter of the `onerror` attribute.
    /// [`WorkerGlobalScope.onerror`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onerror)
    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `onlanguagechange` attribute.
    /// [`WorkerGlobalScope.onlanguagechange`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onlanguagechange)
    pub fn onlanguagechange(&self) -> Any {
        self.inner.get("onlanguagechange").as_::<Any>()
    }

    /// Setter of the `onlanguagechange` attribute.
    /// [`WorkerGlobalScope.onlanguagechange`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onlanguagechange)
    pub fn set_onlanguagechange(&mut self, value: &Any) {
        self.inner.set("onlanguagechange", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `onoffline` attribute.
    /// [`WorkerGlobalScope.onoffline`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)
    pub fn onoffline(&self) -> Any {
        self.inner.get("onoffline").as_::<Any>()
    }

    /// Setter of the `onoffline` attribute.
    /// [`WorkerGlobalScope.onoffline`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onoffline)
    pub fn set_onoffline(&mut self, value: &Any) {
        self.inner.set("onoffline", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `ononline` attribute.
    /// [`WorkerGlobalScope.ononline`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)
    pub fn ononline(&self) -> Any {
        self.inner.get("ononline").as_::<Any>()
    }

    /// Setter of the `ononline` attribute.
    /// [`WorkerGlobalScope.ononline`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/ononline)
    pub fn set_ononline(&mut self, value: &Any) {
        self.inner.set("ononline", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `onrejectionhandled` attribute.
    /// [`WorkerGlobalScope.onrejectionhandled`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onrejectionhandled)
    pub fn onrejectionhandled(&self) -> Any {
        self.inner.get("onrejectionhandled").as_::<Any>()
    }

    /// Setter of the `onrejectionhandled` attribute.
    /// [`WorkerGlobalScope.onrejectionhandled`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onrejectionhandled)
    pub fn set_onrejectionhandled(&mut self, value: &Any) {
        self.inner.set("onrejectionhandled", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `onunhandledrejection` attribute.
    /// [`WorkerGlobalScope.onunhandledrejection`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onunhandledrejection)
    pub fn onunhandledrejection(&self) -> Any {
        self.inner.get("onunhandledrejection").as_::<Any>()
    }

    /// Setter of the `onunhandledrejection` attribute.
    /// [`WorkerGlobalScope.onunhandledrejection`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/onunhandledrejection)
    pub fn set_onunhandledrejection(&mut self, value: &Any) {
        self.inner.set("onunhandledrejection", value);
    }
}
impl WorkerGlobalScope {
    /// Getter of the `fonts` attribute.
    /// [`WorkerGlobalScope.fonts`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/fonts)
    pub fn fonts(&self) -> FontFaceSet {
        self.inner.get("fonts").as_::<FontFaceSet>()
    }
}
impl WorkerGlobalScope {
    /// Getter of the `crypto` attribute.
    /// [`WorkerGlobalScope.crypto`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/crypto)
    pub fn crypto(&self) -> Crypto {
        self.inner.get("crypto").as_::<Crypto>()
    }
}
impl WorkerGlobalScope {
    /// The importScripts method.
    /// [`WorkerGlobalScope.importScripts`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerGlobalScope/importScripts)
    pub fn import_scripts(&self, urls: &Any) -> Undefined {
        self.inner
            .call("importScripts", &[urls.into()])
            .as_::<Undefined>()
    }
}
