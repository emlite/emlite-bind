use super::*;

/// The ScreenDetails class.
/// [`ScreenDetails`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ScreenDetails {
    inner: EventTarget,
}
impl FromVal for ScreenDetails {
    fn from_val(v: &Any) -> Self {
        ScreenDetails {
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
impl core::ops::Deref for ScreenDetails {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ScreenDetails {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ScreenDetails {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ScreenDetails {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ScreenDetails> for Any {
    fn from(s: ScreenDetails) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ScreenDetails> for Any {
    fn from(s: &ScreenDetails) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(ScreenDetails);

impl ScreenDetails {
    /// Getter of the `screens` attribute.
    /// [`ScreenDetails.screens`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/screens)
    pub fn screens(&self) -> FrozenArray<ScreenDetailed> {
        self.inner
            .get("screens")
            .as_::<FrozenArray<ScreenDetailed>>()
    }
}
impl ScreenDetails {
    /// Getter of the `currentScreen` attribute.
    /// [`ScreenDetails.currentScreen`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/currentScreen)
    pub fn current_screen(&self) -> ScreenDetailed {
        self.inner.get("currentScreen").as_::<ScreenDetailed>()
    }
}
impl ScreenDetails {
    /// Getter of the `onscreenschange` attribute.
    /// [`ScreenDetails.onscreenschange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/onscreenschange)
    pub fn onscreenschange(&self) -> Any {
        self.inner.get("onscreenschange").as_::<Any>()
    }

    /// Setter of the `onscreenschange` attribute.
    /// [`ScreenDetails.onscreenschange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/onscreenschange)
    pub fn set_onscreenschange(&mut self, value: &Any) {
        self.inner.set("onscreenschange", value);
    }
}
impl ScreenDetails {
    /// Getter of the `oncurrentscreenchange` attribute.
    /// [`ScreenDetails.oncurrentscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/oncurrentscreenchange)
    pub fn oncurrentscreenchange(&self) -> Any {
        self.inner.get("oncurrentscreenchange").as_::<Any>()
    }

    /// Setter of the `oncurrentscreenchange` attribute.
    /// [`ScreenDetails.oncurrentscreenchange`](https://developer.mozilla.org/en-US/docs/Web/API/ScreenDetails/oncurrentscreenchange)
    pub fn set_oncurrentscreenchange(&mut self, value: &Any) {
        self.inner.set("oncurrentscreenchange", value);
    }
}
