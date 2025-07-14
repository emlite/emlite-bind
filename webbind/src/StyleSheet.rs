use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StyleSheet {
    inner: emlite::Val,
}
impl FromVal for StyleSheet {
    fn from_val(v: &emlite::Val) -> Self {
        StyleSheet {
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
impl core::ops::Deref for StyleSheet {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StyleSheet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StyleSheet {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StyleSheet {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StyleSheet> for emlite::Val {
    fn from(s: StyleSheet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(StyleSheet);

impl StyleSheet {
    pub fn type_(&self) -> jsbind::CSSOMString {
        self.inner.get("type").as_::<jsbind::CSSOMString>()
    }
}
impl StyleSheet {
    pub fn href(&self) -> jsbind::USVString {
        self.inner.get("href").as_::<jsbind::USVString>()
    }
}
impl StyleSheet {
    pub fn owner_node(&self) -> jsbind::Any {
        self.inner.get("ownerNode").as_::<jsbind::Any>()
    }
}
impl StyleSheet {
    pub fn parent_style_sheet(&self) -> CSSStyleSheet {
        self.inner.get("parentStyleSheet").as_::<CSSStyleSheet>()
    }
}
impl StyleSheet {
    pub fn title(&self) -> jsbind::DOMString {
        self.inner.get("title").as_::<jsbind::DOMString>()
    }
}
impl StyleSheet {
    pub fn media(&self) -> MediaList {
        self.inner.get("media").as_::<MediaList>()
    }
}
impl StyleSheet {
    pub fn disabled(&self) -> bool {
        self.inner.get("disabled").as_::<bool>()
    }

    pub fn set_disabled(&mut self, value: bool) {
        self.inner.set("disabled", value);
    }
}
