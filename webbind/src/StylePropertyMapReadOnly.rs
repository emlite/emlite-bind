use super::*;

#[derive(Clone, Debug)]
pub struct StylePropertyMapReadOnly {
    inner: emlite::Val,
}
impl FromVal for StylePropertyMapReadOnly {
    fn from_val(v: &emlite::Val) -> Self {
        StylePropertyMapReadOnly {
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
impl std::ops::Deref for StylePropertyMapReadOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for StylePropertyMapReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<StylePropertyMapReadOnly> for emlite::Val {
    fn from(s: StylePropertyMapReadOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl StylePropertyMapReadOnly {
    pub fn get(&self, property: jsbind::USVString) -> jsbind::Any {
        self.inner
            .call("get", &[property.into()])
            .as_::<jsbind::Any>()
    }
}
impl StylePropertyMapReadOnly {
    pub fn get_all(&self, property: jsbind::USVString) -> jsbind::Sequence<CSSStyleValue> {
        self.inner
            .call("getAll", &[property.into()])
            .as_::<jsbind::Sequence<CSSStyleValue>>()
    }
}
impl StylePropertyMapReadOnly {
    pub fn has(&self, property: jsbind::USVString) -> bool {
        self.inner.call("has", &[property.into()]).as_::<bool>()
    }
}
impl StylePropertyMapReadOnly {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
