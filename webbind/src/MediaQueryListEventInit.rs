use super::*;

/// The MediaQueryListEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaQueryListEventInit {
    inner: Any,
}

impl FromVal for MediaQueryListEventInit {
    fn from_val(v: &Any) -> Self {
        MediaQueryListEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaQueryListEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaQueryListEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaQueryListEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaQueryListEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaQueryListEventInit> for Any {
    fn from(s: MediaQueryListEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaQueryListEventInit> for Any {
    fn from(s: &MediaQueryListEventInit) -> Any {
        s.inner.clone()
    }
}

impl MediaQueryListEventInit {
    /// Getter of the `media` attribute.
    pub fn media(&self) -> JsString {
        self.inner.get("media").as_::<JsString>()
    }

    /// Setter of the `media` attribute.
    pub fn set_media(&mut self, value: &JsString) {
        self.inner.set("media", value);
    }
}
impl MediaQueryListEventInit {
    /// Getter of the `matches` attribute.
    pub fn matches(&self) -> bool {
        self.inner.get("matches").as_::<bool>()
    }

    /// Setter of the `matches` attribute.
    pub fn set_matches(&mut self, value: bool) {
        self.inner.set("matches", value);
    }
}
