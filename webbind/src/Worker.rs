use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Worker {
    inner: EventTarget,
}
impl FromVal for Worker {
    fn from_val(v: &emlite::Val) -> Self {
        Worker {
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
impl core::ops::Deref for Worker {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Worker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Worker {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Worker {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Worker> for emlite::Val {
    fn from(s: Worker) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Worker);

impl Worker {
    pub fn new0(script_url: jsbind::Any) -> Worker {
        Self {
            inner: emlite::Val::global("Worker")
                .new(&[script_url.into()])
                .as_::<EventTarget>(),
        }
    }

    pub fn new1(script_url: jsbind::Any, options: jsbind::Any) -> Worker {
        Self {
            inner: emlite::Val::global("Worker")
                .new(&[script_url.into(), options.into()])
                .as_::<EventTarget>(),
        }
    }
}
impl Worker {
    pub fn terminate(&self) -> jsbind::Undefined {
        self.inner.call("terminate", &[]).as_::<jsbind::Undefined>()
    }
}
impl Worker {
    pub fn post_message0(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn post_message1(
        &self,
        message: jsbind::Any,
        options: StructuredSerializeOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Worker {
    pub fn onerror(&self) -> jsbind::Any {
        self.inner.get("onerror").as_::<jsbind::Any>()
    }

    pub fn set_onerror(&mut self, value: jsbind::Any) {
        self.inner.set("onerror", value);
    }
}
impl Worker {
    pub fn onmessage(&self) -> jsbind::Any {
        self.inner.get("onmessage").as_::<jsbind::Any>()
    }

    pub fn set_onmessage(&mut self, value: jsbind::Any) {
        self.inner.set("onmessage", value);
    }
}
impl Worker {
    pub fn onmessageerror(&self) -> jsbind::Any {
        self.inner.get("onmessageerror").as_::<jsbind::Any>()
    }

    pub fn set_onmessageerror(&mut self, value: jsbind::Any) {
        self.inner.set("onmessageerror", value);
    }
}
