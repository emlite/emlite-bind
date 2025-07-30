use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ColorSelectionResult {
    inner: Any,
}
impl FromVal for ColorSelectionResult {
    fn from_val(v: &Any) -> Self {
        ColorSelectionResult { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ColorSelectionResult {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ColorSelectionResult {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ColorSelectionResult {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ColorSelectionResult {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ColorSelectionResult> for Any {
    fn from(s: ColorSelectionResult) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ColorSelectionResult> for Any {
    fn from(s: &ColorSelectionResult) -> Any {
        s.inner.clone()
    }
}

impl ColorSelectionResult {
    pub fn s_rgb_hex(&self) -> JsString {
        self.inner.get("sRGBHex").as_::<JsString>()
    }

    pub fn set_s_rgb_hex(&mut self, value: &JsString) {
        self.inner.set("sRGBHex", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct ColorSelectionOptions {
    inner: Any,
}
impl FromVal for ColorSelectionOptions {
    fn from_val(v: &Any) -> Self {
        ColorSelectionOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for ColorSelectionOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for ColorSelectionOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for ColorSelectionOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for ColorSelectionOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<ColorSelectionOptions> for Any {
    fn from(s: ColorSelectionOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&ColorSelectionOptions> for Any {
    fn from(s: &ColorSelectionOptions) -> Any {
        s.inner.clone()
    }
}

impl ColorSelectionOptions {
    pub fn signal(&self) -> AbortSignal {
        self.inner.get("signal").as_::<AbortSignal>()
    }

    pub fn set_signal(&mut self, value: &AbortSignal) {
        self.inner.set("signal", value);
    }
}
/// The EyeDropper class.
/// [`EyeDropper`](https://developer.mozilla.org/en-US/docs/Web/API/EyeDropper)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct EyeDropper {
    inner: Any,
}
impl FromVal for EyeDropper {
    fn from_val(v: &Any) -> Self {
        EyeDropper {
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
impl core::ops::Deref for EyeDropper {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for EyeDropper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for EyeDropper {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for EyeDropper {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<EyeDropper> for Any {
    fn from(s: EyeDropper) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&EyeDropper> for Any {
    fn from(s: &EyeDropper) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(EyeDropper);

impl EyeDropper {
    /// The `new EyeDropper(..)` constructor, creating a new EyeDropper instance
    pub fn new() -> EyeDropper {
        Self {
            inner: Any::global("EyeDropper").new(&[]).as_::<Any>(),
        }
    }
}
impl EyeDropper {
    /// The open method.
    /// [`EyeDropper.open`](https://developer.mozilla.org/en-US/docs/Web/API/EyeDropper/open)
    pub fn open0(&self) -> Promise<ColorSelectionResult> {
        self.inner
            .call("open", &[])
            .as_::<Promise<ColorSelectionResult>>()
    }
    /// The open method.
    /// [`EyeDropper.open`](https://developer.mozilla.org/en-US/docs/Web/API/EyeDropper/open)
    pub fn open1(&self, options: &ColorSelectionOptions) -> Promise<ColorSelectionResult> {
        self.inner
            .call("open", &[options.into()])
            .as_::<Promise<ColorSelectionResult>>()
    }
}
