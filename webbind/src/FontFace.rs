use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFace {
    inner: emlite::Val,
}
impl FromVal for FontFace {
    fn from_val(v: &emlite::Val) -> Self {
        FontFace {
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
impl core::ops::Deref for FontFace {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontFace {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontFace {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<FontFace> for emlite::Val {
    fn from(s: FontFace) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&FontFace> for emlite::Val {
    fn from(s: &FontFace) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontFace);

impl FontFace {
    pub fn new0(family: &str, source: &Any) -> FontFace {
        Self {
            inner: emlite::Val::global("FontFace")
                .new(&[family.into(), source.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(family: &str, source: &Any, descriptors: &Any) -> FontFace {
        Self {
            inner: emlite::Val::global("FontFace")
                .new(&[family.into(), source.into(), descriptors.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl FontFace {
    pub fn family(&self) -> String {
        self.inner.get("family").as_::<String>()
    }

    pub fn set_family(&mut self, value: &str) {
        self.inner.set("family", value);
    }
}
impl FontFace {
    pub fn style(&self) -> String {
        self.inner.get("style").as_::<String>()
    }

    pub fn set_style(&mut self, value: &str) {
        self.inner.set("style", value);
    }
}
impl FontFace {
    pub fn weight(&self) -> String {
        self.inner.get("weight").as_::<String>()
    }

    pub fn set_weight(&mut self, value: &str) {
        self.inner.set("weight", value);
    }
}
impl FontFace {
    pub fn stretch(&self) -> String {
        self.inner.get("stretch").as_::<String>()
    }

    pub fn set_stretch(&mut self, value: &str) {
        self.inner.set("stretch", value);
    }
}
impl FontFace {
    pub fn unicode_range(&self) -> String {
        self.inner.get("unicodeRange").as_::<String>()
    }

    pub fn set_unicode_range(&mut self, value: &str) {
        self.inner.set("unicodeRange", value);
    }
}
impl FontFace {
    pub fn feature_settings(&self) -> String {
        self.inner.get("featureSettings").as_::<String>()
    }

    pub fn set_feature_settings(&mut self, value: &str) {
        self.inner.set("featureSettings", value);
    }
}
impl FontFace {
    pub fn variation_settings(&self) -> String {
        self.inner.get("variationSettings").as_::<String>()
    }

    pub fn set_variation_settings(&mut self, value: &str) {
        self.inner.set("variationSettings", value);
    }
}
impl FontFace {
    pub fn display(&self) -> String {
        self.inner.get("display").as_::<String>()
    }

    pub fn set_display(&mut self, value: &str) {
        self.inner.set("display", value);
    }
}
impl FontFace {
    pub fn ascent_override(&self) -> String {
        self.inner.get("ascentOverride").as_::<String>()
    }

    pub fn set_ascent_override(&mut self, value: &str) {
        self.inner.set("ascentOverride", value);
    }
}
impl FontFace {
    pub fn descent_override(&self) -> String {
        self.inner.get("descentOverride").as_::<String>()
    }

    pub fn set_descent_override(&mut self, value: &str) {
        self.inner.set("descentOverride", value);
    }
}
impl FontFace {
    pub fn line_gap_override(&self) -> String {
        self.inner.get("lineGapOverride").as_::<String>()
    }

    pub fn set_line_gap_override(&mut self, value: &str) {
        self.inner.set("lineGapOverride", value);
    }
}
impl FontFace {
    pub fn status(&self) -> FontFaceLoadStatus {
        self.inner.get("status").as_::<FontFaceLoadStatus>()
    }
}
impl FontFace {
    pub fn load(&self) -> Promise {
        self.inner.call("load", &[]).as_::<Promise>()
    }
}
impl FontFace {
    pub fn loaded(&self) -> Promise {
        self.inner.get("loaded").as_::<Promise>()
    }
}
impl FontFace {
    pub fn features(&self) -> FontFaceFeatures {
        self.inner.get("features").as_::<FontFaceFeatures>()
    }
}
impl FontFace {
    pub fn variations(&self) -> FontFaceVariations {
        self.inner.get("variations").as_::<FontFaceVariations>()
    }
}
impl FontFace {
    pub fn palettes(&self) -> FontFacePalettes {
        self.inner.get("palettes").as_::<FontFacePalettes>()
    }
}
