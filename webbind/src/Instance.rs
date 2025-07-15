use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Instance {
    inner: emlite::Val,
}
impl FromVal for Instance {
    fn from_val(v: &emlite::Val) -> Self {
        Instance {
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
impl core::ops::Deref for Instance {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Instance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Instance {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Instance {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Instance> for emlite::Val {
    fn from(s: Instance) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Instance> for emlite::Val {
    fn from(s: &Instance) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Instance);

impl Instance {
    pub fn new0(module: &Module) -> Instance {
        Self {
            inner: emlite::Val::global("Instance")
                .new(&[module.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(module: &Module, import_object: &Object) -> Instance {
        Self {
            inner: emlite::Val::global("Instance")
                .new(&[module.into(), import_object.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Instance {
    pub fn exports(&self) -> Object {
        self.inner.get("exports").as_::<Object>()
    }
}
