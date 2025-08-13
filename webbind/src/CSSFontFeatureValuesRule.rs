use super::*;




/// The CSSFontFeatureValuesRule class.
/// [`CSSFontFeatureValuesRule`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSFontFeatureValuesRule {
    inner: CSSRule,
}

impl FromVal for CSSFontFeatureValuesRule {
    fn from_val(v: &Any) -> Self {
        CSSFontFeatureValuesRule { inner: CSSRule::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CSSFontFeatureValuesRule {
    type Target = CSSRule;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CSSFontFeatureValuesRule {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CSSFontFeatureValuesRule {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CSSFontFeatureValuesRule {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CSSFontFeatureValuesRule> for Any {
    fn from(s: CSSFontFeatureValuesRule) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CSSFontFeatureValuesRule> for Any {
    fn from(s: &CSSFontFeatureValuesRule) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CSSFontFeatureValuesRule);


impl CSSFontFeatureValuesRule {
    /// Getter of the `fontFamily` attribute.
    /// [`CSSFontFeatureValuesRule.fontFamily`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)
    pub fn font_family(&self) -> JsString {
        self.inner.get("fontFamily").as_::<JsString>()
    }

    /// Setter of the `fontFamily` attribute.
    /// [`CSSFontFeatureValuesRule.fontFamily`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/fontFamily)
    pub fn set_font_family(&mut self, value: &JsString) {
        self.inner.set("fontFamily", value);
    }
}
impl CSSFontFeatureValuesRule {
    /// Getter of the `annotation` attribute.
    /// [`CSSFontFeatureValuesRule.annotation`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/annotation)
    pub fn annotation(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("annotation").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `ornaments` attribute.
    /// [`CSSFontFeatureValuesRule.ornaments`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/ornaments)
    pub fn ornaments(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("ornaments").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `stylistic` attribute.
    /// [`CSSFontFeatureValuesRule.stylistic`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/stylistic)
    pub fn stylistic(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("stylistic").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `swash` attribute.
    /// [`CSSFontFeatureValuesRule.swash`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/swash)
    pub fn swash(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("swash").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `characterVariant` attribute.
    /// [`CSSFontFeatureValuesRule.characterVariant`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/characterVariant)
    pub fn character_variant(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("characterVariant").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `styleset` attribute.
    /// [`CSSFontFeatureValuesRule.styleset`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/styleset)
    pub fn styleset(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("styleset").as_::<CSSFontFeatureValuesMap>()
    }

}
impl CSSFontFeatureValuesRule {
    /// Getter of the `historicalForms` attribute.
    /// [`CSSFontFeatureValuesRule.historicalForms`](https://developer.mozilla.org/en-US/docs/Web/API/CSSFontFeatureValuesRule/historicalForms)
    pub fn historical_forms(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("historicalForms").as_::<CSSFontFeatureValuesMap>()
    }

}
