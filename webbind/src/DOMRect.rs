use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRect {
    inner: DOMRectReadOnly,
}
impl FromVal for DOMRect {
    fn from_val(v: &emlite::Val) -> Self {
        DOMRect {
            inner: DOMRectReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMRect {
    type Target = DOMRectReadOnly;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMRect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMRect {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMRect {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMRect> for emlite::Val {
    fn from(s: DOMRect) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMRect);

impl DOMRect {
    pub fn new0() -> DOMRect {
        Self {
            inner: emlite::Val::global("DOMRect")
                .new(&[])
                .as_::<DOMRectReadOnly>(),
        }
    }

    pub fn new1(x: f64) -> DOMRect {
        Self {
            inner: emlite::Val::global("DOMRect")
                .new(&[x.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }

    pub fn new2(x: f64, y: f64) -> DOMRect {
        Self {
            inner: emlite::Val::global("DOMRect")
                .new(&[x.into(), y.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }

    pub fn new3(x: f64, y: f64, width: f64) -> DOMRect {
        Self {
            inner: emlite::Val::global("DOMRect")
                .new(&[x.into(), y.into(), width.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }

    pub fn new4(x: f64, y: f64, width: f64, height: f64) -> DOMRect {
        Self {
            inner: emlite::Val::global("DOMRect")
                .new(&[x.into(), y.into(), width.into(), height.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }
}
impl DOMRect {
    pub fn from_rect0() -> DOMRect {
        emlite::Val::global("domrect")
            .call("fromRect", &[])
            .as_::<DOMRect>()
    }

    pub fn from_rect1(other: DOMRectInit) -> DOMRect {
        emlite::Val::global("domrect")
            .call("fromRect", &[other.into()])
            .as_::<DOMRect>()
    }
}
impl DOMRect {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMRect {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMRect {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl DOMRect {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

    pub fn set_height(&mut self, value: f64) {
        self.inner.set("height", value);
    }
}
