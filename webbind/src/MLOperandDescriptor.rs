use super::*;

/// The MLOperandDescriptor dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOperandDescriptor {
    inner: Any,
}

impl FromVal for MLOperandDescriptor {
    fn from_val(v: &Any) -> Self {
        MLOperandDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLOperandDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLOperandDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLOperandDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLOperandDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLOperandDescriptor> for Any {
    fn from(s: MLOperandDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLOperandDescriptor> for Any {
    fn from(s: &MLOperandDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MLOperandDescriptor {
    /// Getter of the `dataType` attribute.
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }

    /// Setter of the `dataType` attribute.
    pub fn set_data_type(&mut self, value: &MLOperandDataType) {
        self.inner.set("dataType", value);
    }
}
impl MLOperandDescriptor {
    /// Getter of the `shape` attribute.
    pub fn shape(&self) -> TypedArray<u32> {
        self.inner.get("shape").as_::<TypedArray<u32>>()
    }

    /// Setter of the `shape` attribute.
    pub fn set_shape(&mut self, value: TypedArray<u32>) {
        self.inner.set("shape", value);
    }
}
