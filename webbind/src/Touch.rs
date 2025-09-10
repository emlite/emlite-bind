use super::*;

/// The Touch class.
/// [`Touch`](https://developer.mozilla.org/en-US/docs/Web/API/Touch)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Touch {
    inner: Any,
}

impl FromVal for Touch {
    fn from_val(v: &Any) -> Self {
        Touch {
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

impl core::ops::Deref for Touch {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Touch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Touch {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Touch {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Touch> for Any {
    fn from(s: Touch) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Touch> for Any {
    fn from(s: &Touch) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Touch);

impl Touch {
    /// The `new Touch(..)` constructor, creating a new Touch instance
    pub fn new(touch_init_dict: &TouchInit) -> Touch {
        Self {
            inner: Any::global("Touch")
                .new(&[touch_init_dict.into()])
                .as_::<Any>(),
        }
    }
}
impl Touch {
    /// Getter of the `identifier` attribute.
    /// [`Touch.identifier`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/identifier)
    pub fn identifier(&self) -> i32 {
        self.inner.get("identifier").as_::<i32>()
    }
}
impl Touch {
    /// Getter of the `target` attribute.
    /// [`Touch.target`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/target)
    pub fn target(&self) -> EventTarget {
        self.inner.get("target").as_::<EventTarget>()
    }
}
impl Touch {
    /// Getter of the `screenX` attribute.
    /// [`Touch.screenX`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenX)
    pub fn screen_x(&self) -> f64 {
        self.inner.get("screenX").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `screenY` attribute.
    /// [`Touch.screenY`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/screenY)
    pub fn screen_y(&self) -> f64 {
        self.inner.get("screenY").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `clientX` attribute.
    /// [`Touch.clientX`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientX)
    pub fn client_x(&self) -> f64 {
        self.inner.get("clientX").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `clientY` attribute.
    /// [`Touch.clientY`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/clientY)
    pub fn client_y(&self) -> f64 {
        self.inner.get("clientY").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `pageX` attribute.
    /// [`Touch.pageX`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageX)
    pub fn page_x(&self) -> f64 {
        self.inner.get("pageX").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `pageY` attribute.
    /// [`Touch.pageY`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/pageY)
    pub fn page_y(&self) -> f64 {
        self.inner.get("pageY").as_::<f64>()
    }
}
impl Touch {
    /// Getter of the `radiusX` attribute.
    /// [`Touch.radiusX`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusX)
    pub fn radius_x(&self) -> f32 {
        self.inner.get("radiusX").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `radiusY` attribute.
    /// [`Touch.radiusY`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/radiusY)
    pub fn radius_y(&self) -> f32 {
        self.inner.get("radiusY").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `rotationAngle` attribute.
    /// [`Touch.rotationAngle`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/rotationAngle)
    pub fn rotation_angle(&self) -> f32 {
        self.inner.get("rotationAngle").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `force` attribute.
    /// [`Touch.force`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/force)
    pub fn force(&self) -> f32 {
        self.inner.get("force").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `altitudeAngle` attribute.
    /// [`Touch.altitudeAngle`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/altitudeAngle)
    pub fn altitude_angle(&self) -> f32 {
        self.inner.get("altitudeAngle").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `azimuthAngle` attribute.
    /// [`Touch.azimuthAngle`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/azimuthAngle)
    pub fn azimuth_angle(&self) -> f32 {
        self.inner.get("azimuthAngle").as_::<f32>()
    }
}
impl Touch {
    /// Getter of the `touchType` attribute.
    /// [`Touch.touchType`](https://developer.mozilla.org/en-US/docs/Web/API/Touch/touchType)
    pub fn touch_type(&self) -> TouchType {
        self.inner.get("touchType").as_::<TouchType>()
    }
}
