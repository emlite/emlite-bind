use super::*;

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
    pub fn start(&self) -> Function {
        self.inner.get("start").as_::<Function>()
    }

    pub fn set_start(&mut self, value: &Function) {
        self.inner.set("start", value);
    }
}
impl UnderlyingSource {
    pub fn pull(&self) -> Function {
        self.inner.get("pull").as_::<Function>()
    }

    pub fn set_pull(&mut self, value: &Function) {
        self.inner.set("pull", value);
    }
}
impl UnderlyingSource {
    pub fn cancel(&self) -> Function {
        self.inner.get("cancel").as_::<Function>()
    }

    pub fn set_cancel(&mut self, value: &Function) {
        self.inner.set("cancel", value);
    }
}
impl UnderlyingSource {
    pub fn type_(&self) -> ReadableStreamType {
        self.inner.get("type").as_::<ReadableStreamType>()
    }

    pub fn set_type_(&mut self, value: &ReadableStreamType) {
        self.inner.set("type", value);
    }
}
impl UnderlyingSource {
    pub fn auto_allocate_chunk_size(&self) -> u64 {
        self.inner.get("autoAllocateChunkSize").as_::<u64>()
    }

    pub fn set_auto_allocate_chunk_size(&mut self, value: u64) {
        self.inner.set("autoAllocateChunkSize", value);
    }
}
