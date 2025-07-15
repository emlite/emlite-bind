use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl AsRef<emlite::Val> for ClipboardItem {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ClipboardItem {
    fn as_mut(&mut self) -> &mut emlite::Val {
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
jsbind::utils::impl_dyn_cast!(ClipboardItem);

impl ClipboardItem {
    pub fn new0(items: Record<DOMString, Any>) -> ClipboardItem {
        Self {
            inner: emlite::Val::global("ClipboardItem")
                .new(&[items.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(items: Record<DOMString, Any>, options: Any) -> ClipboardItem {
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
    pub fn types(&self) -> FrozenArray<DOMString> {
        self.inner.get("types").as_::<FrozenArray<DOMString>>()
    }
}
impl ClipboardItem {
    pub fn get_type(&self, type_: DOMString) -> Promise {
        self.inner.call("getType", &[type_.into()]).as_::<Promise>()
    }
}
impl ClipboardItem {
    pub fn supports(type_: DOMString) -> bool {
        emlite::Val::global("ClipboardItem")
            .call("supports", &[type_.into()])
            .as_::<bool>()
    }
}
