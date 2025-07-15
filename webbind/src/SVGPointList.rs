use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPointList {
    inner: emlite::Val,
}
impl FromVal for SVGPointList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGPointList {
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
impl core::ops::Deref for SVGPointList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPointList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGPointList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGPointList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGPointList> for emlite::Val {
    fn from(s: SVGPointList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGPointList);

impl SVGPointList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGPointList {
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGPointList {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGPointList {
    pub fn initialize(&self, new_item: DOMPoint) -> DOMPoint {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPointList {
    pub fn get_item(&self, index: u32) -> DOMPoint {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPointList {
    pub fn insert_item_before(&self, new_item: DOMPoint, index: u32) -> DOMPoint {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPointList {
    pub fn replace_item(&self, new_item: DOMPoint, index: u32) -> DOMPoint {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPointList {
    pub fn remove_item(&self, index: u32) -> DOMPoint {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<DOMPoint>()
    }
}
impl SVGPointList {
    pub fn append_item(&self, new_item: DOMPoint) -> DOMPoint {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<DOMPoint>()
    }
}
