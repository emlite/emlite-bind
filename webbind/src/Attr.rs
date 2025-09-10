use super::*;

/// The Attr class.
/// [`Attr`](https://developer.mozilla.org/en-US/docs/Web/API/Attr)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Attr {
    inner: Node,
}

impl FromVal for Attr {
    fn from_val(v: &Any) -> Self {
        Attr {
            inner: Node::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Attr {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Attr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Attr {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Attr {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Attr> for Any {
    fn from(s: Attr) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Attr> for Any {
    fn from(s: &Attr) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Attr);

impl Attr {
    /// Getter of the `namespaceURI` attribute.
    /// [`Attr.namespaceURI`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/namespaceURI)
    pub fn namespace_uri(&self) -> JsString {
        self.inner.get("namespaceURI").as_::<JsString>()
    }
}
impl Attr {
    /// Getter of the `prefix` attribute.
    /// [`Attr.prefix`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/prefix)
    pub fn prefix(&self) -> JsString {
        self.inner.get("prefix").as_::<JsString>()
    }
}
impl Attr {
    /// Getter of the `localName` attribute.
    /// [`Attr.localName`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/localName)
    pub fn local_name(&self) -> JsString {
        self.inner.get("localName").as_::<JsString>()
    }
}
impl Attr {
    /// Getter of the `name` attribute.
    /// [`Attr.name`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl Attr {
    /// Getter of the `value` attribute.
    /// [`Attr.value`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)
    pub fn value(&self) -> JsString {
        self.inner.get("value").as_::<JsString>()
    }

    /// Setter of the `value` attribute.
    /// [`Attr.value`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/value)
    pub fn set_value(&mut self, value: &JsString) {
        self.inner.set("value", value);
    }
}
impl Attr {
    /// Getter of the `ownerElement` attribute.
    /// [`Attr.ownerElement`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/ownerElement)
    pub fn owner_element(&self) -> Element {
        self.inner.get("ownerElement").as_::<Element>()
    }
}
impl Attr {
    /// Getter of the `specified` attribute.
    /// [`Attr.specified`](https://developer.mozilla.org/en-US/docs/Web/API/Attr/specified)
    pub fn specified(&self) -> bool {
        self.inner.get("specified").as_::<bool>()
    }
}
