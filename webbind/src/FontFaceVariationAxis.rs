use super::*;

/// The FontFaceVariationAxis class.
/// [`FontFaceVariationAxis`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceVariationAxis {
    inner: Any,
}
impl FromVal for FontFaceVariationAxis {
    fn from_val(v: &Any) -> Self {
        FontFaceVariationAxis {
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
impl core::ops::Deref for FontFaceVariationAxis {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceVariationAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for FontFaceVariationAxis {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for FontFaceVariationAxis {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<FontFaceVariationAxis> for Any {
    fn from(s: FontFaceVariationAxis) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&FontFaceVariationAxis> for Any {
    fn from(s: &FontFaceVariationAxis) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(FontFaceVariationAxis);

impl FontFaceVariationAxis {
    /// Getter of the `name` attribute.
    /// [`FontFaceVariationAxis.name`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis/name)
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }
}
impl FontFaceVariationAxis {
    /// Getter of the `axisTag` attribute.
    /// [`FontFaceVariationAxis.axisTag`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis/axisTag)
    pub fn axis_tag(&self) -> DOMString {
        self.inner.get("axisTag").as_::<DOMString>()
    }
}
impl FontFaceVariationAxis {
    /// Getter of the `minimumValue` attribute.
    /// [`FontFaceVariationAxis.minimumValue`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis/minimumValue)
    pub fn minimum_value(&self) -> f64 {
        self.inner.get("minimumValue").as_::<f64>()
    }
}
impl FontFaceVariationAxis {
    /// Getter of the `maximumValue` attribute.
    /// [`FontFaceVariationAxis.maximumValue`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis/maximumValue)
    pub fn maximum_value(&self) -> f64 {
        self.inner.get("maximumValue").as_::<f64>()
    }
}
impl FontFaceVariationAxis {
    /// Getter of the `defaultValue` attribute.
    /// [`FontFaceVariationAxis.defaultValue`](https://developer.mozilla.org/en-US/docs/Web/API/FontFaceVariationAxis/defaultValue)
    pub fn default_value(&self) -> f64 {
        self.inner.get("defaultValue").as_::<f64>()
    }
}
