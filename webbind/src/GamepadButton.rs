use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GamepadButton {
    inner: emlite::Val,
}
impl FromVal for GamepadButton {
    fn from_val(v: &emlite::Val) -> Self {
        GamepadButton {
            inner: emlite::Val::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GamepadButton {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GamepadButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GamepadButton {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GamepadButton {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GamepadButton> for emlite::Val {
    fn from(s: GamepadButton) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GamepadButton> for emlite::Val {
    fn from(s: &GamepadButton) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GamepadButton);

impl GamepadButton {
    pub fn pressed(&self) -> bool {
        self.inner.get("pressed").as_::<bool>()
    }
}
impl GamepadButton {
    pub fn touched(&self) -> bool {
        self.inner.get("touched").as_::<bool>()
    }
}
impl GamepadButton {
    pub fn value(&self) -> f64 {
        self.inner.get("value").as_::<f64>()
    }
}
