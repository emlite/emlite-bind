use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserverInit {
    inner: Any,
}
impl FromVal for MutationObserverInit {
    fn from_val(v: &Any) -> Self {
        MutationObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MutationObserverInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MutationObserverInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MutationObserverInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MutationObserverInit> for Any {
    fn from(s: MutationObserverInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MutationObserverInit> for Any {
    fn from(s: &MutationObserverInit) -> Any {
        s.inner.clone()
    }
}

impl MutationObserverInit {
    pub fn child_list(&self) -> bool {
        self.inner.get("childList").as_::<bool>()
    }

    pub fn set_child_list(&mut self, value: bool) {
        self.inner.set("childList", value);
    }
}
impl MutationObserverInit {
    pub fn attributes(&self) -> bool {
        self.inner.get("attributes").as_::<bool>()
    }

    pub fn set_attributes(&mut self, value: bool) {
        self.inner.set("attributes", value);
    }
}
impl MutationObserverInit {
    pub fn character_data(&self) -> bool {
        self.inner.get("characterData").as_::<bool>()
    }

    pub fn set_character_data(&mut self, value: bool) {
        self.inner.set("characterData", value);
    }
}
impl MutationObserverInit {
    pub fn subtree(&self) -> bool {
        self.inner.get("subtree").as_::<bool>()
    }

    pub fn set_subtree(&mut self, value: bool) {
        self.inner.set("subtree", value);
    }
}
impl MutationObserverInit {
    pub fn attribute_old_value(&self) -> bool {
        self.inner.get("attributeOldValue").as_::<bool>()
    }

    pub fn set_attribute_old_value(&mut self, value: bool) {
        self.inner.set("attributeOldValue", value);
    }
}
impl MutationObserverInit {
    pub fn character_data_old_value(&self) -> bool {
        self.inner.get("characterDataOldValue").as_::<bool>()
    }

    pub fn set_character_data_old_value(&mut self, value: bool) {
        self.inner.set("characterDataOldValue", value);
    }
}
impl MutationObserverInit {
    pub fn attribute_filter(&self) -> TypedArray<JsString> {
        self.inner
            .get("attributeFilter")
            .as_::<TypedArray<JsString>>()
    }

    pub fn set_attribute_filter(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("attributeFilter", value);
    }
}
/// The MutationObserver class.
/// [`MutationObserver`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserver {
    inner: Any,
}
impl FromVal for MutationObserver {
    fn from_val(v: &Any) -> Self {
        MutationObserver {
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
impl core::ops::Deref for MutationObserver {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MutationObserver {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MutationObserver {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MutationObserver> for Any {
    fn from(s: MutationObserver) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MutationObserver> for Any {
    fn from(s: &MutationObserver) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MutationObserver);

impl MutationObserver {
    /// The `new MutationObserver(..)` constructor, creating a new MutationObserver instance
    pub fn new(callback: &Function) -> MutationObserver {
        Self {
            inner: Any::global("MutationObserver")
                .new(&[callback.into()])
                .as_::<Any>(),
        }
    }
}
impl MutationObserver {
    /// The observe method.
    /// [`MutationObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    pub fn observe0(&self, target: &Node) -> Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<Undefined>()
    }
    /// The observe method.
    /// [`MutationObserver.observe`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/observe)
    pub fn observe1(&self, target: &Node, options: &MutationObserverInit) -> Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl MutationObserver {
    /// The disconnect method.
    /// [`MutationObserver.disconnect`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/disconnect)
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl MutationObserver {
    /// The takeRecords method.
    /// [`MutationObserver.takeRecords`](https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver/takeRecords)
    pub fn take_records(&self) -> TypedArray<MutationRecord> {
        self.inner
            .call("takeRecords", &[])
            .as_::<TypedArray<MutationRecord>>()
    }
}
