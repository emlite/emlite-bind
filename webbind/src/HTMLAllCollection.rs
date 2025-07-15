use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLAllCollection {
    inner: emlite::Val,
}
impl FromVal for HTMLAllCollection {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLAllCollection {
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
impl core::ops::Deref for HTMLAllCollection {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLAllCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLAllCollection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLAllCollection {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<HTMLAllCollection> for emlite::Val {
    fn from(s: HTMLAllCollection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&HTMLAllCollection> for emlite::Val {
    fn from(s: &HTMLAllCollection) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(HTMLAllCollection);

impl HTMLAllCollection {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl HTMLAllCollection {
    pub fn named_item(&self, name: &str) -> Any {
        self.inner.call("namedItem", &[name.into()]).as_::<Any>()
    }
}
impl HTMLAllCollection {
    pub fn item0(&self) -> Any {
        self.inner.call("item", &[]).as_::<Any>()
    }

    pub fn item1(&self, name_or_index: &str) -> Any {
        self.inner
            .call("item", &[name_or_index.into()])
            .as_::<Any>()
    }
}
