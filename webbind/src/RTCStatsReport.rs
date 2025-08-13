use super::*;




/// The RTCStatsReport class.
/// [`RTCStatsReport`](https://developer.mozilla.org/en-US/docs/Web/API/RTCStatsReport)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCStatsReport {
    inner: Any,
}

impl FromVal for RTCStatsReport {
    fn from_val(v: &Any) -> Self {
        RTCStatsReport { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCStatsReport {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCStatsReport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCStatsReport {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCStatsReport {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCStatsReport> for Any {
    fn from(s: RTCStatsReport) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCStatsReport> for Any {
    fn from(s: &RTCStatsReport) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(RTCStatsReport);


