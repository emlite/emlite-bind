use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for NDEFScanOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NDEFScanOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NDEFScanOptions> for emlite::Val {
    fn from(s: NDEFScanOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for NDEFWriteOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NDEFWriteOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NDEFWriteOptions> for emlite::Val {
    fn from(s: NDEFWriteOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for NDEFMakeReadOnlyOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NDEFMakeReadOnlyOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NDEFMakeReadOnlyOptions> for emlite::Val {
    fn from(s: NDEFMakeReadOnlyOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for NDEFReader {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for NDEFReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<NDEFReader> for emlite::Val {
    fn from(s: NDEFReader) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn onreading(&self) -> jsbind::Any {
        self.inner.get("onreading").as_::<jsbind::Any>()
    }

    pub fn set_onreading(&mut self, value: jsbind::Any) {
        self.inner.set("onreading", value);
    }
}
impl NDEFReader {
    pub fn onreadingerror(&self) -> jsbind::Any {
        self.inner.get("onreadingerror").as_::<jsbind::Any>()
    }

    pub fn set_onreadingerror(&mut self, value: jsbind::Any) {
        self.inner.set("onreadingerror", value);
    }
}
impl NDEFReader {
    pub fn scan0(&self) -> jsbind::Promise {
        self.inner.call("scan", &[]).as_::<jsbind::Promise>()
    }

    pub fn scan1(&self, options: NDEFScanOptions) -> jsbind::Promise {
        self.inner
            .call("scan", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl NDEFReader {
    pub fn write0(&self, message: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("write", &[message.into()])
            .as_::<jsbind::Promise>()
    }

    pub fn write1(&self, message: jsbind::Any, options: NDEFWriteOptions) -> jsbind::Promise {
        self.inner
            .call("write", &[message.into(), options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl NDEFReader {
    pub fn make_read_only0(&self) -> jsbind::Promise {
        self.inner
            .call("makeReadOnly", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn make_read_only1(&self, options: NDEFMakeReadOnlyOptions) -> jsbind::Promise {
        self.inner
            .call("makeReadOnly", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
