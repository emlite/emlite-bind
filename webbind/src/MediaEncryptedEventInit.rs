use super::*;

/// The MediaEncryptedEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaEncryptedEventInit {
    inner: Any,
}

impl FromVal for MediaEncryptedEventInit {
    fn from_val(v: &Any) -> Self {
        MediaEncryptedEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MediaEncryptedEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MediaEncryptedEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MediaEncryptedEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MediaEncryptedEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MediaEncryptedEventInit> for Any {
    fn from(s: MediaEncryptedEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MediaEncryptedEventInit> for Any {
    fn from(s: &MediaEncryptedEventInit) -> Any {
        s.inner.clone()
    }
}

impl MediaEncryptedEventInit {
    /// Getter of the `initDataType` attribute.
    pub fn init_data_type(&self) -> JsString {
        self.inner.get("initDataType").as_::<JsString>()
    }

    /// Setter of the `initDataType` attribute.
    pub fn set_init_data_type(&mut self, value: &JsString) {
        self.inner.set("initDataType", value);
    }
}
impl MediaEncryptedEventInit {
    /// Getter of the `initData` attribute.
    pub fn init_data(&self) -> ArrayBuffer {
        self.inner.get("initData").as_::<ArrayBuffer>()
    }

    /// Setter of the `initData` attribute.
    pub fn set_init_data(&mut self, value: &ArrayBuffer) {
        self.inner.set("initData", value);
    }
}
