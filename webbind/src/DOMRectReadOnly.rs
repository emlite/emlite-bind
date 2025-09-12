use super::*;

/// The DOMRectReadOnly class.
/// [`DOMRectReadOnly`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRectReadOnly {
    inner: Any,
}

impl FromVal for DOMRectReadOnly {
    fn from_val(v: &Any) -> Self {
        DOMRectReadOnly {
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

impl core::ops::Deref for DOMRectReadOnly {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for DOMRectReadOnly {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for DOMRectReadOnly {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMRectReadOnly {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMRectReadOnly> for Any {
    fn from(s: DOMRectReadOnly) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMRectReadOnly> for Any {
    fn from(s: &DOMRectReadOnly) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMRectReadOnly);

impl DOMRectReadOnly {
    /// Getter of the `x` attribute.
    /// [`DOMRectReadOnly.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `y` attribute.
    /// [`DOMRectReadOnly.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `width` attribute.
    /// [`DOMRectReadOnly.width`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `height` attribute.
    /// [`DOMRectReadOnly.height`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `top` attribute.
    /// [`DOMRectReadOnly.top`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/top)
    pub fn top(&self) -> f64 {
        self.inner.get("top").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `right` attribute.
    /// [`DOMRectReadOnly.right`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/right)
    pub fn right(&self) -> f64 {
        self.inner.get("right").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `bottom` attribute.
    /// [`DOMRectReadOnly.bottom`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/bottom)
    pub fn bottom(&self) -> f64 {
        self.inner.get("bottom").as_::<f64>()
    }
}
impl DOMRectReadOnly {
    /// Getter of the `left` attribute.
    /// [`DOMRectReadOnly.left`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/left)
    pub fn left(&self) -> f64 {
        self.inner.get("left").as_::<f64>()
    }
}

impl DOMRectReadOnly {
    /// The `new DOMRectReadOnly(..)` constructor, creating a new DOMRectReadOnly instance
    pub fn new0() -> DOMRectReadOnly {
        Self {
            inner: Any::global("DOMRectReadOnly").new(&[]).as_::<Any>(),
        }
    }

    /// The `new DOMRectReadOnly(..)` constructor, creating a new DOMRectReadOnly instance
    pub fn new1(x: f64) -> DOMRectReadOnly {
        Self {
            inner: Any::global("DOMRectReadOnly").new(&[x.into()]).as_::<Any>(),
        }
    }

    /// The `new DOMRectReadOnly(..)` constructor, creating a new DOMRectReadOnly instance
    pub fn new2(x: f64, y: f64) -> DOMRectReadOnly {
        Self {
            inner: Any::global("DOMRectReadOnly")
                .new(&[x.into(), y.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMRectReadOnly(..)` constructor, creating a new DOMRectReadOnly instance
    pub fn new3(x: f64, y: f64, width: f64) -> DOMRectReadOnly {
        Self {
            inner: Any::global("DOMRectReadOnly")
                .new(&[x.into(), y.into(), width.into()])
                .as_::<Any>(),
        }
    }

    /// The `new DOMRectReadOnly(..)` constructor, creating a new DOMRectReadOnly instance
    pub fn new4(x: f64, y: f64, width: f64, height: f64) -> DOMRectReadOnly {
        Self {
            inner: Any::global("DOMRectReadOnly")
                .new(&[x.into(), y.into(), width.into(), height.into()])
                .as_::<Any>(),
        }
    }
}
impl DOMRectReadOnly {
    /// The fromRect method.
    /// [`DOMRectReadOnly.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/fromRect)
    pub fn from_rect0() -> DOMRectReadOnly {
        Any::global("DOMRectReadOnly")
            .call("fromRect", &[])
            .as_::<DOMRectReadOnly>()
    }
    /// The fromRect method.
    /// [`DOMRectReadOnly.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/fromRect)
    pub fn from_rect1(other: &DOMRectInit) -> DOMRectReadOnly {
        Any::global("DOMRectReadOnly")
            .call("fromRect", &[other.into()])
            .as_::<DOMRectReadOnly>()
    }
}
impl DOMRectReadOnly {
    /// The toJSON method.
    /// [`DOMRectReadOnly.toJSON`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRectReadOnly/toJSON)
    pub fn to_json(&self) -> Object {
        self.inner.call("toJSON", &[]).as_::<Object>()
    }
}
