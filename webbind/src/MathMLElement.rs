use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MathMLElement {
    inner: Element,
}
impl FromVal for MathMLElement {
    fn from_val(v: &emlite::Val) -> Self {
        MathMLElement { inner: Element::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MathMLElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MathMLElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MathMLElement {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MathMLElement {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<MathMLElement> for emlite::Val {
    fn from(s: MathMLElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MathMLElement);


impl MathMLElement {
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }

}
impl MathMLElement {
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: Any) {
        self.inner.set("onbeforexrselect", value);
    }

}
impl MathMLElement {
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }

}
impl MathMLElement {
    pub fn nonce(&self) -> DOMString {
        self.inner.get("nonce").as_::<DOMString>()
    }

    pub fn set_nonce(&mut self, value: DOMString) {
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
    pub fn focus0(&self, ) -> Undefined {
        self.inner.call("focus", &[]).as_::<Undefined>()
    }

    pub fn focus1(&self, options: FocusOptions) -> Undefined {
        self.inner.call("focus", &[options.into(), ]).as_::<Undefined>()
    }

}
impl MathMLElement {
    pub fn blur(&self, ) -> Undefined {
        self.inner.call("blur", &[]).as_::<Undefined>()
    }

}
