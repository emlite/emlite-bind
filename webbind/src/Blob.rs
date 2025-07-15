use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Blob {
    inner: emlite::Val,
}
impl FromVal for Blob {
    fn from_val(v: &emlite::Val) -> Self {
        Blob {
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
impl core::ops::Deref for Blob {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Blob {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Blob {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Blob> for emlite::Val {
    fn from(s: Blob) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Blob> for emlite::Val {
    fn from(s: &Blob) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Blob);

impl Blob {
    pub fn new0() -> Blob {
        Self {
            inner: emlite::Val::global("Blob").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(blob_parts: &Sequence<Any>) -> Blob {
        Self {
            inner: emlite::Val::global("Blob")
                .new(&[blob_parts.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(blob_parts: &Sequence<Any>, options: &Any) -> Blob {
        Self {
            inner: emlite::Val::global("Blob")
                .new(&[blob_parts.into(), options.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Blob {
    pub fn size(&self) -> u64 {
        self.inner.get("size").as_::<u64>()
    }
}
impl Blob {
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }
}
impl Blob {
    pub fn slice0(&self) -> Blob {
        self.inner.call("slice", &[]).as_::<Blob>()
    }

    pub fn slice1(&self, start: i64) -> Blob {
        self.inner.call("slice", &[start.into()]).as_::<Blob>()
    }

    pub fn slice2(&self, start: i64, end: i64) -> Blob {
        self.inner
            .call("slice", &[start.into(), end.into()])
            .as_::<Blob>()
    }

    pub fn slice3(&self, start: i64, end: i64, content_type: &str) -> Blob {
        self.inner
            .call("slice", &[start.into(), end.into(), content_type.into()])
            .as_::<Blob>()
    }
}
impl Blob {
    pub fn stream(&self) -> ReadableStream {
        self.inner.call("stream", &[]).as_::<ReadableStream>()
    }
}
impl Blob {
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
}
impl Blob {
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
    }
}
impl Blob {
    pub fn bytes(&self) -> Promise {
        self.inner.call("bytes", &[]).as_::<Promise>()
    }
}
