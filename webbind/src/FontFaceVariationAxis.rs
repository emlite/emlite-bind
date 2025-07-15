use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FontFaceVariationAxis {
    inner: emlite::Val,
}
impl FromVal for FontFaceVariationAxis {
    fn from_val(v: &emlite::Val) -> Self {
        FontFaceVariationAxis { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for FontFaceVariationAxis {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for FontFaceVariationAxis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for FontFaceVariationAxis {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for FontFaceVariationAxis {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<FontFaceVariationAxis> for emlite::Val {
    fn from(s: FontFaceVariationAxis) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(FontFaceVariationAxis);


impl FontFaceVariationAxis {
    pub fn name(&self) -> DOMString {
        self.inner.get("name").as_::<DOMString>()
    }

}
impl FontFaceVariationAxis {
    pub fn axis_tag(&self) -> DOMString {
        self.inner.get("axisTag").as_::<DOMString>()
    }

}
impl FontFaceVariationAxis {
    pub fn minimum_value(&self) -> f64 {
        self.inner.get("minimumValue").as_::<f64>()
    }

}
impl FontFaceVariationAxis {
    pub fn maximum_value(&self) -> f64 {
        self.inner.get("maximumValue").as_::<f64>()
    }

}
impl FontFaceVariationAxis {
    pub fn default_value(&self) -> f64 {
        self.inner.get("defaultValue").as_::<f64>()
    }

}
