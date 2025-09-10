use super::*;

/// The UnderlyingSource dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnderlyingSource {
    inner: Any,
}

impl FromVal for UnderlyingSource {
    fn from_val(v: &Any) -> Self {
        UnderlyingSource { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for UnderlyingSource {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for UnderlyingSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for UnderlyingSource {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for UnderlyingSource {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<UnderlyingSource> for Any {
    fn from(s: UnderlyingSource) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&UnderlyingSource> for Any {
    fn from(s: &UnderlyingSource) -> Any {
        s.inner.clone()
    }
}

impl UnderlyingSource {
    /// Getter of the `start` attribute.
    pub fn start(&self) -> Function {
        self.inner.get("start").as_::<Function>()
    }

    /// Setter of the `start` attribute.
    pub fn set_start(&mut self, value: &Function) {
        self.inner.set("start", value);
    }
}
impl UnderlyingSource {
    /// Getter of the `pull` attribute.
    pub fn pull(&self) -> Function {
        self.inner.get("pull").as_::<Function>()
    }

    /// Setter of the `pull` attribute.
    pub fn set_pull(&mut self, value: &Function) {
        self.inner.set("pull", value);
    }
}
impl UnderlyingSource {
    /// Getter of the `cancel` attribute.
    pub fn cancel(&self) -> Function {
        self.inner.get("cancel").as_::<Function>()
    }

    /// Setter of the `cancel` attribute.
    pub fn set_cancel(&mut self, value: &Function) {
        self.inner.set("cancel", value);
    }
}
impl UnderlyingSource {
    /// Getter of the `type` attribute.
    pub fn type_(&self) -> ReadableStreamType {
        self.inner.get("type").as_::<ReadableStreamType>()
    }

    /// Setter of the `type` attribute.
    pub fn set_type_(&mut self, value: &ReadableStreamType) {
        self.inner.set("type", value);
    }
}
impl UnderlyingSource {
    /// Getter of the `autoAllocateChunkSize` attribute.
    pub fn auto_allocate_chunk_size(&self) -> u64 {
        self.inner.get("autoAllocateChunkSize").as_::<u64>()
    }

    /// Setter of the `autoAllocateChunkSize` attribute.
    pub fn set_auto_allocate_chunk_size(&mut self, value: u64) {
        self.inner.set("autoAllocateChunkSize", value);
    }
}
