use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ShadowRoot {
    inner: DocumentFragment,
}
impl FromVal for ShadowRoot {
    fn from_val(v: &emlite::Val) -> Self {
        ShadowRoot {
            inner: DocumentFragment::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ShadowRoot {
    type Target = DocumentFragment;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ShadowRoot {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ShadowRoot {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ShadowRoot {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ShadowRoot> for emlite::Val {
    fn from(s: ShadowRoot) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(ShadowRoot);

impl ShadowRoot {
    pub fn mode(&self) -> ShadowRootMode {
        self.inner.get("mode").as_::<ShadowRootMode>()
    }
}
impl ShadowRoot {
    pub fn delegates_focus(&self) -> bool {
        self.inner.get("delegatesFocus").as_::<bool>()
    }
}
impl ShadowRoot {
    pub fn slot_assignment(&self) -> SlotAssignmentMode {
        self.inner.get("slotAssignment").as_::<SlotAssignmentMode>()
    }
}
impl ShadowRoot {
    pub fn clonable(&self) -> bool {
        self.inner.get("clonable").as_::<bool>()
    }
}
impl ShadowRoot {
    pub fn serializable(&self) -> bool {
        self.inner.get("serializable").as_::<bool>()
    }
}
impl ShadowRoot {
    pub fn host(&self) -> Element {
        self.inner.get("host").as_::<Element>()
    }
}
impl ShadowRoot {
    pub fn onslotchange(&self) -> jsbind::Any {
        self.inner.get("onslotchange").as_::<jsbind::Any>()
    }

    pub fn set_onslotchange(&mut self, value: jsbind::Any) {
        self.inner.set("onslotchange", value);
    }
}
impl ShadowRoot {
    pub fn set_html_unsafe(&self, html: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("setHTMLUnsafe", &[html.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl ShadowRoot {
    pub fn get_html0(&self) -> jsbind::DOMString {
        self.inner.call("getHTML", &[]).as_::<jsbind::DOMString>()
    }

    pub fn get_html1(&self, options: GetHTMLOptions) -> jsbind::DOMString {
        self.inner
            .call("getHTML", &[options.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl ShadowRoot {
    pub fn inner_html(&self) -> jsbind::Any {
        self.inner.get("innerHTML").as_::<jsbind::Any>()
    }

    pub fn set_inner_html(&mut self, value: jsbind::Any) {
        self.inner.set("innerHTML", value);
    }
}
impl ShadowRoot {
    pub fn get_animations(&self) -> jsbind::Sequence<Animation> {
        self.inner
            .call("getAnimations", &[])
            .as_::<jsbind::Sequence<Animation>>()
    }
}
