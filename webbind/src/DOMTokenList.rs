use super::*;

#[derive(Clone, Debug)]
pub struct DOMTokenList {
    inner: emlite::Val,
}
impl FromVal for DOMTokenList {
    fn from_val(v: &emlite::Val) -> Self {
        DOMTokenList {
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
impl std::ops::Deref for DOMTokenList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DOMTokenList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DOMTokenList> for emlite::Val {
    fn from(s: DOMTokenList) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DOMTokenList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMTokenList {
    pub fn item(&self, index: u32) -> jsbind::DOMString {
        self.inner
            .call("item", &[index.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl DOMTokenList {
    pub fn contains(&self, token: jsbind::DOMString) -> bool {
        self.inner.call("contains", &[token.into()]).as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn add(&self, tokens: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("add", &[tokens.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DOMTokenList {
    pub fn remove(&self, tokens: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("remove", &[tokens.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DOMTokenList {
    pub fn toggle0(&self, token: jsbind::DOMString) -> bool {
        self.inner.call("toggle", &[token.into()]).as_::<bool>()
    }

    pub fn toggle1(&self, token: jsbind::DOMString, force: bool) -> bool {
        self.inner
            .call("toggle", &[token.into(), force.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn replace(&self, token: jsbind::DOMString, new_token: jsbind::DOMString) -> bool {
        self.inner
            .call("replace", &[token.into(), new_token.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn supports(&self, token: jsbind::DOMString) -> bool {
        self.inner.call("supports", &[token.into()]).as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }

    pub fn set_value(&mut self, value: jsbind::DOMString) {
        self.inner.set("value", value);
    }
}
