use super::*;




/// The CustomStateSet class.
/// [`CustomStateSet`](https://developer.mozilla.org/en-US/docs/Web/API/CustomStateSet)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CustomStateSet {
    inner: Any,
}

impl FromVal for CustomStateSet {
    fn from_val(v: &Any) -> Self {
        CustomStateSet { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CustomStateSet {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CustomStateSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CustomStateSet {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CustomStateSet {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CustomStateSet> for Any {
    fn from(s: CustomStateSet) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CustomStateSet> for Any {
    fn from(s: &CustomStateSet) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CustomStateSet);


