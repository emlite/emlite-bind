use super::*;

/// The MutationRecord class.
/// [`MutationRecord`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationRecord {
    inner: Any,
}
impl FromVal for MutationRecord {
    fn from_val(v: &Any) -> Self {
        MutationRecord {
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
impl core::ops::Deref for MutationRecord {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MutationRecord {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MutationRecord {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MutationRecord> for Any {
    fn from(s: MutationRecord) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MutationRecord> for Any {
    fn from(s: &MutationRecord) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MutationRecord);

impl MutationRecord {
    /// Getter of the `type` attribute.
    /// [`MutationRecord.type`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/type)
    pub fn type_(&self) -> JsString {
        self.inner.get("type").as_::<JsString>()
    }
}
impl MutationRecord {
    /// Getter of the `target` attribute.
    /// [`MutationRecord.target`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/target)
    pub fn target(&self) -> Node {
        self.inner.get("target").as_::<Node>()
    }
}
impl MutationRecord {
    /// Getter of the `addedNodes` attribute.
    /// [`MutationRecord.addedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/addedNodes)
    pub fn added_nodes(&self) -> NodeList {
        self.inner.get("addedNodes").as_::<NodeList>()
    }
}
impl MutationRecord {
    /// Getter of the `removedNodes` attribute.
    /// [`MutationRecord.removedNodes`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/removedNodes)
    pub fn removed_nodes(&self) -> NodeList {
        self.inner.get("removedNodes").as_::<NodeList>()
    }
}
impl MutationRecord {
    /// Getter of the `previousSibling` attribute.
    /// [`MutationRecord.previousSibling`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/previousSibling)
    pub fn previous_sibling(&self) -> Node {
        self.inner.get("previousSibling").as_::<Node>()
    }
}
impl MutationRecord {
    /// Getter of the `nextSibling` attribute.
    /// [`MutationRecord.nextSibling`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/nextSibling)
    pub fn next_sibling(&self) -> Node {
        self.inner.get("nextSibling").as_::<Node>()
    }
}
impl MutationRecord {
    /// Getter of the `attributeName` attribute.
    /// [`MutationRecord.attributeName`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeName)
    pub fn attribute_name(&self) -> JsString {
        self.inner.get("attributeName").as_::<JsString>()
    }
}
impl MutationRecord {
    /// Getter of the `attributeNamespace` attribute.
    /// [`MutationRecord.attributeNamespace`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/attributeNamespace)
    pub fn attribute_namespace(&self) -> JsString {
        self.inner.get("attributeNamespace").as_::<JsString>()
    }
}
impl MutationRecord {
    /// Getter of the `oldValue` attribute.
    /// [`MutationRecord.oldValue`](https://developer.mozilla.org/en-US/docs/Web/API/MutationRecord/oldValue)
    pub fn old_value(&self) -> JsString {
        self.inner.get("oldValue").as_::<JsString>()
    }
}
