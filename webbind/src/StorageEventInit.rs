use super::*;

/// The StorageEventInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StorageEventInit {
    inner: Any,
}

impl FromVal for StorageEventInit {
    fn from_val(v: &Any) -> Self {
        StorageEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for StorageEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for StorageEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for StorageEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for StorageEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<StorageEventInit> for Any {
    fn from(s: StorageEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&StorageEventInit> for Any {
    fn from(s: &StorageEventInit) -> Any {
        s.inner.clone()
    }
}

impl StorageEventInit {
    /// Getter of the `key` attribute.
    pub fn key(&self) -> JsString {
        self.inner.get("key").as_::<JsString>()
    }

    /// Setter of the `key` attribute.
    pub fn set_key(&mut self, value: &JsString) {
        self.inner.set("key", value);
    }
}
impl StorageEventInit {
    /// Getter of the `oldValue` attribute.
    pub fn old_value(&self) -> JsString {
        self.inner.get("oldValue").as_::<JsString>()
    }

    /// Setter of the `oldValue` attribute.
    pub fn set_old_value(&mut self, value: &JsString) {
        self.inner.set("oldValue", value);
    }
}
impl StorageEventInit {
    /// Getter of the `newValue` attribute.
    pub fn new_value(&self) -> JsString {
        self.inner.get("newValue").as_::<JsString>()
    }

    /// Setter of the `newValue` attribute.
    pub fn set_new_value(&mut self, value: &JsString) {
        self.inner.set("newValue", value);
    }
}
impl StorageEventInit {
    /// Getter of the `url` attribute.
    pub fn url(&self) -> JsString {
        self.inner.get("url").as_::<JsString>()
    }

    /// Setter of the `url` attribute.
    pub fn set_url(&mut self, value: &JsString) {
        self.inner.set("url", value);
    }
}
impl StorageEventInit {
    /// Getter of the `storageArea` attribute.
    pub fn storage_area(&self) -> Storage {
        self.inner.get("storageArea").as_::<Storage>()
    }

    /// Setter of the `storageArea` attribute.
    pub fn set_storage_area(&mut self, value: &Storage) {
        self.inner.set("storageArea", value);
    }
}
