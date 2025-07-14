use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUAdapterInfo {
    inner: emlite::Val,
}
impl FromVal for GPUAdapterInfo {
    fn from_val(v: &emlite::Val) -> Self {
        GPUAdapterInfo {
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
impl core::ops::Deref for GPUAdapterInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUAdapterInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUAdapterInfo> for emlite::Val {
    fn from(s: GPUAdapterInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUAdapterInfo {
    pub fn vendor(&self) -> jsbind::DOMString {
        self.inner.get("vendor").as_::<jsbind::DOMString>()
    }
}
impl GPUAdapterInfo {
    pub fn architecture(&self) -> jsbind::DOMString {
        self.inner.get("architecture").as_::<jsbind::DOMString>()
    }
}
impl GPUAdapterInfo {
    pub fn device(&self) -> jsbind::DOMString {
        self.inner.get("device").as_::<jsbind::DOMString>()
    }
}
impl GPUAdapterInfo {
    pub fn description(&self) -> jsbind::DOMString {
        self.inner.get("description").as_::<jsbind::DOMString>()
    }
}
impl GPUAdapterInfo {
    pub fn subgroup_min_size(&self) -> u32 {
        self.inner.get("subgroupMinSize").as_::<u32>()
    }
}
impl GPUAdapterInfo {
    pub fn subgroup_max_size(&self) -> u32 {
        self.inner.get("subgroupMaxSize").as_::<u32>()
    }
}
impl GPUAdapterInfo {
    pub fn is_fallback_adapter(&self) -> bool {
        self.inner.get("isFallbackAdapter").as_::<bool>()
    }
}
