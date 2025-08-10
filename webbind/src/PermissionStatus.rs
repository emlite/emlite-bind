use super::*;

/// The PermissionStatus class.
/// [`PermissionStatus`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct PermissionStatus {
    inner: EventTarget,
}

impl FromVal for PermissionStatus {
    fn from_val(v: &Any) -> Self {
        PermissionStatus {
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

impl core::ops::Deref for PermissionStatus {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for PermissionStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for PermissionStatus {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for PermissionStatus {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<PermissionStatus> for Any {
    fn from(s: PermissionStatus) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&PermissionStatus> for Any {
    fn from(s: &PermissionStatus) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(PermissionStatus);

impl PermissionStatus {
    /// Getter of the `state` attribute.
    /// [`PermissionStatus.state`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/state)
    pub fn state(&self) -> PermissionState {
        self.inner.get("state").as_::<PermissionState>()
    }
}
impl PermissionStatus {
    /// Getter of the `name` attribute.
    /// [`PermissionStatus.name`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }
}
impl PermissionStatus {
    /// Getter of the `onchange` attribute.
    /// [`PermissionStatus.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`PermissionStatus.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/PermissionStatus/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
