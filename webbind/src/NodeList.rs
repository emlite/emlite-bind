use super::*;

/// The NodeList class.
/// [`NodeList`](https://developer.mozilla.org/en-US/docs/Web/API/NodeList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NodeList {
    inner: Any,
}
impl FromVal for NodeList {
    fn from_val(v: &Any) -> Self {
        NodeList {
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
impl core::ops::Deref for NodeList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NodeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for NodeList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for NodeList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<NodeList> for Any {
    fn from(s: NodeList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&NodeList> for Any {
    fn from(s: &NodeList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(NodeList);

impl NodeList {
    /// The item method.
    /// [`NodeList.item`](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/item)
    pub fn item(&self, index: u32) -> Node {
        self.inner.call("item", &[index.into()]).as_::<Node>()
    }
}
impl NodeList {
    /// Getter of the `length` attribute.
    /// [`NodeList.length`](https://developer.mozilla.org/en-US/docs/Web/API/NodeList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
