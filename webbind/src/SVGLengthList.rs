use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLengthList {
    inner: emlite::Val,
}
impl FromVal for SVGLengthList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGLengthList {
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
impl core::ops::Deref for SVGLengthList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGLengthList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGLengthList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGLengthList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SVGLengthList> for emlite::Val {
    fn from(s: SVGLengthList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGLengthList);

impl SVGLengthList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGLengthList {
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGLengthList {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl SVGLengthList {
    pub fn initialize(&self, new_item: SVGLength) -> SVGLength {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<SVGLength>()
    }
}
impl SVGLengthList {
    pub fn get_item(&self, index: u32) -> SVGLength {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<SVGLength>()
    }
}
impl SVGLengthList {
    pub fn insert_item_before(&self, new_item: SVGLength, index: u32) -> SVGLength {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<SVGLength>()
    }
}
impl SVGLengthList {
    pub fn replace_item(&self, new_item: SVGLength, index: u32) -> SVGLength {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<SVGLength>()
    }
}
impl SVGLengthList {
    pub fn remove_item(&self, index: u32) -> SVGLength {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<SVGLength>()
    }
}
impl SVGLengthList {
    pub fn append_item(&self, new_item: SVGLength) -> SVGLength {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<SVGLength>()
    }
}
