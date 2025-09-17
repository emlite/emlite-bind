use super::*;

/// Callback interface NodeFilter
/// Generated wrapper for WebIDL `callback interface NodeFilter`
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NodeFilter {
    inner: Any,
}

impl FromVal for NodeFilter {
    fn from_val(v: &Any) -> Self {
        NodeFilter { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        NodeFilter {
            inner: Any::take_ownership(v),
        }
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for NodeFilter {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NodeFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl From<NodeFilter> for Any {
    fn from(s: NodeFilter) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NodeFilter> for Any {
    fn from(s: &NodeFilter) -> Any {
        s.inner.clone().into()
    }
}

impl NodeFilter {
    /// Wrap a JavaScript function as a NodeFilter
    pub fn from_function(f: &Function) -> NodeFilter {
        NodeFilter {
            inner: Any::from(f.clone()),
        }
    }
}

impl NodeFilter {
    /// Build a NodeFilter from a typed Rust closure matching `acceptNode`
    pub fn from_closure<F>(mut cb: F) -> NodeFilter
    where
        F: FnMut(Node) -> u16 + 'static,
    {
        let c = Closure::bind1(move |a1: Node| cb(a1));
        NodeFilter::from_function(&Function::from(&c))
    }
}

impl NodeFilter {
    pub fn accept_node(&self, node: &Node) -> u16 {
        if self.inner.is_function() {
            // Call as a bare function
            self.inner.invoke(&[node.into()]).as_::<u16>()
        } else {
            // Call the named method on the object
            self.inner.call("acceptNode", &[node.into()]).as_::<u16>()
        }
    }
}
