use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCStats {
    inner: Any,
}
impl FromVal for RTCStats {
    fn from_val(v: &Any) -> Self {
        RTCStats { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCStats {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCStats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCStats {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCStats {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCStats> for Any {
    fn from(s: RTCStats) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCStats> for Any {
    fn from(s: &RTCStats) -> Any {
        s.inner.clone()
    }
}

impl RTCStats {
    pub fn timestamp(&self) -> Any {
        self.inner.get("timestamp").as_::<Any>()
    }

    pub fn set_timestamp(&mut self, value: &Any) {
        self.inner.set("timestamp", value);
    }
}
impl RTCStats {
    pub fn type_(&self) -> RTCStatsType {
        self.inner.get("type").as_::<RTCStatsType>()
    }

    pub fn set_type_(&mut self, value: &RTCStatsType) {
        self.inner.set("type", value);
    }
}
impl RTCStats {
    pub fn id(&self) -> JsString {
        self.inner.get("id").as_::<JsString>()
    }

    pub fn set_id(&mut self, value: &JsString) {
        self.inner.set("id", value);
    }
}
