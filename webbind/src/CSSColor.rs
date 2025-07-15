use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CSSColor {
    inner: CSSColorValue,
}
impl FromVal for CSSColor {
    fn from_val(v: &emlite::Val) -> Self {
        CSSColor {
            inner: CSSColorValue::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for CSSColor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for CSSColor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<CSSColor> for emlite::Val {
    fn from(s: CSSColor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&CSSColor> for emlite::Val {
    fn from(s: &CSSColor) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(CSSColor);

impl CSSColor {
    pub fn new0(color_space: &Any, channels: &Sequence<Any>) -> CSSColor {
        Self {
            inner: emlite::Val::global("CSSColor")
                .new(&[color_space.into(), channels.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(color_space: &Any, channels: &Sequence<Any>, alpha: &Any) -> CSSColor {
        Self {
            inner: emlite::Val::global("CSSColor")
                .new(&[color_space.into(), channels.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSColor {
    pub fn color_space(&self) -> Any {
        self.inner.get("colorSpace").as_::<Any>()
    }

    pub fn set_color_space(&mut self, value: &Any) {
        self.inner.set("colorSpace", value);
    }
}
impl CSSColor {
    pub fn channels(&self) -> ObservableArray<Any> {
        self.inner.get("channels").as_::<ObservableArray<Any>>()
    }

    pub fn set_channels(&mut self, value: &ObservableArray<Any>) {
        self.inner.set("channels", value);
    }
}
impl CSSColor {
    pub fn alpha(&self) -> Any {
        self.inner.get("alpha").as_::<Any>()
    }

    pub fn set_alpha(&mut self, value: &Any) {
        self.inner.set("alpha", value);
    }
}
