use super::*;




/// The WorkerLocation class.
/// [`WorkerLocation`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerLocation {
    inner: Any,
}

impl FromVal for WorkerLocation {
    fn from_val(v: &Any) -> Self {
        WorkerLocation { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for WorkerLocation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for WorkerLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for WorkerLocation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for WorkerLocation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<WorkerLocation> for Any {
    fn from(s: WorkerLocation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&WorkerLocation> for Any {
    fn from(s: &WorkerLocation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(WorkerLocation);


impl WorkerLocation {
    /// Getter of the `href` attribute.
    /// [`WorkerLocation.href`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/href)
    pub fn href(&self) -> JsString {
        self.inner.get("href").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `origin` attribute.
    /// [`WorkerLocation.origin`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/origin)
    pub fn origin(&self) -> JsString {
        self.inner.get("origin").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `protocol` attribute.
    /// [`WorkerLocation.protocol`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/protocol)
    pub fn protocol(&self) -> JsString {
        self.inner.get("protocol").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `host` attribute.
    /// [`WorkerLocation.host`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/host)
    pub fn host(&self) -> JsString {
        self.inner.get("host").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `hostname` attribute.
    /// [`WorkerLocation.hostname`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hostname)
    pub fn hostname(&self) -> JsString {
        self.inner.get("hostname").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `port` attribute.
    /// [`WorkerLocation.port`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/port)
    pub fn port(&self) -> JsString {
        self.inner.get("port").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `pathname` attribute.
    /// [`WorkerLocation.pathname`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/pathname)
    pub fn pathname(&self) -> JsString {
        self.inner.get("pathname").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `search` attribute.
    /// [`WorkerLocation.search`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/search)
    pub fn search(&self) -> JsString {
        self.inner.get("search").as_::<JsString>()
    }

}
impl WorkerLocation {
    /// Getter of the `hash` attribute.
    /// [`WorkerLocation.hash`](https://developer.mozilla.org/en-US/docs/Web/API/WorkerLocation/hash)
    pub fn hash(&self) -> JsString {
        self.inner.get("hash").as_::<JsString>()
    }

}
