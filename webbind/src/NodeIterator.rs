use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NodeIterator {
    inner: emlite::Val,
}
impl FromVal for NodeIterator {
    fn from_val(v: &emlite::Val) -> Self {
        NodeIterator {
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
impl core::ops::Deref for NodeIterator {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NodeIterator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NodeIterator {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NodeIterator {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NodeIterator> for emlite::Val {
    fn from(s: NodeIterator) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&NodeIterator> for emlite::Val {
    fn from(s: &NodeIterator) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NodeIterator);

impl NodeIterator {
    pub fn root(&self) -> Node {
        self.inner.get("root").as_::<Node>()
    }
}
impl NodeIterator {
    pub fn reference_node(&self) -> Node {
        self.inner.get("referenceNode").as_::<Node>()
    }
}
impl NodeIterator {
    pub fn pointer_before_reference_node(&self) -> bool {
        self.inner.get("pointerBeforeReferenceNode").as_::<bool>()
    }
}
impl NodeIterator {
    pub fn what_to_show(&self) -> u32 {
        self.inner.get("whatToShow").as_::<u32>()
    }
}
impl NodeIterator {
    pub fn filter(&self) -> Function {
        self.inner.get("filter").as_::<Function>()
    }
}
impl NodeIterator {
    pub fn next_node(&self) -> Node {
        self.inner.call("nextNode", &[]).as_::<Node>()
    }
}
impl NodeIterator {
    pub fn previous_node(&self) -> Node {
        self.inner.call("previousNode", &[]).as_::<Node>()
    }
}
impl NodeIterator {
    pub fn detach(&self) -> Undefined {
        self.inner.call("detach", &[]).as_::<Undefined>()
    }
}
