use super::*;




/// The RestrictionTarget class.
/// [`RestrictionTarget`](https://developer.mozilla.org/en-US/docs/Web/API/RestrictionTarget)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RestrictionTarget {
    inner: Any,
}

impl FromVal for RestrictionTarget {
    fn from_val(v: &Any) -> Self {
        RestrictionTarget { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RestrictionTarget {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RestrictionTarget {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RestrictionTarget {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RestrictionTarget {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RestrictionTarget> for Any {
    fn from(s: RestrictionTarget) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RestrictionTarget> for Any {
    fn from(s: &RestrictionTarget) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RestrictionTarget);


impl RestrictionTarget {
    /// The fromElement method.
    /// [`RestrictionTarget.fromElement`](https://developer.mozilla.org/en-US/docs/Web/API/RestrictionTarget/fromElement)
    pub fn from_element(element: &Element) -> Promise<RestrictionTarget> {
        Any::global("RestrictionTarget").call("fromElement", &[element.into(), ]).as_::<Promise<RestrictionTarget>>()
    }
}
