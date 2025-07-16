use super::*;

/// The MimeType class.
/// [`MimeType`](https://developer.mozilla.org/en-US/docs/Web/API/MimeType)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MimeType {
    inner: Any,
}
impl FromVal for MimeType {
    fn from_val(v: &Any) -> Self {
        MimeType {
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
impl core::ops::Deref for MimeType {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MimeType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MimeType {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MimeType {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MimeType> for Any {
    fn from(s: MimeType) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MimeType> for Any {
    fn from(s: &MimeType) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MimeType);

impl MimeType {
    /// Getter of the `type` attribute.
    /// [`MimeType.type`](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/type)
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }
}
impl MimeType {
    /// Getter of the `description` attribute.
    /// [`MimeType.description`](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/description)
    pub fn description(&self) -> String {
        self.inner.get("description").as_::<String>()
    }
}
impl MimeType {
    /// Getter of the `suffixes` attribute.
    /// [`MimeType.suffixes`](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/suffixes)
    pub fn suffixes(&self) -> String {
        self.inner.get("suffixes").as_::<String>()
    }
}
impl MimeType {
    /// Getter of the `enabledPlugin` attribute.
    /// [`MimeType.enabledPlugin`](https://developer.mozilla.org/en-US/docs/Web/API/MimeType/enabledPlugin)
    pub fn enabled_plugin(&self) -> Plugin {
        self.inner.get("enabledPlugin").as_::<Plugin>()
    }
}
