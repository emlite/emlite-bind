use super::*;




/// The SVGPathDataSettings dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPathDataSettings {
    inner: Any,
}

impl FromVal for SVGPathDataSettings {
    fn from_val(v: &Any) -> Self {
        SVGPathDataSettings { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGPathDataSettings {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGPathDataSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGPathDataSettings {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGPathDataSettings {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGPathDataSettings> for Any {
    fn from(s: SVGPathDataSettings) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGPathDataSettings> for Any {
    fn from(s: &SVGPathDataSettings) -> Any {
        s.inner.clone()
    }
}

impl SVGPathDataSettings {
    /// Getter of the `normalize` attribute.
    pub fn normalize(&self) -> bool {
        self.inner.get("normalize").as_::<bool>()
    }

    /// Setter of the `normalize` attribute.
    pub fn set_normalize(&mut self, value: bool) {
        self.inner.set("normalize", value);
    }
}
