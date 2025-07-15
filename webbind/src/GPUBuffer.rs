use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBuffer {
    inner: emlite::Val,
}
impl FromVal for GPUBuffer {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBuffer {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBuffer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUBuffer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUBuffer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUBuffer> for emlite::Val {
    fn from(s: GPUBuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUBuffer> for emlite::Val {
    fn from(s: &GPUBuffer) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUBuffer);

impl GPUBuffer {
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }
}
impl GPUBuffer {
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }
}
impl GPUBuffer {
    pub fn map_state(&self) -> GPUBufferMapState {
        self.inner.get("mapState").as_::<GPUBufferMapState>()
    }
}
impl GPUBuffer {
    pub fn map_async0(&self, mode: &Any) -> Promise {
        self.inner.call("mapAsync", &[mode.into()]).as_::<Promise>()
    }

    pub fn map_async1(&self, mode: &Any, offset: &Any) -> Promise {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into()])
            .as_::<Promise>()
    }

    pub fn map_async2(&self, mode: &Any, offset: &Any, size: &Any) -> Promise {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into(), size.into()])
            .as_::<Promise>()
    }
}
impl GPUBuffer {
    pub fn get_mapped_range0(&self) -> ArrayBuffer {
        self.inner.call("getMappedRange", &[]).as_::<ArrayBuffer>()
    }

    pub fn get_mapped_range1(&self, offset: &Any) -> ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into()])
            .as_::<ArrayBuffer>()
    }

    pub fn get_mapped_range2(&self, offset: &Any, size: &Any) -> ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into(), size.into()])
            .as_::<ArrayBuffer>()
    }
}
impl GPUBuffer {
    pub fn unmap(&self) -> Undefined {
        self.inner.call("unmap", &[]).as_::<Undefined>()
    }
}
impl GPUBuffer {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUBuffer {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
