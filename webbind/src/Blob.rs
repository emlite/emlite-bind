use super::*;

/// The Blob class.
/// [`Blob`](https://developer.mozilla.org/en-US/docs/Web/API/Blob)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Blob {
    inner: Any,
}
impl FromVal for Blob {
    fn from_val(v: &Any) -> Self {
        Blob {
            inner: Any::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Blob {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Blob {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for Blob {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for Blob {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<Blob> for Any {
    fn from(s: Blob) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&Blob> for Any {
    fn from(s: &Blob) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Blob);

impl Blob {
    /// The `new Blob(..)` constructor, creating a new Blob instance
    pub fn new0() -> Blob {
        Self {
            inner: Any::global("Blob").new(&[]).as_::<Any>(),
        }
    }

    /// The `new Blob(..)` constructor, creating a new Blob instance
    pub fn new1(blob_parts: &Sequence<Any>) -> Blob {
        Self {
            inner: Any::global("Blob").new(&[blob_parts.into()]).as_::<Any>(),
        }
    }

    /// The `new Blob(..)` constructor, creating a new Blob instance
    pub fn new2(blob_parts: &Sequence<Any>, options: &Any) -> Blob {
        Self {
            inner: Any::global("Blob")
                .new(&[blob_parts.into(), options.into()])
                .as_::<Any>(),
        }
    }
}
impl Blob {
    /// Getter of the `size` attribute.
    /// [`Blob.size`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/size)
    pub fn size(&self) -> u64 {
        self.inner.get("size").as_::<u64>()
    }
}
impl Blob {
    /// Getter of the `type` attribute.
    /// [`Blob.type`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/type)
    pub fn type_(&self) -> String {
        self.inner.get("type").as_::<String>()
    }
}
impl Blob {
    /// The slice method.
    /// [`Blob.slice`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    pub fn slice0(&self) -> Blob {
        self.inner.call("slice", &[]).as_::<Blob>()
    }
    /// The slice method.
    /// [`Blob.slice`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    pub fn slice1(&self, start: i64) -> Blob {
        self.inner.call("slice", &[start.into()]).as_::<Blob>()
    }
    /// The slice method.
    /// [`Blob.slice`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    pub fn slice2(&self, start: i64, end: i64) -> Blob {
        self.inner
            .call("slice", &[start.into(), end.into()])
            .as_::<Blob>()
    }
    /// The slice method.
    /// [`Blob.slice`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/slice)
    pub fn slice3(&self, start: i64, end: i64, content_type: &str) -> Blob {
        self.inner
            .call("slice", &[start.into(), end.into(), content_type.into()])
            .as_::<Blob>()
    }
}
impl Blob {
    /// The stream method.
    /// [`Blob.stream`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/stream)
    pub fn stream(&self) -> ReadableStream {
        self.inner.call("stream", &[]).as_::<ReadableStream>()
    }
}
impl Blob {
    /// The text method.
    /// [`Blob.text`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/text)
    pub fn text(&self) -> Promise {
        self.inner.call("text", &[]).as_::<Promise>()
    }
}
impl Blob {
    /// The arrayBuffer method.
    /// [`Blob.arrayBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/arrayBuffer)
    pub fn array_buffer(&self) -> Promise {
        self.inner.call("arrayBuffer", &[]).as_::<Promise>()
    }
}
impl Blob {
    /// The bytes method.
    /// [`Blob.bytes`](https://developer.mozilla.org/en-US/docs/Web/API/Blob/bytes)
    pub fn bytes(&self) -> Promise {
        self.inner.call("bytes", &[]).as_::<Promise>()
    }
}
