use super::*;




/// The CSSFontFeatureValuesMap class.
/// [`CSSFontFeatureValuesMap`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesMap)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFeatureValuesMap {
    inner: Any,
}

impl FromVal for CSSFontFeatureValuesMap {
    fn from_val(v: &Any) -> Self {
        CSSFontFeatureValuesMap { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSFontFeatureValuesMap {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSFontFeatureValuesMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSFontFeatureValuesMap {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSFontFeatureValuesMap {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSFontFeatureValuesMap> for Any {
    fn from(s: CSSFontFeatureValuesMap) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSFontFeatureValuesMap> for Any {
    fn from(s: &CSSFontFeatureValuesMap) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSFontFeatureValuesMap);


impl CSSFontFeatureValuesMap {
    /// The set method.
    /// [`CSSFontFeatureValuesMap.set`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesMap/set)
    pub fn set(&self, feature_value_name: &JsString, values: &Any) -> Undefined {
        self.inner.call("set", &[feature_value_name.into(), values.into(), ]).as_::<Undefined>()
    }
}
