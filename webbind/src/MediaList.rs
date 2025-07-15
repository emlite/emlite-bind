use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaList {
    inner: emlite::Val,
}
impl FromVal for MediaList {
    fn from_val(v: &emlite::Val) -> Self {
        MediaList {
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
impl core::ops::Deref for MediaList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaList> for emlite::Val {
    fn from(s: MediaList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaList> for emlite::Val {
    fn from(s: &MediaList) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaList);

impl MediaList {
    pub fn media_text(&self) -> CSSOMString {
        self.inner.get("mediaText").as_::<CSSOMString>()
    }

    pub fn set_media_text(&mut self, value: CSSOMString) {
        self.inner.set("mediaText", value);
    }
}
impl MediaList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl MediaList {
    pub fn item(&self, index: u32) -> CSSOMString {
        self.inner
            .call("item", &[index.into()])
            .as_::<CSSOMString>()
    }
}
impl MediaList {
    pub fn append_medium(&self, medium: CSSOMString) -> Undefined {
        self.inner
            .call("appendMedium", &[medium.into()])
            .as_::<Undefined>()
    }
}
impl MediaList {
    pub fn delete_medium(&self, medium: CSSOMString) -> Undefined {
        self.inner
            .call("deleteMedium", &[medium.into()])
            .as_::<Undefined>()
    }
}
