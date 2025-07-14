use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AbortSignal {
    inner: EventTarget,
}
impl FromVal for AbortSignal {
    fn from_val(v: &emlite::Val) -> Self {
        AbortSignal {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for AbortSignal {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for AbortSignal {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<AbortSignal> for emlite::Val {
    fn from(s: AbortSignal) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(AbortSignal);

impl AbortSignal {
    pub fn abort0() -> AbortSignal {
        emlite::Val::global("abortsignal")
            .call("abort", &[])
            .as_::<AbortSignal>()
    }

    pub fn abort1(reason: jsbind::Any) -> AbortSignal {
        emlite::Val::global("abortsignal")
            .call("abort", &[reason.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    pub fn timeout(milliseconds: u64) -> AbortSignal {
        emlite::Val::global("abortsignal")
            .call("timeout", &[milliseconds.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    pub fn any(signals: jsbind::Sequence<AbortSignal>) -> AbortSignal {
        emlite::Val::global("abortsignal")
            .call("any", &[signals.into()])
            .as_::<AbortSignal>()
    }
}
impl AbortSignal {
    pub fn aborted(&self) -> bool {
        self.inner.get("aborted").as_::<bool>()
    }
}
impl AbortSignal {
    pub fn reason(&self) -> jsbind::Any {
        self.inner.get("reason").as_::<jsbind::Any>()
    }
}
impl AbortSignal {
    pub fn throw_if_aborted(&self) -> jsbind::Undefined {
        self.inner
            .call("throwIfAborted", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl AbortSignal {
    pub fn onabort(&self) -> jsbind::Any {
        self.inner.get("onabort").as_::<jsbind::Any>()
    }

    pub fn set_onabort(&mut self, value: jsbind::Any) {
        self.inner.set("onabort", value);
    }
}
