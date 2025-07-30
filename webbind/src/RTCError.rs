use super::*;

/// The RTCError class.
/// [`RTCError`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCError {
    inner: DOMException,
}
impl FromVal for RTCError {
    fn from_val(v: &Any) -> Self {
        RTCError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for RTCError {
    type Target = DOMException;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for RTCError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for RTCError {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for RTCError {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<RTCError> for Any {
    fn from(s: RTCError) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&RTCError> for Any {
    fn from(s: &RTCError) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCError);

impl RTCError {
    /// The `new RTCError(..)` constructor, creating a new RTCError instance
    pub fn new0(init: &Any) -> RTCError {
        Self {
            inner: Any::global("RTCError")
                .new(&[init.into()])
                .as_::<DOMException>(),
        }
    }

    /// The `new RTCError(..)` constructor, creating a new RTCError instance
    pub fn new1(init: &Any, message: &JsString) -> RTCError {
        Self {
            inner: Any::global("RTCError")
                .new(&[init.into(), message.into()])
                .as_::<DOMException>(),
        }
    }
}
impl RTCError {
    /// Getter of the `errorDetail` attribute.
    /// [`RTCError.errorDetail`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/errorDetail)
    pub fn error_detail(&self) -> RTCErrorDetailType {
        self.inner.get("errorDetail").as_::<RTCErrorDetailType>()
    }
}
impl RTCError {
    /// Getter of the `sdpLineNumber` attribute.
    /// [`RTCError.sdpLineNumber`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/sdpLineNumber)
    pub fn sdp_line_number(&self) -> i32 {
        self.inner.get("sdpLineNumber").as_::<i32>()
    }
}
impl RTCError {
    /// Getter of the `sctpCauseCode` attribute.
    /// [`RTCError.sctpCauseCode`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/sctpCauseCode)
    pub fn sctp_cause_code(&self) -> i32 {
        self.inner.get("sctpCauseCode").as_::<i32>()
    }
}
impl RTCError {
    /// Getter of the `receivedAlert` attribute.
    /// [`RTCError.receivedAlert`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/receivedAlert)
    pub fn received_alert(&self) -> u32 {
        self.inner.get("receivedAlert").as_::<u32>()
    }
}
impl RTCError {
    /// Getter of the `sentAlert` attribute.
    /// [`RTCError.sentAlert`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/sentAlert)
    pub fn sent_alert(&self) -> u32 {
        self.inner.get("sentAlert").as_::<u32>()
    }
}
impl RTCError {
    /// Getter of the `httpRequestStatusCode` attribute.
    /// [`RTCError.httpRequestStatusCode`](https://developer.mozilla.org/en-US/docs/Web/API/RTCError/httpRequestStatusCode)
    pub fn http_request_status_code(&self) -> i32 {
        self.inner.get("httpRequestStatusCode").as_::<i32>()
    }
}
