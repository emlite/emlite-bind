use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerGlobalScope {
    inner: EventTarget,
}
impl FromVal for WorkerGlobalScope {
    fn from_val(v: &emlite::Val) -> Self {
        WorkerGlobalScope { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkerGlobalScope {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkerGlobalScope {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WorkerGlobalScope {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WorkerGlobalScope {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<WorkerGlobalScope> for emlite::Val {
    fn from(s: WorkerGlobalScope) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WorkerGlobalScope);


impl WorkerGlobalScope {
    pub fn self_(&self) -> WorkerGlobalScope {
        self.inner.get("self").as_::<WorkerGlobalScope>()
    }

}
impl WorkerGlobalScope {
    pub fn location(&self) -> WorkerLocation {
        self.inner.get("location").as_::<WorkerLocation>()
    }

}
impl WorkerGlobalScope {
    pub fn navigator(&self) -> WorkerNavigator {
        self.inner.get("navigator").as_::<WorkerNavigator>()
    }

}
impl WorkerGlobalScope {
    pub fn import_scripts(&self, urls: Any) -> Undefined {
        self.inner.call("importScripts", &[urls.into(), ]).as_::<Undefined>()
    }

}
impl WorkerGlobalScope {
    pub fn onerror(&self) -> Any {
        self.inner.get("onerror").as_::<Any>()
    }

    pub fn set_onerror(&mut self, value: Any) {
        self.inner.set("onerror", value);
    }

}
impl WorkerGlobalScope {
    pub fn onlanguagechange(&self) -> Any {
        self.inner.get("onlanguagechange").as_::<Any>()
    }

    pub fn set_onlanguagechange(&mut self, value: Any) {
        self.inner.set("onlanguagechange", value);
    }

}
impl WorkerGlobalScope {
    pub fn onoffline(&self) -> Any {
        self.inner.get("onoffline").as_::<Any>()
    }

    pub fn set_onoffline(&mut self, value: Any) {
        self.inner.set("onoffline", value);
    }

}
impl WorkerGlobalScope {
    pub fn ononline(&self) -> Any {
        self.inner.get("ononline").as_::<Any>()
    }

    pub fn set_ononline(&mut self, value: Any) {
        self.inner.set("ononline", value);
    }

}
impl WorkerGlobalScope {
    pub fn onrejectionhandled(&self) -> Any {
        self.inner.get("onrejectionhandled").as_::<Any>()
    }

    pub fn set_onrejectionhandled(&mut self, value: Any) {
        self.inner.set("onrejectionhandled", value);
    }

}
impl WorkerGlobalScope {
    pub fn onunhandledrejection(&self) -> Any {
        self.inner.get("onunhandledrejection").as_::<Any>()
    }

    pub fn set_onunhandledrejection(&mut self, value: Any) {
        self.inner.set("onunhandledrejection", value);
    }

}
impl WorkerGlobalScope {
    pub fn fonts(&self) -> FontFaceSet {
        self.inner.get("fonts").as_::<FontFaceSet>()
    }

}
impl WorkerGlobalScope {
    pub fn crypto(&self) -> Crypto {
        self.inner.get("crypto").as_::<Crypto>()
    }

}
