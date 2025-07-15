use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRectInit {
    inner: emlite::Val,
}
impl FromVal for DOMRectInit {
    fn from_val(v: &emlite::Val) -> Self {
        DOMRectInit { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DOMRectInit {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMRectInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMRectInit {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMRectInit {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMRectInit> for emlite::Val {
    fn from(s: DOMRectInit) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DOMRectInit> for emlite::Val {
    fn from(s: &DOMRectInit) -> emlite::Val {
        s.inner.clone()
    }
}

impl DOMRectInit {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMRectInit {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMRectInit {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl DOMRectInit {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

    pub fn set_height(&mut self, value: f64) {
        self.inner.set("height", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRectReadOnly {
    inner: emlite::Val,
}
impl FromVal for DOMRectReadOnly {
    fn from_val(v: &emlite::Val) -> Self {
        DOMRectReadOnly {
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
impl core::ops::Deref for DOMRectReadOnly {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMRectReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMRectReadOnly {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMRectReadOnly {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMRectReadOnly> for emlite::Val {
    fn from(s: DOMRectReadOnly) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DOMRectReadOnly> for emlite::Val {
    fn from(s: &DOMRectReadOnly) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMRectReadOnly);

impl DOMRectReadOnly {
    pub fn new0() -> DOMRectReadOnly {
        Self {
            inner: emlite::Val::global("DOMRectReadOnly")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new1(x: f64) -> DOMRectReadOnly {
        Self {
            inner: emlite::Val::global("DOMRectReadOnly")
                .new(&[x.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(x: f64, y: f64) -> DOMRectReadOnly {
        Self {
            inner: emlite::Val::global("DOMRectReadOnly")
                .new(&[x.into(), y.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new3(x: f64, y: f64, width: f64) -> DOMRectReadOnly {
        Self {
            inner: emlite::Val::global("DOMRectReadOnly")
                .new(&[x.into(), y.into(), width.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new4(x: f64, y: f64, width: f64, height: f64) -> DOMRectReadOnly {
        Self {
            inner: emlite::Val::global("DOMRectReadOnly")
                .new(&[x.into(), y.into(), width.into(), height.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl DOMRectReadOnly {
    pub fn from_rect0() -> DOMRectReadOnly {
        emlite::Val::global("DOMRectReadOnly")
            .call("fromRect", &[])
            .as_::<DOMRectReadOnly>()
    }

    pub fn from_rect1(other: DOMRectInit) -> DOMRectReadOnly {
        emlite::Val::global("DOMRectReadOnly")
            .call("fromRect", &[other.into()])
            .as_::<DOMRectReadOnly>()
    }
}
impl DOMRectReadOnly {
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn top(&self) -> f64 {
        self.inner.get("top").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn right(&self) -> f64 {
        self.inner.get("right").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn bottom(&self) -> f64 {
        self.inner.get("bottom").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn left(&self) -> f64 {
        self.inner.get("left").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
