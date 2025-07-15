use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSPageDescriptors {
    inner: CSSStyleDeclaration,
}

jsbind::utils::impl_dyn_cast!(CSSPageDescriptors);

impl AsRef<emlite::Val> for CSSPageDescriptors {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}

impl AsMut<emlite::Val> for CSSPageDescriptors {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
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
impl core::ops::Deref for CSSPageDescriptors {
    type Target = CSSStyleDeclaration;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSPageDescriptors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CSSPageDescriptors> for emlite::Val {
    fn from(x: CSSPageDescriptors) -> emlite::Val {
        let handle = x.inner.as_handle();
        core::mem::forget(x);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSPageDescriptors {
    pub fn margin(&self) -> CSSOMString {
        self.inner.get("margin").as_::<CSSOMString>()
    }

    pub fn set_margin(&mut self, value: CSSOMString) {
        self.inner.set("margin", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_top(&self) -> CSSOMString {
        self.inner.get("marginTop").as_::<CSSOMString>()
    }

    pub fn set_margin_top(&mut self, value: CSSOMString) {
        self.inner.set("marginTop", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_right(&self) -> CSSOMString {
        self.inner.get("marginRight").as_::<CSSOMString>()
    }

    pub fn set_margin_right(&mut self, value: CSSOMString) {
        self.inner.set("marginRight", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_bottom(&self) -> CSSOMString {
        self.inner.get("marginBottom").as_::<CSSOMString>()
    }

    pub fn set_margin_bottom(&mut self, value: CSSOMString) {
        self.inner.set("marginBottom", value);
    }
}
impl CSSPageDescriptors {
    pub fn margin_left(&self) -> CSSOMString {
        self.inner.get("marginLeft").as_::<CSSOMString>()
    }

    pub fn set_margin_left(&mut self, value: CSSOMString) {
        self.inner.set("marginLeft", value);
    }
}

impl CSSPageDescriptors {
    pub fn size(&self) -> CSSOMString {
        self.inner.get("size").as_::<CSSOMString>()
    }

    pub fn set_size(&mut self, value: CSSOMString) {
        self.inner.set("size", value);
    }
}
impl CSSPageDescriptors {
    pub fn page_orientation(&self) -> CSSOMString {
        self.inner
            .get("pageOrientation")
            .as_::<CSSOMString>()
    }

    pub fn set_page_orientation(&mut self, value: CSSOMString) {
        self.inner.set("pageOrientation", value);
    }
}

impl CSSPageDescriptors {
    pub fn marks(&self) -> CSSOMString {
        self.inner.get("marks").as_::<CSSOMString>()
    }

    pub fn set_marks(&mut self, value: CSSOMString) {
        self.inner.set("marks", value);
    }
}
impl CSSPageDescriptors {
    pub fn bleed(&self) -> CSSOMString {
        self.inner.get("bleed").as_::<CSSOMString>()
    }

    pub fn set_bleed(&mut self, value: CSSOMString) {
        self.inner.set("bleed", value);
    }
}
