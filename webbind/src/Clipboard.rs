use super::*;




/// The Clipboard class.
/// [`Clipboard`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Clipboard {
    inner: EventTarget,
}

impl FromVal for Clipboard {
    fn from_val(v: &Any) -> Self {
        Clipboard { inner: EventTarget::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Clipboard {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Clipboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Clipboard {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Clipboard {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Clipboard> for Any {
    fn from(s: Clipboard) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Clipboard> for Any {
    fn from(s: &Clipboard) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Clipboard);


impl Clipboard {
    /// The read method.
    /// [`Clipboard.read`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read)
    pub fn read0(&self, ) -> Promise<Any> {
        self.inner.call("read", &[]).as_::<Promise<Any>>()
    }
    /// The read method.
    /// [`Clipboard.read`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/read)
    pub fn read1(&self, formats: &ClipboardUnsanitizedFormats) -> Promise<Any> {
        self.inner.call("read", &[formats.into(), ]).as_::<Promise<Any>>()
    }
}
impl Clipboard {
    /// The readText method.
    /// [`Clipboard.readText`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/readText)
    pub fn read_text(&self, ) -> Promise<JsString> {
        self.inner.call("readText", &[]).as_::<Promise<JsString>>()
    }
}
impl Clipboard {
    /// The write method.
    /// [`Clipboard.write`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/write)
    pub fn write(&self, data: &Any) -> Promise<Undefined> {
        self.inner.call("write", &[data.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl Clipboard {
    /// The writeText method.
    /// [`Clipboard.writeText`](https://developer.mozilla.org/en-US/docs/Web/API/Clipboard/writeText)
    pub fn write_text(&self, data: &JsString) -> Promise<Undefined> {
        self.inner.call("writeText", &[data.into(), ]).as_::<Promise<Undefined>>()
    }
}
