use super::*;

/// The DOMRect class.
/// [`DOMRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DOMRect {
    inner: DOMRectReadOnly,
}

impl FromVal for DOMRect {
    fn from_val(v: &Any) -> Self {
        DOMRect {
            inner: DOMRectReadOnly::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
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

impl AsRef<Any> for DOMRect {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for DOMRect {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<DOMRect> for Any {
    fn from(s: DOMRect) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&DOMRect> for Any {
    fn from(s: &DOMRect) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(DOMRect);

impl DOMRect {
    /// Getter of the `x` attribute.
    /// [`DOMRect.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)
    pub fn x(&self) -> f64 {
        self.inner.get("x").as_::<f64>()
    }

    /// Setter of the `x` attribute.
    /// [`DOMRect.x`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/x)
    pub fn set_x(&mut self, value: f64) {
        self.inner.set("x", value);
    }
}
impl DOMRect {
    /// Getter of the `y` attribute.
    /// [`DOMRect.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)
    pub fn y(&self) -> f64 {
        self.inner.get("y").as_::<f64>()
    }

    /// Setter of the `y` attribute.
    /// [`DOMRect.y`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/y)
    pub fn set_y(&mut self, value: f64) {
        self.inner.set("y", value);
    }
}
impl DOMRect {
    /// Getter of the `width` attribute.
    /// [`DOMRect.width`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)
    pub fn width(&self) -> f64 {
        self.inner.get("width").as_::<f64>()
    }

    /// Setter of the `width` attribute.
    /// [`DOMRect.width`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/width)
    pub fn set_width(&mut self, value: f64) {
        self.inner.set("width", value);
    }
}
impl DOMRect {
    /// Getter of the `height` attribute.
    /// [`DOMRect.height`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)
    pub fn height(&self) -> f64 {
        self.inner.get("height").as_::<f64>()
    }

    /// Setter of the `height` attribute.
    /// [`DOMRect.height`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/height)
    pub fn set_height(&mut self, value: f64) {
        self.inner.set("height", value);
    }
}

impl DOMRect {
    /// The `new DOMRect(..)` constructor, creating a new DOMRect instance
    pub fn new() -> DOMRect {
        Self {
            inner: Any::global("DOMRect").new(&[]).as_::<DOMRectReadOnly>(),
        }
    }
}

impl DOMRect {
    /// The `new DOMRect(..)` constructor, creating a new DOMRect instance
    pub fn new_with_x(x: f64) -> DOMRect {
        Self {
            inner: Any::global("DOMRect")
                .new(&[x.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }
}

impl DOMRect {
    /// The `new DOMRect(..)` constructor, creating a new DOMRect instance
    pub fn new_with_x_and_y(x: f64, y: f64) -> DOMRect {
        Self {
            inner: Any::global("DOMRect")
                .new(&[x.into(), y.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }
}

impl DOMRect {
    /// The `new DOMRect(..)` constructor, creating a new DOMRect instance
    pub fn new_with_x_and_y_and_width(x: f64, y: f64, width: f64) -> DOMRect {
        Self {
            inner: Any::global("DOMRect")
                .new(&[x.into(), y.into(), width.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }
}

impl DOMRect {
    /// The `new DOMRect(..)` constructor, creating a new DOMRect instance
    pub fn new_with_x_and_y_and_width_and_height(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    ) -> DOMRect {
        Self {
            inner: Any::global("DOMRect")
                .new(&[x.into(), y.into(), width.into(), height.into()])
                .as_::<DOMRectReadOnly>(),
        }
    }
}

impl DOMRect {
    /// The fromRect method.
    /// [`DOMRect.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/fromRect)
    pub fn from_rect() -> DOMRect {
        Any::global("DOMRect")
            .call("fromRect", &[])
            .as_::<DOMRect>()
    }
}
impl DOMRect {
    /// The fromRect method.
    /// [`DOMRect.fromRect`](https://developer.mozilla.org/en-US/docs/Web/API/DOMRect/fromRect)
    pub fn from_rect_with_other(other: &DOMRectInit) -> DOMRect {
        Any::global("DOMRect")
            .call("fromRect", &[other.into()])
            .as_::<DOMRect>()
    }
}
