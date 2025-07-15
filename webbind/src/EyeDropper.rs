use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ColorSelectionResult {
    inner: emlite::Val,
}
impl FromVal for ColorSelectionResult {
    fn from_val(v: &emlite::Val) -> Self {
        ColorSelectionResult { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ColorSelectionResult {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ColorSelectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ColorSelectionResult {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ColorSelectionResult {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ColorSelectionResult> for emlite::Val {
    fn from(s: ColorSelectionResult) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ColorSelectionResult> for emlite::Val {
    fn from(s: &ColorSelectionResult) -> emlite::Val {
        s.inner.clone()
    }
}

impl ColorSelectionResult {
    pub fn s_rgb_hex(&self) -> DOMString {
        self.inner.get("sRGBHex").as_::<DOMString>()
    }

    pub fn set_s_rgb_hex(&mut self, value: DOMString) {
        self.inner.set("sRGBHex", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ColorSelectionOptions {
    inner: emlite::Val,
}
impl FromVal for ColorSelectionOptions {
    fn from_val(v: &emlite::Val) -> Self {
        ColorSelectionOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ColorSelectionOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ColorSelectionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for ColorSelectionOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for ColorSelectionOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<ColorSelectionOptions> for emlite::Val {
    fn from(s: ColorSelectionOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&ColorSelectionOptions> for emlite::Val {
    fn from(s: &ColorSelectionOptions) -> emlite::Val {
        s.inner.clone()
    }
}

impl ColorSelectionOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: AbortSignal) {
        self.inner.set("signal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EyeDropper {
    inner: emlite::Val,
}
impl FromVal for EyeDropper {
    fn from_val(v: &emlite::Val) -> Self {
        EyeDropper {
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
impl core::ops::Deref for EyeDropper {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EyeDropper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for EyeDropper {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for EyeDropper {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<EyeDropper> for emlite::Val {
    fn from(s: EyeDropper) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&EyeDropper> for emlite::Val {
    fn from(s: &EyeDropper) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EyeDropper);

impl EyeDropper {
    pub fn new() -> EyeDropper {
        Self {
            inner: emlite::Val::global("EyeDropper")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl EyeDropper {
    pub fn open0(&self) -> Promise {
        self.inner.call("open", &[]).as_::<Promise>()
    }

    pub fn open1(&self, options: ColorSelectionOptions) -> Promise {
        self.inner.call("open", &[options.into()]).as_::<Promise>()
    }
}
