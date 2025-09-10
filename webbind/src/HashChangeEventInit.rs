use super::*;

/// The HashChangeEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HashChangeEventInit {
    inner: Any,
}

impl FromVal for HashChangeEventInit {
    fn from_val(v: &Any) -> Self {
        HashChangeEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for HashChangeEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for HashChangeEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for HashChangeEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for HashChangeEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<HashChangeEventInit> for Any {
    fn from(s: HashChangeEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&HashChangeEventInit> for Any {
    fn from(s: &HashChangeEventInit) -> Any {
        s.inner.clone()
    }
}

impl HashChangeEventInit {
    /// Getter of the `oldURL` attribute.
    pub fn old_url(&self) -> JsString {
        self.inner.get("oldURL").as_::<JsString>()
    }

    /// Setter of the `oldURL` attribute.
    pub fn set_old_url(&mut self, value: &JsString) {
        self.inner.set("oldURL", value);
    }
}
impl HashChangeEventInit {
    /// Getter of the `newURL` attribute.
    pub fn new_url(&self) -> JsString {
        self.inner.get("newURL").as_::<JsString>()
    }

    /// Setter of the `newURL` attribute.
    pub fn set_new_url(&mut self, value: &JsString) {
        self.inner.set("newURL", value);
    }
}
