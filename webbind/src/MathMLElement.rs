use super::*;

#[derive(Clone, Debug)]
pub struct MathMLElement {
    inner: Element,
}
impl FromVal for MathMLElement {
    fn from_val(v: &emlite::Val) -> Self {
        MathMLElement {
            inner: Element::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for MathMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MathMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MathMLElement> for emlite::Val {
    fn from(s: MathMLElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MathMLElement {
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
impl MathMLElement {
    pub fn onbeforexrselect(&self) -> jsbind::Any {
        self.inner.get("onbeforexrselect").as_::<jsbind::Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl MathMLElement {
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }
}
impl MathMLElement {
    pub fn nonce(&self) -> jsbind::DOMString {
        self.inner.get("nonce").as_::<jsbind::DOMString>()
    }

    pub fn set_nonce(&mut self, value: jsbind::DOMString) {
        self.inner.set("nonce", value);
    }
}
impl MathMLElement {
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl MathMLElement {
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl MathMLElement {
    pub fn focus0(&self) -> jsbind::Undefined {
        self.inner.call("focus", &[]).as_::<jsbind::Undefined>()
    }

    pub fn focus1(&self, options: FocusOptions) -> jsbind::Undefined {
        self.inner
            .call("focus", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MathMLElement {
    pub fn blur(&self) -> jsbind::Undefined {
        self.inner.call("blur", &[]).as_::<jsbind::Undefined>()
    }
}
