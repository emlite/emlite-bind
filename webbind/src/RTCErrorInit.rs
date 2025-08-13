use super::*;




/// The RTCErrorInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RTCErrorInit {
    inner: Any,
}

impl FromVal for RTCErrorInit {
    fn from_val(v: &Any) -> Self {
        RTCErrorInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for RTCErrorInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for RTCErrorInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for RTCErrorInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for RTCErrorInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<RTCErrorInit> for Any {
    fn from(s: RTCErrorInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&RTCErrorInit> for Any {
    fn from(s: &RTCErrorInit) -> Any {
        s.inner.clone()
    }
}

impl RTCErrorInit {
    /// Getter of the `errorDetail` attribute.
    pub fn error_detail(&self) -> RTCErrorDetailType {
        self.inner.get("errorDetail").as_::<RTCErrorDetailType>()
    }

    /// Setter of the `errorDetail` attribute.
    pub fn set_error_detail(&mut self, value: &RTCErrorDetailType) {
        self.inner.set("errorDetail", value);
    }
}
impl RTCErrorInit {
    /// Getter of the `sdpLineNumber` attribute.
    pub fn sdp_line_number(&self) -> i32 {
        self.inner.get("sdpLineNumber").as_::<i32>()
    }

    /// Setter of the `sdpLineNumber` attribute.
    pub fn set_sdp_line_number(&mut self, value: i32) {
        self.inner.set("sdpLineNumber", value);
    }
}
impl RTCErrorInit {
    /// Getter of the `sctpCauseCode` attribute.
    pub fn sctp_cause_code(&self) -> i32 {
        self.inner.get("sctpCauseCode").as_::<i32>()
    }

    /// Setter of the `sctpCauseCode` attribute.
    pub fn set_sctp_cause_code(&mut self, value: i32) {
        self.inner.set("sctpCauseCode", value);
    }
}
impl RTCErrorInit {
    /// Getter of the `receivedAlert` attribute.
    pub fn received_alert(&self) -> u32 {
        self.inner.get("receivedAlert").as_::<u32>()
    }

    /// Setter of the `receivedAlert` attribute.
    pub fn set_received_alert(&mut self, value: u32) {
        self.inner.set("receivedAlert", value);
    }
}
impl RTCErrorInit {
    /// Getter of the `sentAlert` attribute.
    pub fn sent_alert(&self) -> u32 {
        self.inner.get("sentAlert").as_::<u32>()
    }

    /// Setter of the `sentAlert` attribute.
    pub fn set_sent_alert(&mut self, value: u32) {
        self.inner.set("sentAlert", value);
    }
}
