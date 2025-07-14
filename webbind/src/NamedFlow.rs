use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NamedFlow {
    inner: EventTarget,
}
impl FromVal for NamedFlow {
    fn from_val(v: &emlite::Val) -> Self {
        NamedFlow {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NamedFlow {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NamedFlow {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NamedFlow> for emlite::Val {
    fn from(s: NamedFlow) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NamedFlow {
    pub fn name(&self) -> jsbind::CSSOMString {
        self.inner.get("name").as_::<jsbind::CSSOMString>()
    }
}
impl NamedFlow {
    pub fn overset(&self) -> bool {
        self.inner.get("overset").as_::<bool>()
    }
}
impl NamedFlow {
    pub fn get_regions(&self) -> jsbind::Sequence<Element> {
        self.inner
            .call("getRegions", &[])
            .as_::<jsbind::Sequence<Element>>()
    }
}
impl NamedFlow {
    pub fn first_empty_region_index(&self) -> i16 {
        self.inner.get("firstEmptyRegionIndex").as_::<i16>()
    }
}
impl NamedFlow {
    pub fn get_content(&self) -> jsbind::Sequence<Node> {
        self.inner
            .call("getContent", &[])
            .as_::<jsbind::Sequence<Node>>()
    }
}
impl NamedFlow {
    pub fn get_regions_by_content(&self, node: Node) -> jsbind::Sequence<Element> {
        self.inner
            .call("getRegionsByContent", &[node.into()])
            .as_::<jsbind::Sequence<Element>>()
    }
}
