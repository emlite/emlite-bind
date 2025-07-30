use super::*;

/// The AbortSignal class.
/// [`AbortSignal`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbortSignal {
    inner: EventTarget,
}
impl FromVal for AbortSignal {
    fn from_val(v: &Any) -> Self {
        AbortSignal {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for AbortSignal {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for AbortSignal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for AbortSignal {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for AbortSignal {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<AbortSignal> for Any {
    fn from(s: AbortSignal) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&AbortSignal> for Any {
    fn from(s: &AbortSignal) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(AbortSignal);

impl AbortSignal {
    /// The abort method.
    /// [`AbortSignal.abort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort)
    pub fn abort0() -> AbortSignal {
        Any::global("AbortSignal")
            .call("abort", &[])
            .as_::<AbortSignal>()
    }
    /// The abort method.
    /// [`AbortSignal.abort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/abort)
    pub fn abort1(reason: &Any) -> AbortSignal {
        Any::global("AbortSignal")
            .call("abort", &[reason.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    /// The timeout method.
    /// [`AbortSignal.timeout`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/timeout)
    pub fn timeout(milliseconds: u64) -> AbortSignal {
        Any::global("AbortSignal")
            .call("timeout", &[milliseconds.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    /// The any method.
    /// [`AbortSignal.any`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/any)
    pub fn any(signals: &TypedArray<AbortSignal>) -> AbortSignal {
        Any::global("AbortSignal")
            .call("any", &[signals.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    /// Getter of the `aborted` attribute.
    /// [`AbortSignal.aborted`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/aborted)
    pub fn aborted(&self) -> bool {
        self.inner.get("aborted").as_::<bool>()
    }
}
impl AbortSignal {
    /// Getter of the `reason` attribute.
    /// [`AbortSignal.reason`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/reason)
    pub fn reason(&self) -> Any {
        self.inner.get("reason").as_::<Any>()
    }
}
impl AbortSignal {
    /// The throwIfAborted method.
    /// [`AbortSignal.throwIfAborted`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/throwIfAborted)
    pub fn throw_if_aborted(&self) -> Undefined {
        self.inner.call("throwIfAborted", &[]).as_::<Undefined>()
    }
}
impl AbortSignal {
    /// Getter of the `onabort` attribute.
    /// [`AbortSignal.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)
    pub fn onabort(&self) -> Any {
        self.inner.get("onabort").as_::<Any>()
    }

    /// Setter of the `onabort` attribute.
    /// [`AbortSignal.onabort`](https://developer.mozilla.org/en-US/docs/Web/API/AbortSignal/onabort)
    pub fn set_onabort(&mut self, value: &Any) {
        self.inner.set("onabort", value);
    }
}
