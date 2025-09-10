use super::*;

/// The VideoEncoderInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct VideoEncoderInit {
    inner: Any,
}

impl FromVal for VideoEncoderInit {
    fn from_val(v: &Any) -> Self {
        VideoEncoderInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for VideoEncoderInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for VideoEncoderInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for VideoEncoderInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for VideoEncoderInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<VideoEncoderInit> for Any {
    fn from(s: VideoEncoderInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&VideoEncoderInit> for Any {
    fn from(s: &VideoEncoderInit) -> Any {
        s.inner.clone()
    }
}

impl VideoEncoderInit {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> Function {
        self.inner.get("output").as_::<Function>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &Function) {
        self.inner.set("output", value);
    }
}
impl VideoEncoderInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> Function {
        self.inner.get("error").as_::<Function>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &Function) {
        self.inner.set("error", value);
    }
}
