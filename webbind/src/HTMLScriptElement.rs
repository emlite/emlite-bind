use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLScriptElement {
    inner: HTMLElement,
}
impl FromVal for HTMLScriptElement {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLScriptElement {
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
impl core::ops::Deref for HTMLScriptElement {
    type Target = HTMLElement;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLScriptElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLScriptElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLScriptElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLScriptElement> for emlite::Val {
    fn from(s: HTMLScriptElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLScriptElement);

impl HTMLScriptElement {
    pub fn new() -> HTMLScriptElement {
        Self {
            inner: emlite::Val::global("HTMLScriptElement")
                .new(&[])
                .as_::<HTMLElement>(),
        }
    }
}
impl HTMLScriptElement {
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    pub fn set_type_(&mut self, value: DOMString) {
        self.inner.set("type", value);
    }
}
impl HTMLScriptElement {
    pub fn src(&self) -> USVString {
        self.inner.get("src").as_::<USVString>()
    }

    pub fn set_src(&mut self, value: USVString) {
        self.inner.set("src", value);
    }
}
impl HTMLScriptElement {
    pub fn no_module(&self) -> bool {
        self.inner.get("noModule").as_::<bool>()
    }

    pub fn set_no_module(&mut self, value: bool) {
        self.inner.set("noModule", value);
    }
}
impl HTMLScriptElement {
    pub fn async_(&self) -> bool {
        self.inner.get("async").as_::<bool>()
    }

    pub fn set_async_(&mut self, value: bool) {
        self.inner.set("async", value);
    }
}
impl HTMLScriptElement {
    pub fn defer(&self) -> bool {
        self.inner.get("defer").as_::<bool>()
    }

    pub fn set_defer(&mut self, value: bool) {
        self.inner.set("defer", value);
    }
}
impl HTMLScriptElement {
    pub fn blocking(&self) -> DOMTokenList {
        self.inner.get("blocking").as_::<DOMTokenList>()
    }
}
impl HTMLScriptElement {
    pub fn cross_origin(&self) -> DOMString {
        self.inner.get("crossOrigin").as_::<DOMString>()
    }

    pub fn set_cross_origin(&mut self, value: DOMString) {
        self.inner.set("crossOrigin", value);
    }
}
impl HTMLScriptElement {
    pub fn referrer_policy(&self) -> DOMString {
        self.inner.get("referrerPolicy").as_::<DOMString>()
    }

    pub fn set_referrer_policy(&mut self, value: DOMString) {
        self.inner.set("referrerPolicy", value);
    }
}
impl HTMLScriptElement {
    pub fn integrity(&self) -> DOMString {
        self.inner.get("integrity").as_::<DOMString>()
    }

    pub fn set_integrity(&mut self, value: DOMString) {
        self.inner.set("integrity", value);
    }
}
impl HTMLScriptElement {
    pub fn fetch_priority(&self) -> DOMString {
        self.inner.get("fetchPriority").as_::<DOMString>()
    }

    pub fn set_fetch_priority(&mut self, value: DOMString) {
        self.inner.set("fetchPriority", value);
    }
}
impl HTMLScriptElement {
    pub fn text(&self) -> DOMString {
        self.inner.get("text").as_::<DOMString>()
    }

    pub fn set_text(&mut self, value: DOMString) {
        self.inner.set("text", value);
    }
}
impl HTMLScriptElement {
    pub fn supports(type_: DOMString) -> bool {
        emlite::Val::global("HTMLScriptElement")
            .call("supports", &[type_.into()])
            .as_::<bool>()
    }
}
impl HTMLScriptElement {
    pub fn charset(&self) -> DOMString {
        self.inner.get("charset").as_::<DOMString>()
    }

    pub fn set_charset(&mut self, value: DOMString) {
        self.inner.set("charset", value);
    }
}
impl HTMLScriptElement {
    pub fn event(&self) -> DOMString {
        self.inner.get("event").as_::<DOMString>()
    }

    pub fn set_event(&mut self, value: DOMString) {
        self.inner.set("event", value);
    }
}
impl HTMLScriptElement {
    pub fn html_for(&self) -> DOMString {
        self.inner.get("htmlFor").as_::<DOMString>()
    }

    pub fn set_html_for(&mut self, value: DOMString) {
        self.inner.set("htmlFor", value);
    }
}
impl HTMLScriptElement {
    pub fn attribution_src(&self) -> USVString {
        self.inner.get("attributionSrc").as_::<USVString>()
    }

    pub fn set_attribution_src(&mut self, value: USVString) {
        self.inner.set("attributionSrc", value);
    }
}
