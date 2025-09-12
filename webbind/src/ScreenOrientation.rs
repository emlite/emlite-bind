use super::*;

/// The ScreenOrientation class.
/// [`ScreenOrientation`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScreenOrientation {
    inner: EventTarget,
}

impl FromVal for ScreenOrientation {
    fn from_val(v: &Any) -> Self {
        ScreenOrientation {
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

impl core::ops::Deref for ScreenOrientation {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for ScreenOrientation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for ScreenOrientation {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for ScreenOrientation {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<ScreenOrientation> for Any {
    fn from(s: ScreenOrientation) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&ScreenOrientation> for Any {
    fn from(s: &ScreenOrientation) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(ScreenOrientation);

impl ScreenOrientation {
    /// Getter of the `type` attribute.
    /// [`ScreenOrientation.type`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/type)
    pub fn type_(&self) -> OrientationType {
        self.inner.get("type").as_::<OrientationType>()
    }
}
impl ScreenOrientation {
    /// Getter of the `angle` attribute.
    /// [`ScreenOrientation.angle`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/angle)
    pub fn angle(&self) -> u16 {
        self.inner.get("angle").as_::<u16>()
    }
}
impl ScreenOrientation {
    /// Getter of the `onchange` attribute.
    /// [`ScreenOrientation.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`ScreenOrientation.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
impl ScreenOrientation {
    /// The lock method.
    /// [`ScreenOrientation.lock`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/lock)
    pub fn lock(&self, orientation: &OrientationLockType) -> Promise<Undefined> {
        self.inner
            .call("lock", &[orientation.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl ScreenOrientation {
    /// The unlock method.
    /// [`ScreenOrientation.unlock`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenOrientation/unlock)
    pub fn unlock(&self) -> Undefined {
        self.inner.call("unlock", &[]).as_::<Undefined>()
    }
}
