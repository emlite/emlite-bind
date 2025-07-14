use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClipboardItem {
    inner: emlite::Val,
}
impl FromVal for ClipboardItem {
    fn from_val(v: &emlite::Val) -> Self {
        ClipboardItem {
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
impl core::ops::Deref for ClipboardItem {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ClipboardItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ClipboardItem> for emlite::Val {
    fn from(s: ClipboardItem) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ClipboardItem {
    pub fn new0(items: jsbind::Record<jsbind::DOMString, jsbind::Any>) -> ClipboardItem {
        Self {
            inner: emlite::Val::global("ClipboardItem")
                .new(&[items.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(
        items: jsbind::Record<jsbind::DOMString, jsbind::Any>,
        options: jsbind::Any,
    ) -> ClipboardItem {
        Self {
            inner: emlite::Val::global("ClipboardItem")
                .new(&[items.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl ClipboardItem {
    pub fn presentation_style(&self) -> PresentationStyle {
        self.inner
            .get("presentationStyle")
            .as_::<PresentationStyle>()
    }
}
impl ClipboardItem {
    pub fn types(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("types")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl ClipboardItem {
    pub fn get_type(&self, type_: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("getType", &[type_.into()])
            .as_::<jsbind::Promise>()
    }
}
impl ClipboardItem {
    pub fn supports(type_: jsbind::DOMString) -> bool {
        emlite::Val::global("clipboarditem")
            .call("supports", &[type_.into()])
            .as_::<bool>()
    }
}
