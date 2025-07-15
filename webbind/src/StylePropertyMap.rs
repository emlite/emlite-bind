use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct StylePropertyMap {
    inner: StylePropertyMapReadOnly,
}
impl FromVal for StylePropertyMap {
    fn from_val(v: &emlite::Val) -> Self {
        StylePropertyMap {
            inner: StylePropertyMapReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for StylePropertyMap {
    type Target = StylePropertyMapReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for StylePropertyMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for StylePropertyMap {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for StylePropertyMap {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<StylePropertyMap> for emlite::Val {
    fn from(s: StylePropertyMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(StylePropertyMap);

impl StylePropertyMap {
    pub fn set(&self, property: USVString, values: Any) -> Undefined {
        self.inner
            .call("set", &[property.into(), values.into()])
            .as_::<Undefined>()
    }
}
impl StylePropertyMap {
    pub fn append(&self, property: USVString, values: Any) -> Undefined {
        self.inner
            .call("append", &[property.into(), values.into()])
            .as_::<Undefined>()
    }
}
impl StylePropertyMap {
    pub fn delete(&self, property: USVString) -> Undefined {
        self.inner
            .call("delete", &[property.into()])
            .as_::<Undefined>()
    }
}
impl StylePropertyMap {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
