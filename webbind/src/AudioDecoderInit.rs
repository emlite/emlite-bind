use super::*;

/// The AudioDecoderInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AudioDecoderInit {
    inner: Any,
}

impl FromVal for AudioDecoderInit {
    fn from_val(v: &Any) -> Self {
        AudioDecoderInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for AudioDecoderInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for AudioDecoderInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for AudioDecoderInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for AudioDecoderInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<AudioDecoderInit> for Any {
    fn from(s: AudioDecoderInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&AudioDecoderInit> for Any {
    fn from(s: &AudioDecoderInit) -> Any {
        s.inner.clone()
    }
}

impl AudioDecoderInit {
    /// Getter of the `output` attribute.
    pub fn output(&self) -> Function {
        self.inner.get("output").as_::<Function>()
    }

    /// Setter of the `output` attribute.
    pub fn set_output(&mut self, value: &Function) {
        self.inner.set("output", value);
    }
}
impl AudioDecoderInit {
    /// Getter of the `error` attribute.
    pub fn error(&self) -> Function {
        self.inner.get("error").as_::<Function>()
    }

    /// Setter of the `error` attribute.
    pub fn set_error(&mut self, value: &Function) {
        self.inner.set("error", value);
    }
}
