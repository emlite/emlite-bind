use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct WorkletOptions {
    inner: emlite::Val,
}
impl FromVal for WorkletOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WorkletOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WorkletOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkletOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<WorkletOptions> for emlite::Val {
    fn from(s: WorkletOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WorkletOptions {
    pub fn credentials(&self) -> RequestCredentials {
        self.inner.get("credentials").as_::<RequestCredentials>()
    }

    pub fn set_credentials(&mut self, value: RequestCredentials) {
        self.inner.set("credentials", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Worklet {
    inner: emlite::Val,
}
impl FromVal for Worklet {
    fn from_val(v: &emlite::Val) -> Self {
        Worklet {
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
impl core::ops::Deref for Worklet {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Worklet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<Worklet> for emlite::Val {
    fn from(s: Worklet) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl Worklet {
    pub fn add_module0(&self, module_url: jsbind::USVString) -> jsbind::Promise {
        self.inner
            .call("addModule", &[module_url.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn add_module1(
        &self,
        module_url: jsbind::USVString,
        options: WorkletOptions,
    ) -> jsbind::Promise {
        self.inner
            .call("addModule", &[module_url.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
