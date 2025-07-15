use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSessionInit {
    inner: emlite::Val,
}
impl FromVal for XRSessionInit {
    fn from_val(v: &emlite::Val) -> Self {
        XRSessionInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSessionInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSessionInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRSessionInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRSessionInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRSessionInit> for emlite::Val {
    fn from(s: XRSessionInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl XRSessionInit {
    pub fn required_features(&self) -> Sequence<DOMString> {
        self.inner.get("requiredFeatures").as_::<Sequence<DOMString>>()
    }

    pub fn set_required_features(&mut self, value: Sequence<DOMString>) {
        self.inner.set("requiredFeatures", value);
    }

}
impl XRSessionInit {
    pub fn optional_features(&self) -> Sequence<DOMString> {
        self.inner.get("optionalFeatures").as_::<Sequence<DOMString>>()
    }

    pub fn set_optional_features(&mut self, value: Sequence<DOMString>) {
        self.inner.set("optionalFeatures", value);
    }

}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRSystem {
    inner: EventTarget,
}
impl FromVal for XRSystem {
    fn from_val(v: &emlite::Val) -> Self {
        XRSystem { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for XRSystem {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for XRSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for XRSystem {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for XRSystem {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<XRSystem> for emlite::Val {
    fn from(s: XRSystem) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(XRSystem);


impl XRSystem {
    pub fn is_session_supported(&self, mode: XRSessionMode) -> Promise {
        self.inner.call("isSessionSupported", &[mode.into(), ]).as_::<Promise>()
    }

}
impl XRSystem {
    pub fn request_session0(&self, mode: XRSessionMode) -> Promise {
        self.inner.call("requestSession", &[mode.into(), ]).as_::<Promise>()
    }

    pub fn request_session1(&self, mode: XRSessionMode, options: XRSessionInit) -> Promise {
        self.inner.call("requestSession", &[mode.into(), options.into(), ]).as_::<Promise>()
    }

}
impl XRSystem {
    pub fn ondevicechange(&self) -> Any {
        self.inner.get("ondevicechange").as_::<Any>()
    }

    pub fn set_ondevicechange(&mut self, value: Any) {
        self.inner.set("ondevicechange", value);
    }

}
