use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ClipboardUnsanitizedFormats {
    inner: emlite::Val,
}
impl FromVal for ClipboardUnsanitizedFormats {
    fn from_val(v: &emlite::Val) -> Self {
        ClipboardUnsanitizedFormats { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ClipboardUnsanitizedFormats {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ClipboardUnsanitizedFormats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ClipboardUnsanitizedFormats {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ClipboardUnsanitizedFormats {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ClipboardUnsanitizedFormats> for emlite::Val {
    fn from(s: ClipboardUnsanitizedFormats) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ClipboardUnsanitizedFormats> for emlite::Val {
    fn from(s: &ClipboardUnsanitizedFormats) -> emlite::Val {
        s.inner.clone()
    }
}

impl ClipboardUnsanitizedFormats {
    pub fn unsanitized(&self) -> Sequence<String> {
        self.inner.get("unsanitized").as_::<Sequence<String>>()
    }

    pub fn set_unsanitized(&mut self, value: &Sequence<String>) {
        self.inner.set("unsanitized", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Clipboard {
    inner: EventTarget,
}
impl FromVal for Clipboard {
    fn from_val(v: &emlite::Val) -> Self {
        Clipboard {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for Clipboard {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Clipboard {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Clipboard> for emlite::Val {
    fn from(s: Clipboard) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Clipboard> for emlite::Val {
    fn from(s: &Clipboard) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Clipboard);

impl Clipboard {
    pub fn read0(&self) -> Promise {
        self.inner.call("read", &[]).as_::<Promise>()
    }

    pub fn read1(&self, formats: &ClipboardUnsanitizedFormats) -> Promise {
        self.inner.call("read", &[formats.into()]).as_::<Promise>()
    }
}
impl Clipboard {
    pub fn read_text(&self) -> Promise {
        self.inner.call("readText", &[]).as_::<Promise>()
    }
}
impl Clipboard {
    pub fn write(&self, data: &Any) -> Promise {
        self.inner.call("write", &[data.into()]).as_::<Promise>()
    }
}
impl Clipboard {
    pub fn write_text(&self, data: &str) -> Promise {
        self.inner
            .call("writeText", &[data.into()])
            .as_::<Promise>()
    }
}
