use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFRecord {
    inner: emlite::Val,
}
impl FromVal for NDEFRecord {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFRecord {
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
impl core::ops::Deref for NDEFRecord {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFRecord {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFRecord {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFRecord {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NDEFRecord> for emlite::Val {
    fn from(s: NDEFRecord) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NDEFRecord);

impl NDEFRecord {
    pub fn new(record_init: Any) -> NDEFRecord {
        Self {
            inner: emlite::Val::global("NDEFRecord")
                .new(&[record_init.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl NDEFRecord {
    pub fn record_type(&self) -> USVString {
        self.inner.get("recordType").as_::<USVString>()
    }
}
impl NDEFRecord {
    pub fn media_type(&self) -> USVString {
        self.inner.get("mediaType").as_::<USVString>()
    }
}
impl NDEFRecord {
    pub fn id(&self) -> USVString {
        self.inner.get("id").as_::<USVString>()
    }
}
impl NDEFRecord {
    pub fn data(&self) -> DataView {
        self.inner.get("data").as_::<DataView>()
    }
}
impl NDEFRecord {
    pub fn encoding(&self) -> USVString {
        self.inner.get("encoding").as_::<USVString>()
    }
}
impl NDEFRecord {
    pub fn lang(&self) -> USVString {
        self.inner.get("lang").as_::<USVString>()
    }
}
impl NDEFRecord {
    pub fn to_records(&self) -> Sequence<NDEFRecord> {
        self.inner
            .call("toRecords", &[])
            .as_::<Sequence<NDEFRecord>>()
    }
}
