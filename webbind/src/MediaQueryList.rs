use super::*;

/// The MediaQueryList class.
/// [`MediaQueryList`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaQueryList {
    inner: EventTarget,
}

impl FromVal for MediaQueryList {
    fn from_val(v: &Any) -> Self {
        MediaQueryList {
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

impl core::ops::Deref for MediaQueryList {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaQueryList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaQueryList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaQueryList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaQueryList> for Any {
    fn from(s: MediaQueryList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaQueryList> for Any {
    fn from(s: &MediaQueryList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MediaQueryList);

impl MediaQueryList {
    /// Getter of the `media` attribute.
    /// [`MediaQueryList.media`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/media)
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }
}
impl MediaQueryList {
    /// Getter of the `matches` attribute.
    /// [`MediaQueryList.matches`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/matches)
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }
}
impl MediaQueryList {
    /// Getter of the `onchange` attribute.
    /// [`MediaQueryList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`MediaQueryList.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl MediaQueryList {
    /// The addListener method.
    /// [`MediaQueryList.addListener`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/addListener)
    pub fn add_listener(&self, callback: &EventListener) -> Undefined {
        self.inner
            .call("addListener", &[callback.into()])
            .as_::<Undefined>()
    }
}
impl MediaQueryList {
    /// The removeListener method.
    /// [`MediaQueryList.removeListener`](https://developer.mozilla.org/en-US/docs/Web/API/MediaQueryList/removeListener)
    pub fn remove_listener(&self, callback: &EventListener) -> Undefined {
        self.inner
            .call("removeListener", &[callback.into()])
            .as_::<Undefined>()
    }
}
