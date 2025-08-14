use super::*;




/// The FetchLaterResult class.
/// [`FetchLaterResult`](https://developer.mozilla.org/en-US/docs/Web/API/FetchLaterResult)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FetchLaterResult {
    inner: Any,
}

impl FromVal for FetchLaterResult {
    fn from_val(v: &Any) -> Self {
        FetchLaterResult { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for FetchLaterResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FetchLaterResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FetchLaterResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FetchLaterResult {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<FetchLaterResult> for Any {
    fn from(s: FetchLaterResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FetchLaterResult> for Any {
    fn from(s: &FetchLaterResult) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FetchLaterResult);


impl FetchLaterResult {
    /// Getter of the `activated` attribute.
    /// [`FetchLaterResult.activated`](https://developer.mozilla.org/en-US/docs/Web/API/FetchLaterResult/activated)
    pub fn activated(&self) -> bool {
        self.inner.get("activated").as_::<bool>()
    }

}
