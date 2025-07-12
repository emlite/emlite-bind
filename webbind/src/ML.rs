use super::*;

#[derive(Clone, Debug)]
pub struct ML {
    inner: emlite::Val,
}
impl FromVal for ML {
    fn from_val(v: &emlite::Val) -> Self {
        ML {
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
impl std::ops::Deref for ML {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for ML {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<ML> for emlite::Val {
    fn from(s: ML) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl ML {
    pub fn create_context(&self, gpu_device: GPUDevice) -> jsbind::Promise {
        self.inner
            .call("createContext", &[gpu_device.into()])
            .as_::<jsbind::Promise>()
    }
}
