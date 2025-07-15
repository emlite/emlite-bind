use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
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
impl core::ops::Deref for DOMTokenList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMTokenList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMTokenList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMTokenList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMTokenList> for emlite::Val {
    fn from(s: DOMTokenList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DOMTokenList> for emlite::Val {
    fn from(s: &DOMTokenList) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMTokenList);

impl DOMTokenList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DOMTokenList {
    pub fn item(&self, index: u32) -> String {
        self.inner.call("item", &[index.into()]).as_::<String>()
    }
}
impl DOMTokenList {
    pub fn contains(&self, token: &str) -> bool {
        self.inner.call("contains", &[token.into()]).as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn add(&self, tokens: &str) -> Undefined {
        self.inner.call("add", &[tokens.into()]).as_::<Undefined>()
    }
}
impl DOMTokenList {
    pub fn remove(&self, tokens: &str) -> Undefined {
        self.inner
            .call("remove", &[tokens.into()])
            .as_::<Undefined>()
    }
}
impl DOMTokenList {
    pub fn toggle0(&self, token: &str) -> bool {
        self.inner.call("toggle", &[token.into()]).as_::<bool>()
    }

    pub fn toggle1(&self, token: &str, force: bool) -> bool {
        self.inner
            .call("toggle", &[token.into(), force.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn replace(&self, token: &str, new_token: &str) -> bool {
        self.inner
            .call("replace", &[token.into(), new_token.into()])
            .as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn supports(&self, token: &str) -> bool {
        self.inner.call("supports", &[token.into()]).as_::<bool>()
    }
}
impl DOMTokenList {
    pub fn value(&self) -> String {
        self.inner.get("value").as_::<String>()
    }

    pub fn set_value(&mut self, value: &str) {
        self.inner.set("value", value);
    }
}
