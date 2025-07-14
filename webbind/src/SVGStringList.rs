use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGStringList {
    inner: emlite::Val,
}
impl FromVal for SVGStringList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGStringList {
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
impl core::ops::Deref for SVGStringList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGStringList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGStringList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGStringList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGStringList> for emlite::Val {
    fn from(s: SVGStringList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGStringList);

impl SVGStringList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGStringList {
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGStringList {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl SVGStringList {
    pub fn initialize(&self, new_item: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl SVGStringList {
    pub fn get_item(&self, index: u32) -> jsbind::DOMString {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl SVGStringList {
    pub fn insert_item_before(&self, new_item: jsbind::DOMString, index: u32) -> jsbind::DOMString {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl SVGStringList {
    pub fn replace_item(&self, new_item: jsbind::DOMString, index: u32) -> jsbind::DOMString {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl SVGStringList {
    pub fn remove_item(&self, index: u32) -> jsbind::DOMString {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl SVGStringList {
    pub fn append_item(&self, new_item: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<jsbind::DOMString>()
    }
}
