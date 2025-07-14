use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PreferenceObject {
    inner: EventTarget,
}
impl FromVal for PreferenceObject {
    fn from_val(v: &emlite::Val) -> Self {
        PreferenceObject {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for PreferenceObject {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for PreferenceObject {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<PreferenceObject> for emlite::Val {
    fn from(s: PreferenceObject) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(PreferenceObject);

impl PreferenceObject {
    pub fn override_(&self) -> jsbind::DOMString {
        self.inner.get("override").as_::<jsbind::DOMString>()
    }
}
impl PreferenceObject {
    pub fn value(&self) -> jsbind::DOMString {
        self.inner.get("value").as_::<jsbind::DOMString>()
    }
}
impl PreferenceObject {
    pub fn valid_values(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("validValues")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl PreferenceObject {
    pub fn clear_override(&self) -> jsbind::Undefined {
        self.inner
            .call("clearOverride", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl PreferenceObject {
    pub fn request_override(&self, value: jsbind::DOMString) -> jsbind::Promise {
        self.inner
            .call("requestOverride", &[value.into()])
            .as_::<jsbind::Promise>()
    }
}
impl PreferenceObject {
    pub fn onchange(&self) -> jsbind::Any {
        self.inner.get("onchange").as_::<jsbind::Any>()
    }

    pub fn set_onchange(&mut self, value: jsbind::Any) {
        self.inner.set("onchange", value);
    }
}
