use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GPUQuerySet {
    inner: emlite::Val,
}
impl FromVal for GPUQuerySet {
    fn from_val(v: &emlite::Val) -> Self {
        GPUQuerySet {
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
impl core::ops::Deref for GPUQuerySet {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUQuerySet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GPUQuerySet> for emlite::Val {
    fn from(s: GPUQuerySet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl GPUQuerySet {
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
impl GPUQuerySet {
    pub fn type_(&self) -> GPUQueryType {
        self.inner.get("type").as_::<GPUQueryType>()
    }
}
impl GPUQuerySet {
    pub fn count(&self) -> jsbind::Any {
        self.inner.get("count").as_::<jsbind::Any>()
    }
}
impl GPUQuerySet {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
