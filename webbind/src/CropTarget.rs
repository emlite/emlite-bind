use super::*;

/// The CropTarget class.
/// [`CropTarget`](https://developer.mozilla.org/en-US/docs/Web/API/CropTarget)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CropTarget {
    inner: Any,
}
impl FromVal for CropTarget {
    fn from_val(v: &Any) -> Self {
        CropTarget {
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
impl core::ops::Deref for CropTarget {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CropTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CropTarget {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CropTarget {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CropTarget> for Any {
    fn from(s: CropTarget) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CropTarget> for Any {
    fn from(s: &CropTarget) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CropTarget);

impl CropTarget {
    /// The fromElement method.
    /// [`CropTarget.fromElement`](https://developer.mozilla.org/en-US/docs/Web/API/CropTarget/fromElement)
    pub fn from_element(element: &Element) -> Promise {
        Any::global("CropTarget")
            .call("fromElement", &[element.into()])
            .as_::<Promise>()
    }
}
