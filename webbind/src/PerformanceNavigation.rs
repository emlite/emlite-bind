use super::*;




/// The PerformanceNavigation class.
/// [`PerformanceNavigation`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PerformanceNavigation {
    inner: Any,
}

impl FromVal for PerformanceNavigation {
    fn from_val(v: &Any) -> Self {
        PerformanceNavigation { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for PerformanceNavigation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PerformanceNavigation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PerformanceNavigation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PerformanceNavigation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<PerformanceNavigation> for Any {
    fn from(s: PerformanceNavigation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PerformanceNavigation> for Any {
    fn from(s: &PerformanceNavigation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PerformanceNavigation);


impl PerformanceNavigation {
    /// Getter of the `type` attribute.
    /// [`PerformanceNavigation.type`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/type)
    pub fn type_(&self) -> u16 {
        self.inner.get("type").as_::<u16>()
    }

}
impl PerformanceNavigation {
    /// Getter of the `redirectCount` attribute.
    /// [`PerformanceNavigation.redirectCount`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/redirectCount)
    pub fn redirect_count(&self) -> u16 {
        self.inner.get("redirectCount").as_::<u16>()
    }

}
impl PerformanceNavigation {
    /// The toJSON method.
    /// [`PerformanceNavigation.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceNavigation/toJSON)
    pub fn to_json(&self, ) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
