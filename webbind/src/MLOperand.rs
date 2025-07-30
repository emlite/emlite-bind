use super::*;

/// The MLOperand class.
/// [`MLOperand`](https://developer.mozilla.org/en-US/docs/Web/API/MLOperand)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOperand {
    inner: Any,
}
impl FromVal for MLOperand {
    fn from_val(v: &Any) -> Self {
        MLOperand {
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
impl core::ops::Deref for MLOperand {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLOperand {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLOperand {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLOperand> for Any {
    fn from(s: MLOperand) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLOperand> for Any {
    fn from(s: &MLOperand) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MLOperand);

impl MLOperand {
    /// Getter of the `dataType` attribute.
    /// [`MLOperand.dataType`](https://developer.mozilla.org/en-US/docs/Web/API/MLOperand/dataType)
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }
}
impl MLOperand {
    /// Getter of the `shape` attribute.
    /// [`MLOperand.shape`](https://developer.mozilla.org/en-US/docs/Web/API/MLOperand/shape)
    pub fn shape(&self) -> TypedArray<u32> {
        self.inner.get("shape").as_::<TypedArray<u32>>()
    }
}
