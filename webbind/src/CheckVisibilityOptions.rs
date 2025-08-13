use super::*;




/// The CheckVisibilityOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CheckVisibilityOptions {
    inner: Any,
}

impl FromVal for CheckVisibilityOptions {
    fn from_val(v: &Any) -> Self {
        CheckVisibilityOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CheckVisibilityOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CheckVisibilityOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CheckVisibilityOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CheckVisibilityOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CheckVisibilityOptions> for Any {
    fn from(s: CheckVisibilityOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CheckVisibilityOptions> for Any {
    fn from(s: &CheckVisibilityOptions) -> Any {
        s.inner.clone()
    }
}

impl CheckVisibilityOptions {
    /// Getter of the `checkOpacity` attribute.
    pub fn check_opacity(&self) -> bool {
        self.inner.get("checkOpacity").as_::<bool>()
    }

    /// Setter of the `checkOpacity` attribute.
    pub fn set_check_opacity(&mut self, value: bool) {
        self.inner.set("checkOpacity", value);
    }
}
impl CheckVisibilityOptions {
    /// Getter of the `checkVisibilityCSS` attribute.
    pub fn check_visibility_css(&self) -> bool {
        self.inner.get("checkVisibilityCSS").as_::<bool>()
    }

    /// Setter of the `checkVisibilityCSS` attribute.
    pub fn set_check_visibility_css(&mut self, value: bool) {
        self.inner.set("checkVisibilityCSS", value);
    }
}
impl CheckVisibilityOptions {
    /// Getter of the `contentVisibilityAuto` attribute.
    pub fn content_visibility_auto(&self) -> bool {
        self.inner.get("contentVisibilityAuto").as_::<bool>()
    }

    /// Setter of the `contentVisibilityAuto` attribute.
    pub fn set_content_visibility_auto(&mut self, value: bool) {
        self.inner.set("contentVisibilityAuto", value);
    }
}
impl CheckVisibilityOptions {
    /// Getter of the `opacityProperty` attribute.
    pub fn opacity_property(&self) -> bool {
        self.inner.get("opacityProperty").as_::<bool>()
    }

    /// Setter of the `opacityProperty` attribute.
    pub fn set_opacity_property(&mut self, value: bool) {
        self.inner.set("opacityProperty", value);
    }
}
impl CheckVisibilityOptions {
    /// Getter of the `visibilityProperty` attribute.
    pub fn visibility_property(&self) -> bool {
        self.inner.get("visibilityProperty").as_::<bool>()
    }

    /// Setter of the `visibilityProperty` attribute.
    pub fn set_visibility_property(&mut self, value: bool) {
        self.inner.set("visibilityProperty", value);
    }
}
