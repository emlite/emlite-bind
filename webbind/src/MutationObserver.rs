use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserverInit {
    inner: emlite::Val,
}
impl FromVal for MutationObserverInit {
    fn from_val(v: &emlite::Val) -> Self {
        MutationObserverInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MutationObserverInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MutationObserverInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MutationObserverInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MutationObserverInit> for emlite::Val {
    fn from(s: MutationObserverInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MutationObserverInit> for emlite::Val {
    fn from(s: &MutationObserverInit) -> emlite::Val {
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
    pub fn attribute_filter(&self) -> Sequence<DOMString> {
        self.inner
            .get("attributeFilter")
            .as_::<Sequence<DOMString>>()
    }

    pub fn set_attribute_filter(&mut self, value: Sequence<DOMString>) {
        self.inner.set("attributeFilter", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MutationObserver {
    inner: emlite::Val,
}
impl FromVal for MutationObserver {
    fn from_val(v: &emlite::Val) -> Self {
        MutationObserver {
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
impl core::ops::Deref for MutationObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MutationObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MutationObserver {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MutationObserver {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MutationObserver> for emlite::Val {
    fn from(s: MutationObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MutationObserver> for emlite::Val {
    fn from(s: &MutationObserver) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MutationObserver);

impl MutationObserver {
    pub fn new(callback: Function) -> MutationObserver {
        Self {
            inner: emlite::Val::global("MutationObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl MutationObserver {
    pub fn observe0(&self, target: Node) -> Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<Undefined>()
    }

    pub fn observe1(&self, target: Node, options: MutationObserverInit) -> Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl MutationObserver {
    pub fn disconnect(&self) -> Undefined {
        self.inner.call("disconnect", &[]).as_::<Undefined>()
    }
}
impl MutationObserver {
    pub fn take_records(&self) -> Sequence<MutationRecord> {
        self.inner
            .call("takeRecords", &[])
            .as_::<Sequence<MutationRecord>>()
    }
}
