use super::*;




/// The XRDepthStateInit dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct XRDepthStateInit {
    inner: Any,
}

impl FromVal for XRDepthStateInit {
    fn from_val(v: &Any) -> Self {
        XRDepthStateInit { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for XRDepthStateInit {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for XRDepthStateInit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for XRDepthStateInit {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for XRDepthStateInit {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<XRDepthStateInit> for Any {
    fn from(s: XRDepthStateInit) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&XRDepthStateInit> for Any {
    fn from(s: &XRDepthStateInit) -> Any {
        s.inner.clone()
    }
}

impl XRDepthStateInit {
    /// Getter of the `usagePreference` attribute.
    pub fn usage_preference(&self) -> TypedArray<XRDepthUsage> {
        self.inner.get("usagePreference").as_::<TypedArray<XRDepthUsage>>()
    }

    /// Setter of the `usagePreference` attribute.
    pub fn set_usage_preference(&mut self, value: &TypedArray<XRDepthUsage>) {
        self.inner.set("usagePreference", value);
    }
}
impl XRDepthStateInit {
    /// Getter of the `dataFormatPreference` attribute.
    pub fn data_format_preference(&self) -> TypedArray<XRDepthDataFormat> {
        self.inner.get("dataFormatPreference").as_::<TypedArray<XRDepthDataFormat>>()
    }

    /// Setter of the `dataFormatPreference` attribute.
    pub fn set_data_format_preference(&mut self, value: &TypedArray<XRDepthDataFormat>) {
        self.inner.set("dataFormatPreference", value);
    }
}
impl XRDepthStateInit {
    /// Getter of the `depthTypeRequest` attribute.
    pub fn depth_type_request(&self) -> TypedArray<XRDepthType> {
        self.inner.get("depthTypeRequest").as_::<TypedArray<XRDepthType>>()
    }

    /// Setter of the `depthTypeRequest` attribute.
    pub fn set_depth_type_request(&mut self, value: &TypedArray<XRDepthType>) {
        self.inner.set("depthTypeRequest", value);
    }
}
impl XRDepthStateInit {
    /// Getter of the `matchDepthView` attribute.
    pub fn match_depth_view(&self) -> bool {
        self.inner.get("matchDepthView").as_::<bool>()
    }

    /// Setter of the `matchDepthView` attribute.
    pub fn set_match_depth_view(&mut self, value: bool) {
        self.inner.set("matchDepthView", value);
    }
}
