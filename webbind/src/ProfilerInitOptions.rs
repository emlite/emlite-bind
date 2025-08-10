use super::*;

/// The ProfilerInitOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ProfilerInitOptions {
    inner: Any,
}

impl FromVal for ProfilerInitOptions {
    fn from_val(v: &Any) -> Self {
        ProfilerInitOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for ProfilerInitOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ProfilerInitOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ProfilerInitOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ProfilerInitOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ProfilerInitOptions> for Any {
    fn from(s: ProfilerInitOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ProfilerInitOptions> for Any {
    fn from(s: &ProfilerInitOptions) -> Any {
        s.inner.clone()
    }
}

impl ProfilerInitOptions {
    /// Getter of the `sampleInterval` attribute.
    pub fn sample_interval(&self) -> Any {
        self.inner.get("sampleInterval").as_::<Any>()
    }

    /// Setter of the `sampleInterval` attribute.
    pub fn set_sample_interval(&mut self, value: &Any) {
        self.inner.set("sampleInterval", value);
    }
}
impl ProfilerInitOptions {
    /// Getter of the `maxBufferSize` attribute.
    pub fn max_buffer_size(&self) -> u32 {
        self.inner.get("maxBufferSize").as_::<u32>()
    }

    /// Setter of the `maxBufferSize` attribute.
    pub fn set_max_buffer_size(&mut self, value: u32) {
        self.inner.set("maxBufferSize", value);
    }
}
