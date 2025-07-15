use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SecurityPolicyViolationEvent {
    inner: Event,
}
impl FromVal for SecurityPolicyViolationEvent {
    fn from_val(v: &emlite::Val) -> Self {
        SecurityPolicyViolationEvent { inner: Event::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SecurityPolicyViolationEvent {
    type Target = Event;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SecurityPolicyViolationEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SecurityPolicyViolationEvent {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SecurityPolicyViolationEvent {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SecurityPolicyViolationEvent> for emlite::Val {
    fn from(s: SecurityPolicyViolationEvent) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SecurityPolicyViolationEvent);



impl SecurityPolicyViolationEvent {
    pub fn new0(type_: DOMString) -> SecurityPolicyViolationEvent {
        Self {
            inner: emlite::Val::global("SecurityPolicyViolationEvent").new(&[type_.into()]).as_::<Event>(),
        }
    }

    pub fn new1(type_: DOMString, event_init_dict: Any) -> SecurityPolicyViolationEvent {
        Self {
            inner: emlite::Val::global("SecurityPolicyViolationEvent").new(&[type_.into(), event_init_dict.into()]).as_::<Event>(),
        }
    }

}
impl SecurityPolicyViolationEvent {
    pub fn document_uri(&self) -> USVString {
        self.inner.get("documentURI").as_::<USVString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn referrer(&self) -> USVString {
        self.inner.get("referrer").as_::<USVString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn blocked_uri(&self) -> USVString {
        self.inner.get("blockedURI").as_::<USVString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn effective_directive(&self) -> DOMString {
        self.inner.get("effectiveDirective").as_::<DOMString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn violated_directive(&self) -> DOMString {
        self.inner.get("violatedDirective").as_::<DOMString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn original_policy(&self) -> DOMString {
        self.inner.get("originalPolicy").as_::<DOMString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn source_file(&self) -> USVString {
        self.inner.get("sourceFile").as_::<USVString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn sample(&self) -> DOMString {
        self.inner.get("sample").as_::<DOMString>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.inner.get("disposition").as_::<SecurityPolicyViolationEventDisposition>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn status_code(&self) -> u16 {
        self.inner.get("statusCode").as_::<u16>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }

}
impl SecurityPolicyViolationEvent {
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }

}
