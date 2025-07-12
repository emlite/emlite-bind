use super::*;

#[derive(Clone, Debug)]
pub struct PortalActivateOptions {
    inner: emlite::Val,
}
impl FromVal for PortalActivateOptions {
    fn from_val(v: &emlite::Val) -> Self {
        PortalActivateOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for PortalActivateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for PortalActivateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<PortalActivateOptions> for emlite::Val {
    fn from(s: PortalActivateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PortalActivateOptions {
    pub fn data(&self) -> jsbind::Any {
        self.inner.get("data").as_::<jsbind::Any>()
    }

    pub fn set_data(&mut self, value: jsbind::Any) {
        self.inner.set("data", value);
    }
}
#[derive(Clone, Debug)]
pub struct HTMLPortalElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPortalElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLPortalElement {
            inner: HTMLElement::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for HTMLPortalElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for HTMLPortalElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<HTMLPortalElement> for emlite::Val {
    fn from(s: HTMLPortalElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl HTMLPortalElement {
    pub fn new() -> HTMLPortalElement {
        Self {
            inner: emlite::Val::global("HTMLPortalElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLPortalElement {
    pub fn src(&self) -> jsbind::USVString {
        self.inner.get("src").as_::<jsbind::USVString>()
    }

    pub fn set_src(&mut self, value: jsbind::USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLPortalElement {
    pub fn referrer_policy(&self) -> jsbind::DOMString {
        self.inner.get("referrerPolicy").as_::<jsbind::DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: jsbind::DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLPortalElement {
    pub fn activate0(&self) -> jsbind::Promise {
        self.inner.call("activate", &[]).as_::<jsbind::Promise>()
    }

    pub fn activate1(&self, options: PortalActivateOptions) -> jsbind::Promise {
        self.inner
            .call("activate", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl HTMLPortalElement {
    pub fn post_message0(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn post_message1(
        &self,
        message: jsbind::Any,
        options: StructuredSerializeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl HTMLPortalElement {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl HTMLPortalElement {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}
