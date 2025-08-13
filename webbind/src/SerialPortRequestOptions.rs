use super::*;




/// The SerialPortRequestOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SerialPortRequestOptions {
    inner: Any,
}

impl FromVal for SerialPortRequestOptions {
    fn from_val(v: &Any) -> Self {
        SerialPortRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SerialPortRequestOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SerialPortRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SerialPortRequestOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SerialPortRequestOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SerialPortRequestOptions> for Any {
    fn from(s: SerialPortRequestOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SerialPortRequestOptions> for Any {
    fn from(s: &SerialPortRequestOptions) -> Any {
        s.inner.clone()
    }
}

impl SerialPortRequestOptions {
    /// Getter of the `filters` attribute.
    pub fn filters(&self) -> TypedArray<SerialPortFilter> {
        self.inner.get("filters").as_::<TypedArray<SerialPortFilter>>()
    }

    /// Setter of the `filters` attribute.
    pub fn set_filters(&mut self, value: &TypedArray<SerialPortFilter>) {
        self.inner.set("filters", value);
    }
}
impl SerialPortRequestOptions {
    /// Getter of the `allowedBluetoothServiceClassIds` attribute.
    pub fn allowed_bluetooth_service_class_ids(&self) -> TypedArray<Any> {
        self.inner.get("allowedBluetoothServiceClassIds").as_::<TypedArray<Any>>()
    }

    /// Setter of the `allowedBluetoothServiceClassIds` attribute.
    pub fn set_allowed_bluetooth_service_class_ids(&mut self, value: &TypedArray<Any>) {
        self.inner.set("allowedBluetoothServiceClassIds", value);
    }
}
