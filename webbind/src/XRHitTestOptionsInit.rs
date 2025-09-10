use super::*;

/// The XRHitTestOptionsInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRHitTestOptionsInit {
    inner: Any,
}

impl FromVal for XRHitTestOptionsInit {
    fn from_val(v: &Any) -> Self {
        XRHitTestOptionsInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRHitTestOptionsInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRHitTestOptionsInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRHitTestOptionsInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRHitTestOptionsInit {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<XRHitTestOptionsInit> for Any {
    fn from(s: XRHitTestOptionsInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRHitTestOptionsInit> for Any {
    fn from(s: &XRHitTestOptionsInit) -> Any {
        s.inner.clone()
    }
}

impl XRHitTestOptionsInit {
    /// Getter of the `space` attribute.
    pub fn space(&self) -> XRSpace {
        self.inner.get("space").as_::<XRSpace>()
    }

    /// Setter of the `space` attribute.
    pub fn set_space(&mut self, value: &XRSpace) {
        self.inner.set("space", value);
    }
}
impl XRHitTestOptionsInit {
    /// Getter of the `entityTypes` attribute.
    pub fn entity_types(&self) -> TypedArray<XRHitTestTrackableType> {
        self.inner
            .get("entityTypes")
            .as_::<TypedArray<XRHitTestTrackableType>>()
    }

    /// Setter of the `entityTypes` attribute.
    pub fn set_entity_types(&mut self, value: &TypedArray<XRHitTestTrackableType>) {
        self.inner.set("entityTypes", value);
    }
}
impl XRHitTestOptionsInit {
    /// Getter of the `offsetRay` attribute.
    pub fn offset_ray(&self) -> XRRay {
        self.inner.get("offsetRay").as_::<XRRay>()
    }

    /// Setter of the `offsetRay` attribute.
    pub fn set_offset_ray(&mut self, value: &XRRay) {
        self.inner.set("offsetRay", value);
    }
}
