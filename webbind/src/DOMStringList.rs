use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMStringList {
    inner: emlite::Val,
}
impl FromVal for DOMStringList {
    fn from_val(v: &emlite::Val) -> Self {
        DOMStringList {
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
impl core::ops::Deref for DOMStringList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMStringList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMStringList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMStringList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMStringList> for emlite::Val {
    fn from(s: DOMStringList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMStringList);

impl DOMStringList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMStringList {
    pub fn item(&self, index: u32) -> jsbind::DOMString {
        self.inner
            .call("item", &[index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl DOMStringList {
    pub fn contains(&self, string: jsbind::DOMString) -> bool {
        self.inner.call("contains", &[string.into()]).as_::<bool>()
    }
}
