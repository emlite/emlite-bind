use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TrackEventInit {
    inner: Any,
}
impl FromVal for TrackEventInit {
    fn from_val(v: &Any) -> Self {
        TrackEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for TrackEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for TrackEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for TrackEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for TrackEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<TrackEventInit> for Any {
    fn from(s: TrackEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&TrackEventInit> for Any {
    fn from(s: &TrackEventInit) -> Any {
        s.inner.clone()
    }
}

impl TrackEventInit {
    pub fn track(&self) -> Any {
        self.inner.get("track").as_::<Any>()
    }

    pub fn set_track(&mut self, value: &Any) {
        self.inner.set("track", value);
    }
}
