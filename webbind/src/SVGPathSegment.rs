use super::*;

/// The SVGPathSegment class.
/// [`SVGPathSegment`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegment)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPathSegment {
    inner: Any,
}
impl FromVal for SVGPathSegment {
    fn from_val(v: &Any) -> Self {
        SVGPathSegment {
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
impl core::ops::Deref for SVGPathSegment {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGPathSegment {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGPathSegment {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGPathSegment {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGPathSegment> for Any {
    fn from(s: SVGPathSegment) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGPathSegment> for Any {
    fn from(s: &SVGPathSegment) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGPathSegment);

impl SVGPathSegment {
    /// Getter of the `type` attribute.
    /// [`SVGPathSegment.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegment/type)
    pub fn type_(&self) -> DOMString {
        self.inner.get("type").as_::<DOMString>()
    }

    /// Setter of the `type` attribute.
    /// [`SVGPathSegment.type`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegment/type)
    pub fn set_type_(&mut self, value: &DOMString) {
        self.inner.set("type", value);
    }
}
impl SVGPathSegment {
    /// Getter of the `values` attribute.
    /// [`SVGPathSegment.values`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegment/values)
    pub fn values(&self) -> FrozenArray<f32> {
        self.inner.get("values").as_::<FrozenArray<f32>>()
    }

    /// Setter of the `values` attribute.
    /// [`SVGPathSegment.values`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPathSegment/values)
    pub fn set_values(&mut self, value: FrozenArray<f32>) {
        self.inner.set("values", value);
    }
}
