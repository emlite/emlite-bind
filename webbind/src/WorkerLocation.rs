use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WorkerLocation {
    inner: emlite::Val,
}
impl FromVal for WorkerLocation {
    fn from_val(v: &emlite::Val) -> Self {
        WorkerLocation {
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
impl core::ops::Deref for WorkerLocation {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WorkerLocation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WorkerLocation {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WorkerLocation {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WorkerLocation> for emlite::Val {
    fn from(s: WorkerLocation) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(WorkerLocation);

impl WorkerLocation {
    pub fn href(&self) -> USVString {
        self.inner.get("href").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn origin(&self) -> USVString {
        self.inner.get("origin").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn protocol(&self) -> USVString {
        self.inner.get("protocol").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn host(&self) -> USVString {
        self.inner.get("host").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn hostname(&self) -> USVString {
        self.inner.get("hostname").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn port(&self) -> USVString {
        self.inner.get("port").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn pathname(&self) -> USVString {
        self.inner.get("pathname").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn search(&self) -> USVString {
        self.inner.get("search").as_::<USVString>()
    }
}
impl WorkerLocation {
    pub fn hash(&self) -> USVString {
        self.inner.get("hash").as_::<USVString>()
    }
}
