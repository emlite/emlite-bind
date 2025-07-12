use super::*;

#[derive(Clone, Debug)]
pub struct PluginArray {
    inner: emlite::Val,
}
impl FromVal for PluginArray {
    fn from_val(v: &emlite::Val) -> Self {
        PluginArray {
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
impl std::ops::Deref for PluginArray {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PluginArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PluginArray> for emlite::Val {
    fn from(s: PluginArray) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PluginArray {
    pub fn refresh(&self) -> jsbind::Undefined {
        self.inner.call("refresh", &[]).as_::<jsbind::Undefined>()
    }
}
impl PluginArray {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl PluginArray {
    pub fn item(&self, index: u32) -> Plugin {
        self.inner.call("item", &[index.into()]).as_::<Plugin>()
    }
}
impl PluginArray {
    pub fn named_item(&self, name: jsbind::DOMString) -> Plugin {
        self.inner.call("namedItem", &[name.into()]).as_::<Plugin>()
    }
}
