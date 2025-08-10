use super::*;

/// The NamedNodeMap class.
/// [`NamedNodeMap`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NamedNodeMap {
    inner: Any,
}

impl FromVal for NamedNodeMap {
    fn from_val(v: &Any) -> Self {
        NamedNodeMap {
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

impl core::ops::Deref for NamedNodeMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for NamedNodeMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for NamedNodeMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for NamedNodeMap {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<NamedNodeMap> for Any {
    fn from(s: NamedNodeMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&NamedNodeMap> for Any {
    fn from(s: &NamedNodeMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(NamedNodeMap);

impl NamedNodeMap {
    /// Getter of the `length` attribute.
    /// [`NamedNodeMap.length`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl NamedNodeMap {
    /// The item method.
    /// [`NamedNodeMap.item`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/item)
    pub fn item(&self, index: u32) -> Attr {
        self.inner.call("item", &[index.into()]).as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The getNamedItem method.
    /// [`NamedNodeMap.getNamedItem`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItem)
    pub fn get_named_item(&self, qualified_name: &JsString) -> Attr {
        self.inner
            .call("getNamedItem", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The getNamedItemNS method.
    /// [`NamedNodeMap.getNamedItemNS`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/getNamedItemNS)
    pub fn get_named_item_ns(&self, namespace: &JsString, local_name: &JsString) -> Attr {
        self.inner
            .call("getNamedItemNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The setNamedItem method.
    /// [`NamedNodeMap.setNamedItem`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItem)
    pub fn set_named_item(&self, attr: &Attr) -> Attr {
        self.inner
            .call("setNamedItem", &[attr.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The setNamedItemNS method.
    /// [`NamedNodeMap.setNamedItemNS`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/setNamedItemNS)
    pub fn set_named_item_ns(&self, attr: &Attr) -> Attr {
        self.inner
            .call("setNamedItemNS", &[attr.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The removeNamedItem method.
    /// [`NamedNodeMap.removeNamedItem`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItem)
    pub fn remove_named_item(&self, qualified_name: &JsString) -> Attr {
        self.inner
            .call("removeNamedItem", &[qualified_name.into()])
            .as_::<Attr>()
    }
}
impl NamedNodeMap {
    /// The removeNamedItemNS method.
    /// [`NamedNodeMap.removeNamedItemNS`](https://developer.mozilla.org/en-US/docs/Web/API/NamedNodeMap/removeNamedItemNS)
    pub fn remove_named_item_ns(&self, namespace: &JsString, local_name: &JsString) -> Attr {
        self.inner
            .call("removeNamedItemNS", &[namespace.into(), local_name.into()])
            .as_::<Attr>()
    }
}
