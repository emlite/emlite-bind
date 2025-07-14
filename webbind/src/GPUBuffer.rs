use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<GPUBuffer> for emlite::Val {
    fn from(s: GPUBuffer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUBuffer {
    pub fn size(&self) -> jsbind::Any {
        self.inner.get("size").as_::<jsbind::Any>()
    }
}
impl GPUBuffer {
    pub fn usage(&self) -> jsbind::Any {
        self.inner.get("usage").as_::<jsbind::Any>()
    }
}
impl GPUBuffer {
    pub fn map_state(&self) -> GPUBufferMapState {
        self.inner.get("mapState").as_::<GPUBufferMapState>()
    }
}
impl GPUBuffer {
    pub fn map_async0(&self, mode: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("mapAsync", &[mode.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn map_async1(&self, mode: jsbind::Any, offset: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn map_async2(
        &self,
        mode: jsbind::Any,
        offset: jsbind::Any,
        size: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call("mapAsync", &[mode.into(), offset.into(), size.into()])
            .as_::<jsbind::Promise>()
    }
}
impl GPUBuffer {
    pub fn get_mapped_range0(&self) -> jsbind::ArrayBuffer {
        self.inner
            .call("getMappedRange", &[])
            .as_::<jsbind::ArrayBuffer>()
    }

    pub fn get_mapped_range1(&self, offset: jsbind::Any) -> jsbind::ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into()])
            .as_::<jsbind::ArrayBuffer>()
    }

    pub fn get_mapped_range2(&self, offset: jsbind::Any, size: jsbind::Any) -> jsbind::ArrayBuffer {
        self.inner
            .call("getMappedRange", &[offset.into(), size.into()])
            .as_::<jsbind::ArrayBuffer>()
    }
}
impl GPUBuffer {
    pub fn unmap(&self) -> jsbind::Undefined {
        self.inner.call("unmap", &[]).as_::<jsbind::Undefined>()
    }
}
impl GPUBuffer {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
impl GPUBuffer {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
