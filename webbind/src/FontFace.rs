use super::*;

/// The FontFace class.
/// [`FontFace`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFace {
    inner: Any,
}
impl FromVal for FontFace {
    fn from_val(v: &Any) -> Self {
        FontFace {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFace {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFace {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFace {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFace {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFace> for Any {
    fn from(s: FontFace) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFace> for Any {
    fn from(s: &FontFace) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontFace);

impl FontFace {
    /// The `new FontFace(..)` constructor, creating a new FontFace instance
    pub fn new0(family: &str, source: &Any) -> FontFace {
        Self {
            inner: Any::global("FontFace")
                .new(&[family.into(), source.into()])
                .as_::<Any>(),
        }
    }

    /// The `new FontFace(..)` constructor, creating a new FontFace instance
    pub fn new1(family: &str, source: &Any, descriptors: &Any) -> FontFace {
        Self {
            inner: Any::global("FontFace")
                .new(&[family.into(), source.into(), descriptors.into()])
                .as_::<Any>(),
        }
    }
}
impl FontFace {
    /// Getter of the `family` attribute.
    /// [`FontFace.family`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)
    pub fn family(&self) -> String {
        self.inner.get("family").as_::<String>()
    }

    /// Setter of the `family` attribute.
    /// [`FontFace.family`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/family)
    pub fn set_family(&mut self, value: &str) {
        self.inner.set("family", value);
    }
}
impl FontFace {
    /// Getter of the `style` attribute.
    /// [`FontFace.style`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)
    pub fn style(&self) -> String {
        self.inner.get("style").as_::<String>()
    }

    /// Setter of the `style` attribute.
    /// [`FontFace.style`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/style)
    pub fn set_style(&mut self, value: &str) {
        self.inner.set("style", value);
    }
}
impl FontFace {
    /// Getter of the `weight` attribute.
    /// [`FontFace.weight`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)
    pub fn weight(&self) -> String {
        self.inner.get("weight").as_::<String>()
    }

    /// Setter of the `weight` attribute.
    /// [`FontFace.weight`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/weight)
    pub fn set_weight(&mut self, value: &str) {
        self.inner.set("weight", value);
    }
}
impl FontFace {
    /// Getter of the `stretch` attribute.
    /// [`FontFace.stretch`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)
    pub fn stretch(&self) -> String {
        self.inner.get("stretch").as_::<String>()
    }

    /// Setter of the `stretch` attribute.
    /// [`FontFace.stretch`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/stretch)
    pub fn set_stretch(&mut self, value: &str) {
        self.inner.set("stretch", value);
    }
}
impl FontFace {
    /// Getter of the `unicodeRange` attribute.
    /// [`FontFace.unicodeRange`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)
    pub fn unicode_range(&self) -> String {
        self.inner.get("unicodeRange").as_::<String>()
    }

    /// Setter of the `unicodeRange` attribute.
    /// [`FontFace.unicodeRange`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/unicodeRange)
    pub fn set_unicode_range(&mut self, value: &str) {
        self.inner.set("unicodeRange", value);
    }
}
impl FontFace {
    /// Getter of the `featureSettings` attribute.
    /// [`FontFace.featureSettings`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)
    pub fn feature_settings(&self) -> String {
        self.inner.get("featureSettings").as_::<String>()
    }

    /// Setter of the `featureSettings` attribute.
    /// [`FontFace.featureSettings`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/featureSettings)
    pub fn set_feature_settings(&mut self, value: &str) {
        self.inner.set("featureSettings", value);
    }
}
impl FontFace {
    /// Getter of the `variationSettings` attribute.
    /// [`FontFace.variationSettings`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)
    pub fn variation_settings(&self) -> String {
        self.inner.get("variationSettings").as_::<String>()
    }

    /// Setter of the `variationSettings` attribute.
    /// [`FontFace.variationSettings`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variationSettings)
    pub fn set_variation_settings(&mut self, value: &str) {
        self.inner.set("variationSettings", value);
    }
}
impl FontFace {
    /// Getter of the `display` attribute.
    /// [`FontFace.display`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)
    pub fn display(&self) -> String {
        self.inner.get("display").as_::<String>()
    }

    /// Setter of the `display` attribute.
    /// [`FontFace.display`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/display)
    pub fn set_display(&mut self, value: &str) {
        self.inner.set("display", value);
    }
}
impl FontFace {
    /// Getter of the `ascentOverride` attribute.
    /// [`FontFace.ascentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/ascentOverride)
    pub fn ascent_override(&self) -> String {
        self.inner.get("ascentOverride").as_::<String>()
    }

    /// Setter of the `ascentOverride` attribute.
    /// [`FontFace.ascentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/ascentOverride)
    pub fn set_ascent_override(&mut self, value: &str) {
        self.inner.set("ascentOverride", value);
    }
}
impl FontFace {
    /// Getter of the `descentOverride` attribute.
    /// [`FontFace.descentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/descentOverride)
    pub fn descent_override(&self) -> String {
        self.inner.get("descentOverride").as_::<String>()
    }

    /// Setter of the `descentOverride` attribute.
    /// [`FontFace.descentOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/descentOverride)
    pub fn set_descent_override(&mut self, value: &str) {
        self.inner.set("descentOverride", value);
    }
}
impl FontFace {
    /// Getter of the `lineGapOverride` attribute.
    /// [`FontFace.lineGapOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/lineGapOverride)
    pub fn line_gap_override(&self) -> String {
        self.inner.get("lineGapOverride").as_::<String>()
    }

    /// Setter of the `lineGapOverride` attribute.
    /// [`FontFace.lineGapOverride`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/lineGapOverride)
    pub fn set_line_gap_override(&mut self, value: &str) {
        self.inner.set("lineGapOverride", value);
    }
}
impl FontFace {
    /// Getter of the `status` attribute.
    /// [`FontFace.status`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/status)
    pub fn status(&self) -> FontFaceLoadStatus {
        self.inner.get("status").as_::<FontFaceLoadStatus>()
    }
}
impl FontFace {
    /// The load method.
    /// [`FontFace.load`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/load)
    pub fn load(&self) -> Promise<FontFace> {
        self.inner.call("load", &[]).as_::<Promise<FontFace>>()
    }
}
impl FontFace {
    /// Getter of the `loaded` attribute.
    /// [`FontFace.loaded`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/loaded)
    pub fn loaded(&self) -> Promise<FontFace> {
        self.inner.get("loaded").as_::<Promise<FontFace>>()
    }
}
impl FontFace {
    /// Getter of the `features` attribute.
    /// [`FontFace.features`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/features)
    pub fn features(&self) -> FontFaceFeatures {
        self.inner.get("features").as_::<FontFaceFeatures>()
    }
}
impl FontFace {
    /// Getter of the `variations` attribute.
    /// [`FontFace.variations`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/variations)
    pub fn variations(&self) -> FontFaceVariations {
        self.inner.get("variations").as_::<FontFaceVariations>()
    }
}
impl FontFace {
    /// Getter of the `palettes` attribute.
    /// [`FontFace.palettes`](https://developer.mozilla.org/en-US/docs/Web/API/FontFace/palettes)
    pub fn palettes(&self) -> FontFacePalettes {
        self.inner.get("palettes").as_::<FontFacePalettes>()
    }
}
