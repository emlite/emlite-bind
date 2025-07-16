use super::*;

/// The PreferenceManager class.
/// [`PreferenceManager`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PreferenceManager {
    inner: Any,
}
impl FromVal for PreferenceManager {
    fn from_val(v: &Any) -> Self {
        PreferenceManager {
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
impl core::ops::Deref for PreferenceManager {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PreferenceManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PreferenceManager {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PreferenceManager {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PreferenceManager> for Any {
    fn from(s: PreferenceManager) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PreferenceManager> for Any {
    fn from(s: &PreferenceManager) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PreferenceManager);

impl PreferenceManager {
    /// Getter of the `colorScheme` attribute.
    /// [`PreferenceManager.colorScheme`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager/colorScheme)
    pub fn color_scheme(&self) -> PreferenceObject {
        self.inner.get("colorScheme").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    /// Getter of the `contrast` attribute.
    /// [`PreferenceManager.contrast`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager/contrast)
    pub fn contrast(&self) -> PreferenceObject {
        self.inner.get("contrast").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    /// Getter of the `reducedMotion` attribute.
    /// [`PreferenceManager.reducedMotion`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager/reducedMotion)
    pub fn reduced_motion(&self) -> PreferenceObject {
        self.inner.get("reducedMotion").as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    /// Getter of the `reducedTransparency` attribute.
    /// [`PreferenceManager.reducedTransparency`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager/reducedTransparency)
    pub fn reduced_transparency(&self) -> PreferenceObject {
        self.inner
            .get("reducedTransparency")
            .as_::<PreferenceObject>()
    }
}
impl PreferenceManager {
    /// Getter of the `reducedData` attribute.
    /// [`PreferenceManager.reducedData`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceManager/reducedData)
    pub fn reduced_data(&self) -> PreferenceObject {
        self.inner.get("reducedData").as_::<PreferenceObject>()
    }
}
