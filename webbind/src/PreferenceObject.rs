use super::*;

/// The PreferenceObject class.
/// [`PreferenceObject`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PreferenceObject {
    inner: EventTarget,
}
impl FromVal for PreferenceObject {
    fn from_val(v: &Any) -> Self {
        PreferenceObject {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for PreferenceObject {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for PreferenceObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for PreferenceObject {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for PreferenceObject {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<PreferenceObject> for Any {
    fn from(s: PreferenceObject) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&PreferenceObject> for Any {
    fn from(s: &PreferenceObject) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(PreferenceObject);

impl PreferenceObject {
    /// Getter of the `override` attribute.
    /// [`PreferenceObject.override`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/override)
    pub fn override_(&self) -> String {
        self.inner.get("override").as_::<String>()
    }
}
impl PreferenceObject {
    /// Getter of the `value` attribute.
    /// [`PreferenceObject.value`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/value)
    pub fn value(&self) -> String {
        self.inner.get("value").as_::<String>()
    }
}
impl PreferenceObject {
    /// Getter of the `validValues` attribute.
    /// [`PreferenceObject.validValues`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/validValues)
    pub fn valid_values(&self) -> FrozenArray<String> {
        self.inner.get("validValues").as_::<FrozenArray<String>>()
    }
}
impl PreferenceObject {
    /// The clearOverride method.
    /// [`PreferenceObject.clearOverride`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/clearOverride)
    pub fn clear_override(&self) -> Undefined {
        self.inner.call("clearOverride", &[]).as_::<Undefined>()
    }
}
impl PreferenceObject {
    /// The requestOverride method.
    /// [`PreferenceObject.requestOverride`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/requestOverride)
    pub fn request_override(&self, value: &str) -> Promise<Undefined> {
        self.inner
            .call("requestOverride", &[value.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl PreferenceObject {
    /// Getter of the `onchange` attribute.
    /// [`PreferenceObject.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`PreferenceObject.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PreferenceObject/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
