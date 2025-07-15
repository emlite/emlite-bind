use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MediaKeyStatusMap {
    inner: emlite::Val,
}
impl FromVal for MediaKeyStatusMap {
    fn from_val(v: &emlite::Val) -> Self {
        MediaKeyStatusMap {
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
impl core::ops::Deref for MediaKeyStatusMap {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MediaKeyStatusMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MediaKeyStatusMap {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MediaKeyStatusMap {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MediaKeyStatusMap> for emlite::Val {
    fn from(s: MediaKeyStatusMap) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&MediaKeyStatusMap> for emlite::Val {
    fn from(s: &MediaKeyStatusMap) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MediaKeyStatusMap);

impl MediaKeyStatusMap {
    pub fn size(&self) -> u32 {
        self.inner.get("size").as_::<u32>()
    }
}
impl MediaKeyStatusMap {
    pub fn has(&self, key_id: &Any) -> bool {
        self.inner.call("has", &[key_id.into()]).as_::<bool>()
    }
}
impl MediaKeyStatusMap {
    pub fn get(&self, key_id: &Any) -> Any {
        self.inner.call("get", &[key_id.into()]).as_::<Any>()
    }
}
