use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SharedWorker {
    inner: EventTarget,
}
impl FromVal for SharedWorker {
    fn from_val(v: &emlite::Val) -> Self {
        SharedWorker {
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
impl core::ops::Deref for SharedWorker {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SharedWorker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SharedWorker {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SharedWorker {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SharedWorker> for emlite::Val {
    fn from(s: SharedWorker) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&SharedWorker> for emlite::Val {
    fn from(s: &SharedWorker) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SharedWorker);

impl SharedWorker {
    pub fn new0(script_url: &Any) -> SharedWorker {
        Self {
            inner: emlite::Val::global("SharedWorker")
                .new(&[script_url.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(script_url: &Any, options: &Any) -> SharedWorker {
        Self {
            inner: emlite::Val::global("SharedWorker")
                .new(&[script_url.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl SharedWorker {
    pub fn port(&self) -> Any {
        self.inner.get("port").as_::<Any>()
    }
}
impl SharedWorker {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: &Any) {
        self.inner.set("onerror", value);
    }
}
