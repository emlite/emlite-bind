use super::*;

#[derive(Clone, Debug)]
pub struct FontData {
    inner: emlite::Val,
}
impl FromVal for FontData {
    fn from_val(v: &emlite::Val) -> Self {
        FontData {
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
impl std::ops::Deref for FontData {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for FontData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<FontData> for emlite::Val {
    fn from(s: FontData) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl FontData {
    pub fn blob(&self) -> jsbind::Promise {
        self.inner.call("blob", &[]).as_::<jsbind::Promise>()
    }
}
impl FontData {
    pub fn postscript_name(&self) -> jsbind::USVString {
        self.inner.get("postscriptName").as_::<jsbind::USVString>()
    }
}
impl FontData {
    pub fn full_name(&self) -> jsbind::USVString {
        self.inner.get("fullName").as_::<jsbind::USVString>()
    }
}
impl FontData {
    pub fn family(&self) -> jsbind::USVString {
        self.inner.get("family").as_::<jsbind::USVString>()
    }
}
impl FontData {
    pub fn style(&self) -> jsbind::USVString {
        self.inner.get("style").as_::<jsbind::USVString>()
    }
}
