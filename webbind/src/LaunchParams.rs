use super::*;

/// The LaunchParams class.
/// [`LaunchParams`](https://developer.mozilla.org/en-US/docs/Web/API/LaunchParams)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct LaunchParams {
    inner: Any,
}
impl FromVal for LaunchParams {
    fn from_val(v: &Any) -> Self {
        LaunchParams {
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
impl core::ops::Deref for LaunchParams {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for LaunchParams {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for LaunchParams {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for LaunchParams {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<LaunchParams> for Any {
    fn from(s: LaunchParams) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&LaunchParams> for Any {
    fn from(s: &LaunchParams) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(LaunchParams);

impl LaunchParams {
    /// Getter of the `targetURL` attribute.
    /// [`LaunchParams.targetURL`](https://developer.mozilla.org/en-US/docs/Web/API/LaunchParams/targetURL)
    pub fn target_url(&self) -> JsString {
        self.inner.get("targetURL").as_::<JsString>()
    }
}
impl LaunchParams {
    /// Getter of the `files` attribute.
    /// [`LaunchParams.files`](https://developer.mozilla.org/en-US/docs/Web/API/LaunchParams/files)
    pub fn files(&self) -> TypedArray<FileSystemHandle> {
        self.inner
            .get("files")
            .as_::<TypedArray<FileSystemHandle>>()
    }
}
