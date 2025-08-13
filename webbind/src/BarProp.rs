use super::*;




/// The BarProp class.
/// [`BarProp`](https://developer.mozilla.org/en-US/docs/Web/API/BarProp)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BarProp {
    inner: Any,
}

impl FromVal for BarProp {
    fn from_val(v: &Any) -> Self {
        BarProp { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for BarProp {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BarProp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BarProp {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BarProp {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<BarProp> for Any {
    fn from(s: BarProp) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BarProp> for Any {
    fn from(s: &BarProp) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BarProp);


impl BarProp {
    /// Getter of the `visible` attribute.
    /// [`BarProp.visible`](https://developer.mozilla.org/en-US/docs/Web/API/BarProp/visible)
    pub fn visible(&self) -> bool {
        self.inner.get("visible").as_::<bool>()
    }

}
