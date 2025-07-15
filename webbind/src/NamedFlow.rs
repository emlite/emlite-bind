use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NamedFlow {
    inner: EventTarget,
}
impl FromVal for NamedFlow {
    fn from_val(v: &emlite::Val) -> Self {
        NamedFlow { inner: EventTarget::from_val(v) }
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
impl AsRef<emlite::Val> for NamedFlow {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NamedFlow {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(NamedFlow);


impl NamedFlow {
    pub fn name(&self) -> CSSOMString {
        self.inner.get("name").as_::<CSSOMString>()
    }

}
impl NamedFlow {
    pub fn overset(&self) -> bool {
        self.inner.get("overset").as_::<bool>()
    }

}
impl NamedFlow {
    pub fn get_regions(&self, ) -> Sequence<Element> {
        self.inner.call("getRegions", &[]).as_::<Sequence<Element>>()
    }

}
impl NamedFlow {
    pub fn first_empty_region_index(&self) -> i16 {
        self.inner.get("firstEmptyRegionIndex").as_::<i16>()
    }

}
impl NamedFlow {
    pub fn get_content(&self, ) -> Sequence<Node> {
        self.inner.call("getContent", &[]).as_::<Sequence<Node>>()
    }

}
impl NamedFlow {
    pub fn get_regions_by_content(&self, node: Node) -> Sequence<Element> {
        self.inner.call("getRegionsByContent", &[node.into(), ]).as_::<Sequence<Element>>()
    }

}
