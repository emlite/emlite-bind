use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
impl From<CSSColor> for emlite::Val {
    fn from(s: CSSColor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CSSColor {
    pub fn new0(color_space: jsbind::Any, channels: jsbind::Sequence<jsbind::Any>) -> CSSColor {
        Self {
            inner: emlite::Val::global("CSSColor")
                .new(&[color_space.into(), channels.into()])
                .as_::<CSSColorValue>(),
        }
    }

    pub fn new1(
        color_space: jsbind::Any,
        channels: jsbind::Sequence<jsbind::Any>,
        alpha: jsbind::Any,
    ) -> CSSColor {
        Self {
            inner: emlite::Val::global("CSSColor")
                .new(&[color_space.into(), channels.into(), alpha.into()])
                .as_::<CSSColorValue>(),
        }
    }
}
impl CSSColor {
    pub fn color_space(&self) -> jsbind::Any {
        self.inner.get("colorSpace").as_::<jsbind::Any>()
    }

    pub fn set_color_space(&mut self, value: jsbind::Any) {
        self.inner.set("colorSpace", value);
    }
}
impl CSSColor {
    pub fn channels(&self) -> jsbind::ObservableArray<jsbind::Any> {
        self.inner
            .get("channels")
            .as_::<jsbind::ObservableArray<jsbind::Any>>()
    }

    pub fn set_channels(&mut self, value: jsbind::ObservableArray<jsbind::Any>) {
        self.inner.set("channels", value);
    }
}
impl CSSColor {
    pub fn alpha(&self) -> jsbind::Any {
        self.inner.get("alpha").as_::<jsbind::Any>()
    }

    pub fn set_alpha(&mut self, value: jsbind::Any) {
        self.inner.set("alpha", value);
    }
}
