use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct BlobEventInit {
    inner: Any,
}
impl FromVal for BlobEventInit {
    fn from_val(v: &Any) -> Self {
        BlobEventInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for BlobEventInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for BlobEventInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for BlobEventInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for BlobEventInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<BlobEventInit> for Any {
    fn from(s: BlobEventInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&BlobEventInit> for Any {
    fn from(s: &BlobEventInit) -> Any {
        s.inner.clone()
    }
}

impl BlobEventInit {
    pub fn data(&self) -> Blob {
        self.inner.get("data").as_::<Blob>()
    }

    pub fn set_data(&mut self, value: &Blob) {
        self.inner.set("data", value);
    }
}
impl BlobEventInit {
    pub fn timecode(&self) -> Any {
        self.inner.get("timecode").as_::<Any>()
    }

    pub fn set_timecode(&mut self, value: &Any) {
        self.inner.set("timecode", value);
    }
}
