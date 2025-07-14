use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Keyboard {
    inner: EventTarget,
}
impl FromVal for Keyboard {
    fn from_val(v: &emlite::Val) -> Self {
        Keyboard {
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
impl AsRef<emlite::Val> for Keyboard {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Keyboard {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Keyboard> for emlite::Val {
    fn from(s: Keyboard) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Keyboard);

impl Keyboard {
    pub fn lock0(&self) -> jsbind::Promise {
        self.inner.call("lock", &[]).as_::<jsbind::Promise>()
    }

    pub fn lock1(&self, key_codes: jsbind::Sequence<jsbind::DOMString>) -> jsbind::Promise {
        self.inner
            .call("lock", &[key_codes.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Keyboard {
    pub fn unlock(&self) -> jsbind::Undefined {
        self.inner.call("unlock", &[]).as_::<jsbind::Undefined>()
    }
}
impl Keyboard {
    pub fn get_layout_map(&self) -> jsbind::Promise {
        self.inner
            .call("getLayoutMap", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Keyboard {
    pub fn onlayoutchange(&self) -> jsbind::Any {
        self.inner.get("onlayoutchange").as_::<jsbind::Any>()
    }

    pub fn set_onlayoutchange(&mut self, value: jsbind::Any) {
        self.inner.set("onlayoutchange", value);
    }
}
