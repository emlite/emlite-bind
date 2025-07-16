use super::*;

/// The CSSColor class.
/// [`CSSColor`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSColor {
    inner: CSSColorValue,
}
impl FromVal for CSSColor {
    fn from_val(v: &Any) -> Self {
        CSSColor {
            inner: CSSColorValue::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for CSSColor {
    type Target = CSSColorValue;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for CSSColor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for CSSColor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for CSSColor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<CSSColor> for Any {
    fn from(s: CSSColor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&CSSColor> for Any {
    fn from(s: &CSSColor) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSColor);

impl CSSColor {
    /// The `new CSSColor(..)` constructor, creating a new CSSColor instance
    pub fn new0(color_space: &Any, channels: &Sequence<Any>) -> CSSColor {
        Self {
            inner: Any::global("CSSColor")
                .new(&[color_space.into(), channels.into()])
                .as_::<CSSColorValue>(),
        }
    }

    /// The `new CSSColor(..)` constructor, creating a new CSSColor instance
    pub fn new1(color_space: &Any, channels: &Sequence<Any>, alpha: &Any) -> CSSColor {
        Self {
            inner: Any::global("CSSColor")
                .new(&[color_space.into(), channels.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSColor {
    /// Getter of the `colorSpace` attribute.
    /// [`CSSColor.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/colorSpace)
    pub fn color_space(&self) -> Any {
        self.inner.get("colorSpace").as_::<Any>()
    }

    /// Setter of the `colorSpace` attribute.
    /// [`CSSColor.colorSpace`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/colorSpace)
    pub fn set_color_space(&mut self, value: &Any) {
        self.inner.set("colorSpace", value);
    }
}
impl CSSColor {
    /// Getter of the `channels` attribute.
    /// [`CSSColor.channels`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/channels)
    pub fn channels(&self) -> ObservableArray<Any> {
        self.inner.get("channels").as_::<ObservableArray<Any>>()
    }

    /// Setter of the `channels` attribute.
    /// [`CSSColor.channels`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/channels)
    pub fn set_channels(&mut self, value: &ObservableArray<Any>) {
        self.inner.set("channels", value);
    }
}
impl CSSColor {
    /// Getter of the `alpha` attribute.
    /// [`CSSColor.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/alpha)
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    /// Setter of the `alpha` attribute.
    /// [`CSSColor.alpha`](https://developer.mozilla.org/en-US/docs/Web/API/CSSColor/alpha)
    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}
