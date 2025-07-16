use super::*;

/// The IntrinsicSizes class.
/// [`IntrinsicSizes`](https://developer.mozilla.org/en-US/docs/Web/API/IntrinsicSizes)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IntrinsicSizes {
    inner: Any,
}
impl FromVal for IntrinsicSizes {
    fn from_val(v: &Any) -> Self {
        IntrinsicSizes {
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
impl core::ops::Deref for IntrinsicSizes {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IntrinsicSizes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IntrinsicSizes {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IntrinsicSizes {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IntrinsicSizes> for Any {
    fn from(s: IntrinsicSizes) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IntrinsicSizes> for Any {
    fn from(s: &IntrinsicSizes) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IntrinsicSizes);

impl IntrinsicSizes {
    /// Getter of the `minContentSize` attribute.
    /// [`IntrinsicSizes.minContentSize`](https://developer.mozilla.org/en-US/docs/Web/API/IntrinsicSizes/minContentSize)
    pub fn min_content_size(&self) -> f64 {
        self.inner.get("minContentSize").as_::<f64>()
    }
}
impl IntrinsicSizes {
    /// Getter of the `maxContentSize` attribute.
    /// [`IntrinsicSizes.maxContentSize`](https://developer.mozilla.org/en-US/docs/Web/API/IntrinsicSizes/maxContentSize)
    pub fn max_content_size(&self) -> f64 {
        self.inner.get("maxContentSize").as_::<f64>()
    }
}
