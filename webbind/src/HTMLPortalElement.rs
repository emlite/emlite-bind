use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for PortalActivateOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PortalActivateOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for PortalActivateOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PortalActivateOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<PortalActivateOptions> for emlite::Val {
    fn from(s: PortalActivateOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl PortalActivateOptions {
    pub fn data(&self) -> Any {
        self.inner.get("data").as_::<Any>()
    }

    pub fn set_data(&mut self, value: Any) {
        self.inner.set("data", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLPortalElement {
    inner: HTMLElement,
}
impl FromVal for HTMLPortalElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLPortalElement { inner: HTMLElement::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLPortalElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLPortalElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLPortalElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLPortalElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLPortalElement> for emlite::Val {
    fn from(s: HTMLPortalElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLPortalElement);



impl HTMLPortalElement {
    pub fn new() -> HTMLPortalElement {
        Self {
            inner: emlite::Val::global("HTMLPortalElement").new(&[]).as_::<HTMLElement>(),
        }
    }

}
impl HTMLPortalElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }

}
impl HTMLPortalElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }

}
impl HTMLPortalElement {
    pub fn activate0(&self, ) -> Promise {
        self.inner.call("activate", &[]).as_::<Promise>()
    }

    pub fn activate1(&self, options: PortalActivateOptions) -> Promise {
        self.inner.call("activate", &[options.into(), ]).as_::<Promise>()
    }

}
impl HTMLPortalElement {
    pub fn post_message0(&self, message: Any) -> Undefined {
        self.inner.call("postMessage", &[message.into(), ]).as_::<Undefined>()
    }

    pub fn post_message1(&self, message: Any, options: StructuredSerializeOptions) -> Undefined {
        self.inner.call("postMessage", &[message.into(), options.into(), ]).as_::<Undefined>()
    }

}
impl HTMLPortalElement {
    pub fn onmessage(&self) -> Any {
        self.inner.get("onmessage").as_::<Any>()
    }

    pub fn set_onmessage(&mut self, value: Any) {
        self.inner.set("onmessage", value);
    }

}
impl HTMLPortalElement {
    pub fn onmessageerror(&self) -> Any {
        self.inner.get("onmessageerror").as_::<Any>()
    }

    pub fn set_onmessageerror(&mut self, value: Any) {
        self.inner.set("onmessageerror", value);
    }

}
