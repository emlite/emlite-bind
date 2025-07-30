use super::*;

/// The XRPermissionStatus class.
/// [`XRPermissionStatus`](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRPermissionStatus {
    inner: PermissionStatus,
}
impl FromVal for XRPermissionStatus {
    fn from_val(v: &Any) -> Self {
        XRPermissionStatus {
            inner: PermissionStatus::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRPermissionStatus {
    type Target = PermissionStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRPermissionStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for XRPermissionStatus {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for XRPermissionStatus {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<XRPermissionStatus> for Any {
    fn from(s: XRPermissionStatus) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&XRPermissionStatus> for Any {
    fn from(s: &XRPermissionStatus) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(XRPermissionStatus);

impl XRPermissionStatus {
    /// Getter of the `granted` attribute.
    /// [`XRPermissionStatus.granted`](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus/granted)
    pub fn granted(&self) -> TypedArray<JsString> {
        self.inner.get("granted").as_::<TypedArray<JsString>>()
    }

    /// Setter of the `granted` attribute.
    /// [`XRPermissionStatus.granted`](https://developer.mozilla.org/en-US/docs/Web/API/XRPermissionStatus/granted)
    pub fn set_granted(&mut self, value: &TypedArray<JsString>) {
        self.inner.set("granted", value);
    }
}
