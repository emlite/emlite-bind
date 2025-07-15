use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFeatureValuesMap {
    inner: emlite::Val,
}
impl FromVal for CSSFontFeatureValuesMap {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontFeatureValuesMap {
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
impl core::ops::Deref for CSSFontFeatureValuesMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSFontFeatureValuesMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CSSFontFeatureValuesMap {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSFontFeatureValuesMap {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSFontFeatureValuesMap> for emlite::Val {
    fn from(s: CSSFontFeatureValuesMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(CSSFontFeatureValuesMap);

impl CSSFontFeatureValuesMap {
    pub fn set(&self, feature_value_name: CSSOMString, values: Any) -> Undefined {
        self.inner
            .call("set", &[feature_value_name.into(), values.into()])
            .as_::<Undefined>()
    }
}
