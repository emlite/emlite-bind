use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Global {
    inner: emlite::Val,
}
impl FromVal for Global {
    fn from_val(v: &emlite::Val) -> Self {
        Global {
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
impl core::ops::Deref for Global {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Global {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Global {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Global {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Global> for emlite::Val {
    fn from(s: Global) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Global);

impl Global {
    pub fn new0(descriptor: jsbind::Any) -> Global {
        Self {
            inner: emlite::Val::global("Global")
                .new(&[descriptor.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(descriptor: jsbind::Any, v: jsbind::Any) -> Global {
        Self {
            inner: emlite::Val::global("Global")
                .new(&[descriptor.into(), v.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Global {
    pub fn value_of(&self) -> jsbind::Any {
        self.inner.call("valueOf", &[]).as_::<jsbind::Any>()
    }
}
impl Global {
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }

    pub fn set_value(&mut self, value: jsbind::Any) {
        self.inner.set("value", value);
    }
}
