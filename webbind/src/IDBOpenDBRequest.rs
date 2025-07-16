use super::*;

/// The IDBOpenDBRequest class.
/// [`IDBOpenDBRequest`](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IDBOpenDBRequest {
    inner: IDBRequest,
}
impl FromVal for IDBOpenDBRequest {
    fn from_val(v: &Any) -> Self {
        IDBOpenDBRequest {
            inner: IDBRequest::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IDBOpenDBRequest {
    type Target = IDBRequest;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IDBOpenDBRequest {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for IDBOpenDBRequest {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for IDBOpenDBRequest {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<IDBOpenDBRequest> for Any {
    fn from(s: IDBOpenDBRequest) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&IDBOpenDBRequest> for Any {
    fn from(s: &IDBOpenDBRequest) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(IDBOpenDBRequest);

impl IDBOpenDBRequest {
    /// Getter of the `onblocked` attribute.
    /// [`IDBOpenDBRequest.onblocked`](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)
    pub fn onblocked(&self) -> Any {
        self.inner.get("onblocked").as_::<Any>()
    }

    /// Setter of the `onblocked` attribute.
    /// [`IDBOpenDBRequest.onblocked`](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onblocked)
    pub fn set_onblocked(&mut self, value: &Any) {
        self.inner.set("onblocked", value);
    }
}
impl IDBOpenDBRequest {
    /// Getter of the `onupgradeneeded` attribute.
    /// [`IDBOpenDBRequest.onupgradeneeded`](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)
    pub fn onupgradeneeded(&self) -> Any {
        self.inner.get("onupgradeneeded").as_::<Any>()
    }

    /// Setter of the `onupgradeneeded` attribute.
    /// [`IDBOpenDBRequest.onupgradeneeded`](https://developer.mozilla.org/en-US/docs/Web/API/IDBOpenDBRequest/onupgradeneeded)
    pub fn set_onupgradeneeded(&mut self, value: &Any) {
        self.inner.set("onupgradeneeded", value);
    }
}
