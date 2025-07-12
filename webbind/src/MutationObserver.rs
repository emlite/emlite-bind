use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MutationObserverInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MutationObserverInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MutationObserverInit> for emlite::Val {
    fn from(s: MutationObserverInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    pub fn attribute_filter(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("attributeFilter")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_attribute_filter(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("attributeFilter", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MutationObserver {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MutationObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MutationObserver> for emlite::Val {
    fn from(s: MutationObserver) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MutationObserver {
    pub fn new(callback: jsbind::Function) -> MutationObserver {
        Self {
            inner: emlite::Val::global("MutationObserver")
                .new(&[callback.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl MutationObserver {
    pub fn observe0(&self, target: Node) -> jsbind::Undefined {
        self.inner
            .call("observe", &[target.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn observe1(&self, target: Node, options: MutationObserverInit) -> jsbind::Undefined {
        self.inner
            .call("observe", &[target.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MutationObserver {
    pub fn disconnect(&self) -> jsbind::Undefined {
        self.inner
            .call("disconnect", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl MutationObserver {
    pub fn take_records(&self) -> jsbind::Sequence<MutationRecord> {
        self.inner
            .call("takeRecords", &[])
            .as_::<jsbind::Sequence<MutationRecord>>()
    }
}
