use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCError {
    inner: DOMException,
}
impl FromVal for RTCError {
    fn from_val(v: &emlite::Val) -> Self {
        RTCError {
            inner: DOMException::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for RTCError {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for RTCError {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<RTCError> for emlite::Val {
    fn from(s: RTCError) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&RTCError> for emlite::Val {
    fn from(s: &RTCError) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(RTCError);

impl RTCError {
    pub fn new0(init: &Any) -> RTCError {
        Self {
            inner: emlite::Val::global("RTCError")
                .new(&[init.into()])
                .as_::<DOMException>(),
        }
    }

    pub fn new1(init: &Any, message: &str) -> RTCError {
        Self {
            inner: emlite::Val::global("RTCError")
                .new(&[init.into(), message.into()])
                .as_::<DOMException>(),
        }
    }
}
impl RTCError {
    pub fn error_detail(&self) -> RTCErrorDetailType {
        self.inner.get("errorDetail").as_::<RTCErrorDetailType>()
    }
}
impl RTCError {
    pub fn sdp_line_number(&self) -> i32 {
        self.inner.get("sdpLineNumber").as_::<i32>()
    }
}
impl RTCError {
    pub fn sctp_cause_code(&self) -> i32 {
        self.inner.get("sctpCauseCode").as_::<i32>()
    }
}
impl RTCError {
    pub fn received_alert(&self) -> u32 {
        self.inner.get("receivedAlert").as_::<u32>()
    }
}
impl RTCError {
    pub fn sent_alert(&self) -> u32 {
        self.inner.get("sentAlert").as_::<u32>()
    }
}
impl RTCError {
    pub fn http_request_status_code(&self) -> i32 {
        self.inner.get("httpRequestStatusCode").as_::<i32>()
    }
}
