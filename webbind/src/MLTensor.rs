use super::*;

/// The MLTensor class.
/// [`MLTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensor {
    inner: Any,
}
impl FromVal for MLTensor {
    fn from_val(v: &Any) -> Self {
        MLTensor {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTensor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLTensor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLTensor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLTensor> for Any {
    fn from(s: MLTensor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLTensor> for Any {
    fn from(s: &MLTensor) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MLTensor);

impl MLTensor {
    /// Getter of the `dataType` attribute.
    /// [`MLTensor.dataType`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/dataType)
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }
}
impl MLTensor {
    /// Getter of the `shape` attribute.
    /// [`MLTensor.shape`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/shape)
    pub fn shape(&self) -> TypedArray<u32> {
        self.inner.get("shape").as_::<TypedArray<u32>>()
    }
}
impl MLTensor {
    /// Getter of the `readable` attribute.
    /// [`MLTensor.readable`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/readable)
    pub fn readable(&self) -> bool {
        self.inner.get("readable").as_::<bool>()
    }
}
impl MLTensor {
    /// Getter of the `writable` attribute.
    /// [`MLTensor.writable`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/writable)
    pub fn writable(&self) -> bool {
        self.inner.get("writable").as_::<bool>()
    }
}
impl MLTensor {
    /// Getter of the `constant` attribute.
    /// [`MLTensor.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/constant)
    pub fn constant(&self) -> bool {
        self.inner.get("constant").as_::<bool>()
    }
}
impl MLTensor {
    /// The destroy method.
    /// [`MLTensor.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/MLTensor/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
