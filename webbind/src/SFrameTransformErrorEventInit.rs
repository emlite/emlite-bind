use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SFrameTransformErrorEventInit {
    inner: Any,
}
impl FromVal for SFrameTransformErrorEventInit {
    fn from_val(v: &Any) -> Self {
        SFrameTransformErrorEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SFrameTransformErrorEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SFrameTransformErrorEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SFrameTransformErrorEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SFrameTransformErrorEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SFrameTransformErrorEventInit> for Any {
    fn from(s: SFrameTransformErrorEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SFrameTransformErrorEventInit> for Any {
    fn from(s: &SFrameTransformErrorEventInit) -> Any {
        s.inner.clone()
    }
}

impl SFrameTransformErrorEventInit {
    pub fn error_type(&self) -> SFrameTransformErrorEventType {
        self.inner
            .get("errorType")
            .as_::<SFrameTransformErrorEventType>()
    }

    pub fn set_error_type(&mut self, value: &SFrameTransformErrorEventType) {
        self.inner.set("errorType", value);
    }
}
impl SFrameTransformErrorEventInit {
    pub fn frame(&self) -> Any {
        self.inner.get("frame").as_::<Any>()
    }

    pub fn set_frame(&mut self, value: &Any) {
        self.inner.set("frame", value);
    }
}
impl SFrameTransformErrorEventInit {
    pub fn key_id(&self) -> Any {
        self.inner.get("keyID").as_::<Any>()
    }

    pub fn set_key_id(&mut self, value: &Any) {
        self.inner.set("keyID", value);
    }
}
