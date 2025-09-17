use super::*;

/// The NodeIterator class.
/// [`NodeIterator`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NodeIterator {
    inner: Any,
}

impl FromVal for NodeIterator {
    fn from_val(v: &Any) -> Self {
        NodeIterator {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NodeIterator {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NodeIterator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NodeIterator {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NodeIterator {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NodeIterator> for Any {
    fn from(s: NodeIterator) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NodeIterator> for Any {
    fn from(s: &NodeIterator) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NodeIterator);

impl NodeIterator {
    /// Getter of the `root` attribute.
    /// [`NodeIterator.root`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/root)
    pub fn root(&self) -> Node {
        self.inner.get("root").as_::<Node>()
    }
}
impl NodeIterator {
    /// Getter of the `referenceNode` attribute.
    /// [`NodeIterator.referenceNode`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/referenceNode)
    pub fn reference_node(&self) -> Node {
        self.inner.get("referenceNode").as_::<Node>()
    }
}
impl NodeIterator {
    /// Getter of the `pointerBeforeReferenceNode` attribute.
    /// [`NodeIterator.pointerBeforeReferenceNode`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/pointerBeforeReferenceNode)
    pub fn pointer_before_reference_node(&self) -> bool {
        self.inner.get("pointerBeforeReferenceNode").as_::<bool>()
    }
}
impl NodeIterator {
    /// Getter of the `whatToShow` attribute.
    /// [`NodeIterator.whatToShow`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/whatToShow)
    pub fn what_to_show(&self) -> u32 {
        self.inner.get("whatToShow").as_::<u32>()
    }
}
impl NodeIterator {
    /// Getter of the `filter` attribute.
    /// [`NodeIterator.filter`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/filter)
    pub fn filter(&self) -> NodeFilter {
        self.inner.get("filter").as_::<NodeFilter>()
    }
}
impl NodeIterator {
    /// The nextNode method.
    /// [`NodeIterator.nextNode`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/nextNode)
    pub fn next_node(&self) -> Node {
        self.inner.call("nextNode", &[]).as_::<Node>()
    }
}
impl NodeIterator {
    /// The previousNode method.
    /// [`NodeIterator.previousNode`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/previousNode)
    pub fn previous_node(&self) -> Node {
        self.inner.call("previousNode", &[]).as_::<Node>()
    }
}
impl NodeIterator {
    /// The detach method.
    /// [`NodeIterator.detach`](https://developer.mozilla.org/en-US/docs/Web/API/NodeIterator/detach)
    pub fn detach(&self) -> Undefined {
        self.inner.call("detach", &[]).as_::<Undefined>()
    }
}
