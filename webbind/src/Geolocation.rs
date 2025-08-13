use super::*;




/// The Geolocation class.
/// [`Geolocation`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Geolocation {
    inner: Any,
}

impl FromVal for Geolocation {
    fn from_val(v: &Any) -> Self {
        Geolocation { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Geolocation {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Geolocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Geolocation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Geolocation {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Geolocation> for Any {
    fn from(s: Geolocation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Geolocation> for Any {
    fn from(s: &Geolocation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Geolocation);


impl Geolocation {
    /// The getCurrentPosition method.
    /// [`Geolocation.getCurrentPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)
    pub fn get_current_position0(&self, success_callback: &Function) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), ]).as_::<Undefined>()
    }
    /// The getCurrentPosition method.
    /// [`Geolocation.getCurrentPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)
    pub fn get_current_position1(&self, success_callback: &Function, error_callback: &Function) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), error_callback.into(), ]).as_::<Undefined>()
    }
    /// The getCurrentPosition method.
    /// [`Geolocation.getCurrentPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/getCurrentPosition)
    pub fn get_current_position2(&self, success_callback: &Function, error_callback: &Function, options: &PositionOptions) -> Undefined {
        self.inner.call("getCurrentPosition", &[success_callback.into(), error_callback.into(), options.into(), ]).as_::<Undefined>()
    }
}
impl Geolocation {
    /// The watchPosition method.
    /// [`Geolocation.watchPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)
    pub fn watch_position0(&self, success_callback: &Function) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), ]).as_::<i32>()
    }
    /// The watchPosition method.
    /// [`Geolocation.watchPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)
    pub fn watch_position1(&self, success_callback: &Function, error_callback: &Function) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), error_callback.into(), ]).as_::<i32>()
    }
    /// The watchPosition method.
    /// [`Geolocation.watchPosition`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/watchPosition)
    pub fn watch_position2(&self, success_callback: &Function, error_callback: &Function, options: &PositionOptions) -> i32 {
        self.inner.call("watchPosition", &[success_callback.into(), error_callback.into(), options.into(), ]).as_::<i32>()
    }
}
impl Geolocation {
    /// The clearWatch method.
    /// [`Geolocation.clearWatch`](https://developer.mozilla.org/en-US/docs/Web/API/Geolocation/clearWatch)
    pub fn clear_watch(&self, watch_id: i32) -> Undefined {
        self.inner.call("clearWatch", &[watch_id.into(), ]).as_::<Undefined>()
    }
}
