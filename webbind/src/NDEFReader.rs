use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFScanOptions {
    inner: emlite::Val,
}
impl FromVal for NDEFScanOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFScanOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFScanOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFScanOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFScanOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NDEFScanOptions> for emlite::Val {
    fn from(s: NDEFScanOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NDEFScanOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFWriteOptions {
    inner: emlite::Val,
}
impl FromVal for NDEFWriteOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFWriteOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFWriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFWriteOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFWriteOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NDEFWriteOptions> for emlite::Val {
    fn from(s: NDEFWriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NDEFWriteOptions {
    pub fn overwrite(&self) -> bool {
        self.inner.get("overwrite").as_::<bool>()
    }

    pub fn set_overwrite(&mut self, value: bool) {
        self.inner.set("overwrite", value);
    }
}
impl NDEFWriteOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFMakeReadOnlyOptions {
    inner: emlite::Val,
}
impl FromVal for NDEFMakeReadOnlyOptions {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFMakeReadOnlyOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for NDEFMakeReadOnlyOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFMakeReadOnlyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFMakeReadOnlyOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFMakeReadOnlyOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NDEFMakeReadOnlyOptions> for emlite::Val {
    fn from(s: NDEFMakeReadOnlyOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl NDEFMakeReadOnlyOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct NDEFReader {
    inner: EventTarget,
}
impl FromVal for NDEFReader {
    fn from_val(v: &emlite::Val) -> Self {
        NDEFReader {
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
impl core::ops::Deref for NDEFReader {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for NDEFReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for NDEFReader {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for NDEFReader {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<NDEFReader> for emlite::Val {
    fn from(s: NDEFReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(NDEFReader);

impl NDEFReader {
    pub fn new() -> NDEFReader {
        Self {
            inner: emlite::Val::global("NDEFReader")
                .new(&[])
                .as_::<EventTarget>(),
        }
    }
}
impl NDEFReader {
    pub fn onreading(&self) -> Any {
        self.inner.get("onreading").as_::<Any>()
    }

    pub fn set_onreading(&mut self, value: Any) {
        self.inner.set("onreading", value);
    }
}
impl NDEFReader {
    pub fn onreadingerror(&self) -> Any {
        self.inner.get("onreadingerror").as_::<Any>()
    }

    pub fn set_onreadingerror(&mut self, value: Any) {
        self.inner.set("onreadingerror", value);
    }
}
impl NDEFReader {
    pub fn scan0(&self) -> Promise {
        self.inner.call("scan", &[]).as_::<Promise>()
    }

    pub fn scan1(&self, options: NDEFScanOptions) -> Promise {
        self.inner.call("scan", &[options.into()]).as_::<Promise>()
    }
}
impl NDEFReader {
    pub fn write0(&self, message: Any) -> Promise {
        self.inner.call("write", &[message.into()]).as_::<Promise>()
    }

    pub fn write1(&self, message: Any, options: NDEFWriteOptions) -> Promise {
        self.inner
            .call("write", &[message.into(), options.into()])
            .as_::<Promise>()
    }
}
impl NDEFReader {
    pub fn make_read_only0(&self) -> Promise {
        self.inner.call("makeReadOnly", &[]).as_::<Promise>()
    }

    pub fn make_read_only1(&self, options: NDEFMakeReadOnlyOptions) -> Promise {
        self.inner
            .call("makeReadOnly", &[options.into()])
            .as_::<Promise>()
    }
}
