use super::*;

#[derive(Clone, Debug)]
pub struct SVGNumberList {
    inner: emlite::Val,
}
impl FromVal for SVGNumberList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGNumberList {
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
impl std::ops::Deref for SVGNumberList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGNumberList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGNumberList> for emlite::Val {
    fn from(s: SVGNumberList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGNumberList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGNumberList {
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGNumberList {
    pub fn clear(&self) -> jsbind::Undefined {
        self.inner.call("clear", &[]).as_::<jsbind::Undefined>()
    }
}
impl SVGNumberList {
    pub fn initialize(&self, new_item: SVGNumber) -> SVGNumber {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    pub fn get_item(&self, index: u32) -> SVGNumber {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    pub fn insert_item_before(&self, new_item: SVGNumber, index: u32) -> SVGNumber {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    pub fn replace_item(&self, new_item: SVGNumber, index: u32) -> SVGNumber {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    pub fn remove_item(&self, index: u32) -> SVGNumber {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    pub fn append_item(&self, new_item: SVGNumber) -> SVGNumber {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<SVGNumber>()
    }
}
