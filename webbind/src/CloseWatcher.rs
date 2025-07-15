use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CloseWatcher {
    inner: EventTarget,
}
impl FromVal for CloseWatcher {
    fn from_val(v: &emlite::Val) -> Self {
        CloseWatcher {
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
impl core::ops::Deref for CloseWatcher {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CloseWatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for CloseWatcher {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CloseWatcher {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CloseWatcher> for emlite::Val {
    fn from(s: CloseWatcher) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CloseWatcher> for emlite::Val {
    fn from(s: &CloseWatcher) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CloseWatcher);

impl CloseWatcher {
    pub fn new0() -> CloseWatcher {
        Self {
            inner: emlite::Val::global("CloseWatcher")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(options: &Any) -> CloseWatcher {
        Self {
            inner: emlite::Val::global("CloseWatcher")
                .new(&[options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl CloseWatcher {
    pub fn request_close(&self) -> Undefined {
        self.inner.call("requestClose", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl CloseWatcher {
    pub fn oncancel(&self) -> Any {
        self.inner.get("oncancel").as_::<Any>()
    }

    pub fn set_oncancel(&mut self, value: &Any) {
        self.inner.set("oncancel", value);
    }
}
impl CloseWatcher {
    pub fn onclose(&self) -> Any {
        self.inner.get("onclose").as_::<Any>()
    }

    pub fn set_onclose(&mut self, value: &Any) {
        self.inner.set("onclose", value);
    }
}
