use super::*;

#[derive(Clone, Debug)]
pub struct FocusOptions {
    inner: emlite::Val,
}
impl FromVal for FocusOptions {
    fn from_val(v: &emlite::Val) -> Self {
        FocusOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for FocusOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FocusOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FocusOptions> for emlite::Val {
    fn from(s: FocusOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FocusOptions {
    pub fn prevent_scroll(&self) -> bool {
        self.inner.get("preventScroll").as_::<bool>()
    }

    pub fn set_prevent_scroll(&mut self, value: bool) {
        self.inner.set("preventScroll", value);
    }
}
impl FocusOptions {
    pub fn focus_visible(&self) -> bool {
        self.inner.get("focusVisible").as_::<bool>()
    }

    pub fn set_focus_visible(&mut self, value: bool) {
        self.inner.set("focusVisible", value);
    }
}
#[derive(Clone, Debug)]
pub struct SVGElement {
    inner: Element,
}
impl FromVal for SVGElement {
    fn from_val(v: &emlite::Val) -> Self {
        SVGElement {
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
impl std::ops::Deref for SVGElement {
    type Target = Element;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for SVGElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<SVGElement> for emlite::Val {
    fn from(s: SVGElement) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SVGElement {
    pub fn class_name(&self) -> SVGAnimatedString {
        self.inner.get("className").as_::<SVGAnimatedString>()
    }
}
impl SVGElement {
    pub fn owner_svg_element(&self) -> SVGSVGElement {
        self.inner.get("ownerSVGElement").as_::<SVGSVGElement>()
    }
}
impl SVGElement {
    pub fn viewport_element(&self) -> SVGElement {
        self.inner.get("viewportElement").as_::<SVGElement>()
    }
}
impl SVGElement {
    pub fn onbeforexrselect(&self) -> jsbind::Any {
        self.inner.get("onbeforexrselect").as_::<jsbind::Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl SVGElement {
    pub fn corresponding_element(&self) -> SVGElement {
        self.inner.get("correspondingElement").as_::<SVGElement>()
    }
}
impl SVGElement {
    pub fn corresponding_use_element(&self) -> SVGUseElement {
        self.inner
            .get("correspondingUseElement")
            .as_::<SVGUseElement>()
    }
}
impl SVGElement {
    pub fn dataset(&self) -> DOMStringMap {
        self.inner.get("dataset").as_::<DOMStringMap>()
    }
}
impl SVGElement {
    pub fn nonce(&self) -> jsbind::DOMString {
        self.inner.get("nonce").as_::<jsbind::DOMString>()
    }

    pub fn set_nonce(&mut self, value: jsbind::DOMString) {
        self.inner.set("nonce", value);
    }
}
impl SVGElement {
    pub fn autofocus(&self) -> bool {
        self.inner.get("autofocus").as_::<bool>()
    }

    pub fn set_autofocus(&mut self, value: bool) {
        self.inner.set("autofocus", value);
    }
}
impl SVGElement {
    pub fn tab_index(&self) -> i32 {
        self.inner.get("tabIndex").as_::<i32>()
    }

    pub fn set_tab_index(&mut self, value: i32) {
        self.inner.set("tabIndex", value);
    }
}
impl SVGElement {
    pub fn focus0(&self) -> jsbind::Undefined {
        self.inner.call("focus", &[]).as_::<jsbind::Undefined>()
    }

    pub fn focus1(&self, options: FocusOptions) -> jsbind::Undefined {
        self.inner
            .call("focus", &[options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl SVGElement {
    pub fn blur(&self) -> jsbind::Undefined {
        self.inner.call("blur", &[]).as_::<jsbind::Undefined>()
    }
}
impl SVGElement {
    pub fn style(&self) -> CSSStyleDeclaration {
        self.inner.get("style").as_::<CSSStyleDeclaration>()
    }
}
