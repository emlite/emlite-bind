use super::*;

/// The Keyboard class.
/// [`Keyboard`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Keyboard {
    inner: EventTarget,
}
impl FromVal for Keyboard {
    fn from_val(v: &Any) -> Self {
        Keyboard {
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
impl core::ops::Deref for Keyboard {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Keyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Keyboard {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Keyboard {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Keyboard> for Any {
    fn from(s: Keyboard) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Keyboard> for Any {
    fn from(s: &Keyboard) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Keyboard);

impl Keyboard {
    /// The lock method.
    /// [`Keyboard.lock`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/lock)
    pub fn lock0(&self) -> Promise {
        self.inner.call("lock", &[]).as_::<Promise>()
    }
    /// The lock method.
    /// [`Keyboard.lock`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/lock)
    pub fn lock1(&self, key_codes: &Sequence<String>) -> Promise {
        self.inner
            .call("lock", &[key_codes.into()])
            .as_::<Promise>()
    }
}
impl Keyboard {
    /// The unlock method.
    /// [`Keyboard.unlock`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/unlock)
    pub fn unlock(&self) -> Undefined {
        self.inner.call("unlock", &[]).as_::<Undefined>()
    }
}
impl Keyboard {
    /// The getLayoutMap method.
    /// [`Keyboard.getLayoutMap`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/getLayoutMap)
    pub fn get_layout_map(&self) -> Promise {
        self.inner.call("getLayoutMap", &[]).as_::<Promise>()
    }
}
impl Keyboard {
    /// Getter of the `onlayoutchange` attribute.
    /// [`Keyboard.onlayoutchange`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/onlayoutchange)
    pub fn onlayoutchange(&self) -> Any {
        self.inner.get("onlayoutchange").as_::<Any>()
    }

    /// Setter of the `onlayoutchange` attribute.
    /// [`Keyboard.onlayoutchange`](https://developer.mozilla.org/en-US/docs/Web/API/Keyboard/onlayoutchange)
    pub fn set_onlayoutchange(&mut self, value: &Any) {
        self.inner.set("onlayoutchange", value);
    }
}
