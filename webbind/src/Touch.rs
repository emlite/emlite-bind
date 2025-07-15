use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Touch {
    inner: emlite::Val,
}
impl FromVal for Touch {
    fn from_val(v: &emlite::Val) -> Self {
        Touch {
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
impl core::ops::Deref for Touch {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Touch {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Touch {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Touch {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Touch> for emlite::Val {
    fn from(s: Touch) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&Touch> for emlite::Val {
    fn from(s: &Touch) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(Touch);

impl Touch {
    pub fn new(touch_init_dict: &Any) -> Touch {
        Self {
            inner: emlite::Val::global("Touch")
                .new(&[touch_init_dict.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl Touch {
    pub fn identifier(&self) -> i32 {
        self.inner.get("identifier").as_::<i32>()
    }
}
impl Touch {
    pub fn target(&self) -> EventTarget {
        self.inner.get("target").as_::<EventTarget>()
    }
}
impl Touch {
    pub fn screen_x(&self) -> f64 {
        self.inner.get("screenX").as_::<f64>()
    }
}
impl Touch {
    pub fn screen_y(&self) -> f64 {
        self.inner.get("screenY").as_::<f64>()
    }
}
impl Touch {
    pub fn client_x(&self) -> f64 {
        self.inner.get("clientX").as_::<f64>()
    }
}
impl Touch {
    pub fn client_y(&self) -> f64 {
        self.inner.get("clientY").as_::<f64>()
    }
}
impl Touch {
    pub fn page_x(&self) -> f64 {
        self.inner.get("pageX").as_::<f64>()
    }
}
impl Touch {
    pub fn page_y(&self) -> f64 {
        self.inner.get("pageY").as_::<f64>()
    }
}
impl Touch {
    pub fn radius_x(&self) -> f32 {
        self.inner.get("radiusX").as_::<f32>()
    }
}
impl Touch {
    pub fn radius_y(&self) -> f32 {
        self.inner.get("radiusY").as_::<f32>()
    }
}
impl Touch {
    pub fn rotation_angle(&self) -> f32 {
        self.inner.get("rotationAngle").as_::<f32>()
    }
}
impl Touch {
    pub fn force(&self) -> f32 {
        self.inner.get("force").as_::<f32>()
    }
}
impl Touch {
    pub fn altitude_angle(&self) -> f32 {
        self.inner.get("altitudeAngle").as_::<f32>()
    }
}
impl Touch {
    pub fn azimuth_angle(&self) -> f32 {
        self.inner.get("azimuthAngle").as_::<f32>()
    }
}
impl Touch {
    pub fn touch_type(&self) -> TouchType {
        self.inner.get("touchType").as_::<TouchType>()
    }
}
