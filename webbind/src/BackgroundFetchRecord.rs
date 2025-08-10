use super::*;

/// The BackgroundFetchRecord class.
/// [`BackgroundFetchRecord`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRecord)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BackgroundFetchRecord {
    inner: Any,
}

impl FromVal for BackgroundFetchRecord {
    fn from_val(v: &Any) -> Self {
        BackgroundFetchRecord {
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

impl core::ops::Deref for BackgroundFetchRecord {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for BackgroundFetchRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for BackgroundFetchRecord {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for BackgroundFetchRecord {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<BackgroundFetchRecord> for Any {
    fn from(s: BackgroundFetchRecord) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&BackgroundFetchRecord> for Any {
    fn from(s: &BackgroundFetchRecord) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(BackgroundFetchRecord);

impl BackgroundFetchRecord {
    /// Getter of the `request` attribute.
    /// [`BackgroundFetchRecord.request`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRecord/request)
    pub fn request(&self) -> Request {
        self.inner.get("request").as_::<Request>()
    }
}
impl BackgroundFetchRecord {
    /// Getter of the `responseReady` attribute.
    /// [`BackgroundFetchRecord.responseReady`](https://developer.mozilla.org/en-US/docs/Web/API/BackgroundFetchRecord/responseReady)
    pub fn response_ready(&self) -> Promise<Response> {
        self.inner.get("responseReady").as_::<Promise<Response>>()
    }
}
