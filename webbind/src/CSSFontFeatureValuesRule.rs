use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CSSFontFeatureValuesRule {
    inner: CSSRule,
}
impl FromVal for CSSFontFeatureValuesRule {
    fn from_val(v: &emlite::Val) -> Self {
        CSSFontFeatureValuesRule {
            inner: CSSRule::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl From<CSSFontFeatureValuesRule> for emlite::Val {
    fn from(s: CSSFontFeatureValuesRule) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSFontFeatureValuesRule {
    pub fn font_family(&self) -> jsbind::CSSOMString {
        self.inner.get("fontFamily").as_::<jsbind::CSSOMString>()
    }

    pub fn set_font_family(&mut self, value: jsbind::CSSOMString) {
        self.inner.set("fontFamily", value);
    }
}
impl CSSFontFeatureValuesRule {
    pub fn annotation(&self) -> CSSFontFeatureValuesMap {
        self.inner
            .get("annotation")
            .as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn ornaments(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("ornaments").as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn stylistic(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("stylistic").as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn swash(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("swash").as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn character_variant(&self) -> CSSFontFeatureValuesMap {
        self.inner
            .get("characterVariant")
            .as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn styleset(&self) -> CSSFontFeatureValuesMap {
        self.inner.get("styleset").as_::<CSSFontFeatureValuesMap>()
    }
}
impl CSSFontFeatureValuesRule {
    pub fn historical_forms(&self) -> CSSFontFeatureValuesMap {
        self.inner
            .get("historicalForms")
            .as_::<CSSFontFeatureValuesMap>()
    }
}
