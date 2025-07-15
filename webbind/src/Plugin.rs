use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Plugin {
    inner: emlite::Val,
}
impl FromVal for Plugin {
    fn from_val(v: &emlite::Val) -> Self {
        Plugin {
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
impl core::ops::Deref for Plugin {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Plugin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Plugin {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Plugin {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Plugin> for emlite::Val {
    fn from(s: Plugin) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Plugin> for emlite::Val {
    fn from(s: &Plugin) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Plugin);

impl Plugin {
    pub fn name(&self) -> String {
        self.inner.get("name").as_::<String>()
    }
}
impl Plugin {
    pub fn description(&self) -> String {
        self.inner.get("description").as_::<String>()
    }
}
impl Plugin {
    pub fn filename(&self) -> String {
        self.inner.get("filename").as_::<String>()
    }
}
impl Plugin {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl Plugin {
    pub fn item(&self, index: u32) -> MimeType {
        self.inner.call("item", &[index.into()]).as_::<MimeType>()
    }
}
impl Plugin {
    pub fn named_item(&self, name: &str) -> MimeType {
        self.inner
            .call("namedItem", &[name.into()])
            .as_::<MimeType>()
    }
}
