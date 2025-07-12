use super::*;

#[derive(Clone, Debug)]
pub struct CSSPageDescriptors {
    inner: CSSStyleDeclaration,
}
impl FromVal for CSSPageDescriptors {
    fn from_val(v: &emlite::Val) -> Self {
        CSSPageDescriptors {
            inner: CSSStyleDeclaration::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CSSPageDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CSSPageDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPageDescriptors> for emlite::Val {
    fn from(x: CSSPageDescriptors) -> emlite::Val {
        let handle = x.inner.as_handle();
        std::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPageDescriptors {
    pub fn margin(&self) -> jsbind::CSSOMString {
        self.inner.get("margin").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("margin", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_top(&self) -> jsbind::CSSOMString {
        self.inner.get("marginTop").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_top(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_right(&self) -> jsbind::CSSOMString {
        self.inner.get("marginRight").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_right(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_bottom(&self) -> jsbind::CSSOMString {
        self.inner.get("marginBottom").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_bottom(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_left(&self) -> jsbind::CSSOMString {
        self.inner.get("marginLeft").as_::<jsbind::CSSOMString>()
    }

    pub fn set_margin_left(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marginLeft", value);
    }
}

impl CSSPageDescriptors {
    pub fn size(&self) -> jsbind::CSSOMString {
        self.inner.get("size").as_::<jsbind::CSSOMString>()
    }

    pub fn set_size(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("size", value);
    }
}
impl CSSPageDescriptors {
    pub fn page_orientation(&self) -> jsbind::CSSOMString {
        self.inner
            .get("pageOrientation")
            .as_::<jsbind::CSSOMString>()
    }

    pub fn set_page_orientation(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("pageOrientation", value);
    }
}

impl CSSPageDescriptors {
    pub fn marks(&self) -> jsbind::CSSOMString {
        self.inner.get("marks").as_::<jsbind::CSSOMString>()
    }

    pub fn set_marks(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("marks", value);
    }
}
impl CSSPageDescriptors {
    pub fn bleed(&self) -> jsbind::CSSOMString {
        self.inner.get("bleed").as_::<jsbind::CSSOMString>()
    }

    pub fn set_bleed(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("bleed", value);
    }
}
