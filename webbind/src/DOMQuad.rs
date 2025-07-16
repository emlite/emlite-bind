use super::*;

/// The DOMQuad class.
/// [`DOMQuad`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMQuad {
    inner: Any,
}
impl FromVal for DOMQuad {
    fn from_val(v: &Any) -> Self {
        DOMQuad {
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
impl core::ops::Deref for DOMQuad {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DOMQuad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DOMQuad {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DOMQuad {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DOMQuad> for Any {
    fn from(s: DOMQuad) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DOMQuad> for Any {
    fn from(s: &DOMQuad) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DOMQuad);

impl DOMQuad {
    /// The `new DOMQuad(..)` constructor, creating a new DOMQuad instance
    pub fn new0() -> DOMQuad {
        Self {
            inner: Any::global("DOMQuad").new(&[]).as_::<Any>(),
        }
    }

    /// The `new DOMQuad(..)` constructor, creating a new DOMQuad instance
    pub fn new1(p1: &DOMPointInit) -> DOMQuad {
        Self {
            inner: Any::global("DOMQuad").new(&[p1.into()]).as_::<Any>(),
        }
    }

    /// The `new DOMQuad(..)` constructor, creating a new DOMQuad instance
    pub fn new2(p1: &DOMPointInit, p2: &DOMPointInit) -> DOMQuad {
        Self {
            inner: Any::global("DOMQuad")
                .new(&[p1.into(), p2.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMQuad(..)` constructor, creating a new DOMQuad instance
    pub fn new3(p1: &DOMPointInit, p2: &DOMPointInit, p3: &DOMPointInit) -> DOMQuad {
        Self {
            inner: Any::global("DOMQuad")
                .new(&[p1.into(), p2.into(), p3.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMQuad(..)` constructor, creating a new DOMQuad instance
    pub fn new4(
        p1: &DOMPointInit,
        p2: &DOMPointInit,
        p3: &DOMPointInit,
        p4: &DOMPointInit,
    ) -> DOMQuad {
        Self {
            inner: Any::global("DOMQuad")
                .new(&[p1.into(), p2.into(), p3.into(), p4.into()])
                .as_::<Any>(),
        }
    }
}
impl DOMQuad {
    /// The fromRect method.
    /// [`DOMQuad.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/fromRect)
    pub fn from_rect0() -> DOMQuad {
        Any::global("DOMQuad")
            .call("fromRect", &[])
            .as_::<DOMQuad>()
    }
    /// The fromRect method.
    /// [`DOMQuad.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/fromRect)
    pub fn from_rect1(other: &DOMRectInit) -> DOMQuad {
        Any::global("DOMQuad")
            .call("fromRect", &[other.into()])
            .as_::<DOMQuad>()
    }
}
impl DOMQuad {
    /// The fromQuad method.
    /// [`DOMQuad.fromQuad`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/fromQuad)
    pub fn from_quad0() -> DOMQuad {
        Any::global("DOMQuad")
            .call("fromQuad", &[])
            .as_::<DOMQuad>()
    }
    /// The fromQuad method.
    /// [`DOMQuad.fromQuad`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/fromQuad)
    pub fn from_quad1(other: &DOMQuadInit) -> DOMQuad {
        Any::global("DOMQuad")
            .call("fromQuad", &[other.into()])
            .as_::<DOMQuad>()
    }
}
impl DOMQuad {
    /// Getter of the `p1` attribute.
    /// [`DOMQuad.p1`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p1)
    pub fn p1(&self) -> DOMPoint {
        self.inner.get("p1").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    /// Getter of the `p2` attribute.
    /// [`DOMQuad.p2`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p2)
    pub fn p2(&self) -> DOMPoint {
        self.inner.get("p2").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    /// Getter of the `p3` attribute.
    /// [`DOMQuad.p3`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p3)
    pub fn p3(&self) -> DOMPoint {
        self.inner.get("p3").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    /// Getter of the `p4` attribute.
    /// [`DOMQuad.p4`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/p4)
    pub fn p4(&self) -> DOMPoint {
        self.inner.get("p4").as_::<DOMPoint>()
    }
}
impl DOMQuad {
    /// The getBounds method.
    /// [`DOMQuad.getBounds`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/getBounds)
    pub fn get_bounds(&self) -> DOMRect {
        self.inner.call("getBounds", &[]).as_::<DOMRect>()
    }
}
impl DOMQuad {
    /// The toJSON method.
    /// [`DOMQuad.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/DOMQuad/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
