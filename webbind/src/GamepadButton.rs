use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for GamepadButton {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for GamepadButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<GamepadButton> for emlite::Val {
    fn from(s: GamepadButton) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
