use super::*;

/// The FencedFrameConfig class.
/// [`FencedFrameConfig`](https://developer.mozilla.org/en-US/docs/Web/API/FencedFrameConfig)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FencedFrameConfig {
    inner: Any,
}

impl FromVal for FencedFrameConfig {
    fn from_val(v: &Any) -> Self {
        FencedFrameConfig {
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

impl core::ops::Deref for FencedFrameConfig {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for FencedFrameConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for FencedFrameConfig {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for FencedFrameConfig {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<FencedFrameConfig> for Any {
    fn from(s: FencedFrameConfig) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&FencedFrameConfig> for Any {
    fn from(s: &FencedFrameConfig) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(FencedFrameConfig);

impl FencedFrameConfig {
    /// The `new FencedFrameConfig(..)` constructor, creating a new FencedFrameConfig instance
    pub fn new(url: &JsString) -> FencedFrameConfig {
        Self {
            inner: Any::global("FencedFrameConfig")
                .new(&[url.into()])
                .as_::<Any>(),
        }
    }
}

impl FencedFrameConfig {
    /// The setSharedStorageContext method.
    /// [`FencedFrameConfig.setSharedStorageContext`](https://developer.mozilla.org/en-US/docs/Web/API/FencedFrameConfig/setSharedStorageContext)
    pub fn set_shared_storage_context(&self, context_string: &JsString) -> Undefined {
        self.inner
            .call("setSharedStorageContext", &[context_string.into()])
            .as_::<Undefined>()
    }
}
