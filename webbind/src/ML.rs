use super::*;

/// The ML class.
/// [`ML`](https://developer.mozilla.org/en-US/docs/Web/API/ML)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ML {
    inner: Any,
}
impl FromVal for ML {
    fn from_val(v: &Any) -> Self {
        ML {
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
impl core::ops::Deref for ML {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ML {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ML {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ML {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ML> for Any {
    fn from(s: ML) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ML> for Any {
    fn from(s: &ML) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ML);

impl ML {
    /// The createContext method.
    /// [`ML.createContext`](https://developer.mozilla.org/en-US/docs/Web/API/ML/createContext)
    pub fn create_context(&self, gpu_device: &GPUDevice) -> Promise {
        self.inner
            .call("createContext", &[gpu_device.into()])
            .as_::<Promise>()
    }
}
