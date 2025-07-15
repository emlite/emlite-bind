use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMQuad {
    inner: emlite::Val,
}
impl FromVal for DOMQuad {
    fn from_val(v: &emlite::Val) -> Self {
        DOMQuad {
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
impl core::ops::Deref for DOMQuad {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMQuad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DOMQuad {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DOMQuad {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DOMQuad> for emlite::Val {
    fn from(s: DOMQuad) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DOMQuad);

impl DOMQuad {
    pub fn new0() -> DOMQuad {
        Self {
            inner: emlite::Val::global("DOMQuad").new(&[]).as_::<emlite::Val>(),
        }
    }

    pub fn new1(p1: DOMPointInit) -> DOMQuad {
        Self {
            inner: emlite::Val::global("DOMQuad")
                .new(&[p1.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new2(p1: DOMPointInit, p2: DOMPointInit) -> DOMQuad {
        Self {
            inner: emlite::Val::global("DOMQuad")
                .new(&[p1.into(), p2.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new3(p1: DOMPointInit, p2: DOMPointInit, p3: DOMPointInit) -> DOMQuad {
        Self {
            inner: emlite::Val::global("DOMQuad")
                .new(&[p1.into(), p2.into(), p3.into()])
                .as_::<emlite::Val>(),
        }
    }

    pub fn new4(p1: DOMPointInit, p2: DOMPointInit, p3: DOMPointInit, p4: DOMPointInit) -> DOMQuad {
        Self {
            inner: emlite::Val::global("DOMQuad")
                .new(&[p1.into(), p2.into(), p3.into(), p4.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl DOMQuad {
    pub fn from_rect0() -> DOMQuad {
        emlite::Val::global("DOMQuad")
            .call("fromRect", &[])
            .as_::<DOMQuad>()
    }

    pub fn from_rect1(other: DOMRectInit) -> DOMQuad {
        emlite::Val::global("DOMQuad")
            .call("fromRect", &[other.into()])
            .as_::<DOMQuad>()
    }
}
impl DOMQuad {
    pub fn from_quad0() -> DOMQuad {
        emlite::Val::global("DOMQuad")
            .call("fromQuad", &[])
            .as_::<DOMQuad>()
    }

    pub fn from_quad1(other: DOMQuadInit) -> DOMQuad {
        emlite::Val::global("DOMQuad")
            .call("fromQuad", &[other.into()])
            .as_::<DOMQuad>()
    }
}
impl DOMQuad {
    pub fn p1(&self) -> DOMPoint {
        self.inner.get("p1").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    pub fn p2(&self) -> DOMPoint {
        self.inner.get("p2").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    pub fn p3(&self) -> DOMPoint {
        self.inner.get("p3").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    pub fn p4(&self) -> DOMPoint {
        self.inner.get("p4").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    pub fn get_bounds(&self) -> DOMRect {
        self.inner.call("getBounds", &[]).as_::<DOMRect>()
    }
}
impl DOMQuad {
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
