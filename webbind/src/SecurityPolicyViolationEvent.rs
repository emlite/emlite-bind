use super::*;

/// The SecurityPolicyViolationEvent class.
/// [`SecurityPolicyViolationEvent`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SecurityPolicyViolationEvent {
    inner: Event,
}
impl FromVal for SecurityPolicyViolationEvent {
    fn from_val(v: &Any) -> Self {
        SecurityPolicyViolationEvent {
            inner: Event::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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
impl AsRef<Any> for SecurityPolicyViolationEvent {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SecurityPolicyViolationEvent {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SecurityPolicyViolationEvent> for Any {
    fn from(s: SecurityPolicyViolationEvent) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SecurityPolicyViolationEvent> for Any {
    fn from(s: &SecurityPolicyViolationEvent) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SecurityPolicyViolationEvent);

impl SecurityPolicyViolationEvent {
    /// The `new SecurityPolicyViolationEvent(..)` constructor, creating a new SecurityPolicyViolationEvent instance
    pub fn new0(type_: &str) -> SecurityPolicyViolationEvent {
        Self {
            inner: Any::global("SecurityPolicyViolationEvent")
                .new(&[type_.into()])
                .as_::<Event>(),
        }
    }

    /// The `new SecurityPolicyViolationEvent(..)` constructor, creating a new SecurityPolicyViolationEvent instance
    pub fn new1(type_: &str, event_init_dict: &Any) -> SecurityPolicyViolationEvent {
        Self {
            inner: Any::global("SecurityPolicyViolationEvent")
                .new(&[type_.into(), event_init_dict.into()])
                .as_::<Event>(),
        }
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `documentURI` attribute.
    /// [`SecurityPolicyViolationEvent.documentURI`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/documentURI)
    pub fn document_uri(&self) -> String {
        self.inner.get("documentURI").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `referrer` attribute.
    /// [`SecurityPolicyViolationEvent.referrer`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/referrer)
    pub fn referrer(&self) -> String {
        self.inner.get("referrer").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `blockedURI` attribute.
    /// [`SecurityPolicyViolationEvent.blockedURI`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/blockedURI)
    pub fn blocked_uri(&self) -> String {
        self.inner.get("blockedURI").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `effectiveDirective` attribute.
    /// [`SecurityPolicyViolationEvent.effectiveDirective`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/effectiveDirective)
    pub fn effective_directive(&self) -> String {
        self.inner.get("effectiveDirective").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `violatedDirective` attribute.
    /// [`SecurityPolicyViolationEvent.violatedDirective`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/violatedDirective)
    pub fn violated_directive(&self) -> String {
        self.inner.get("violatedDirective").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `originalPolicy` attribute.
    /// [`SecurityPolicyViolationEvent.originalPolicy`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/originalPolicy)
    pub fn original_policy(&self) -> String {
        self.inner.get("originalPolicy").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `sourceFile` attribute.
    /// [`SecurityPolicyViolationEvent.sourceFile`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sourceFile)
    pub fn source_file(&self) -> String {
        self.inner.get("sourceFile").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `sample` attribute.
    /// [`SecurityPolicyViolationEvent.sample`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/sample)
    pub fn sample(&self) -> String {
        self.inner.get("sample").as_::<String>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `disposition` attribute.
    /// [`SecurityPolicyViolationEvent.disposition`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/disposition)
    pub fn disposition(&self) -> SecurityPolicyViolationEventDisposition {
        self.inner
            .get("disposition")
            .as_::<SecurityPolicyViolationEventDisposition>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `statusCode` attribute.
    /// [`SecurityPolicyViolationEvent.statusCode`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/statusCode)
    pub fn status_code(&self) -> u16 {
        self.inner.get("statusCode").as_::<u16>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `lineNumber` attribute.
    /// [`SecurityPolicyViolationEvent.lineNumber`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/lineNumber)
    pub fn line_number(&self) -> u32 {
        self.inner.get("lineNumber").as_::<u32>()
    }
}
impl SecurityPolicyViolationEvent {
    /// Getter of the `columnNumber` attribute.
    /// [`SecurityPolicyViolationEvent.columnNumber`](https://developer.mozilla.org/en-US/docs/Web/API/SecurityPolicyViolationEvent/columnNumber)
    pub fn column_number(&self) -> u32 {
        self.inner.get("columnNumber").as_::<u32>()
    }
}
