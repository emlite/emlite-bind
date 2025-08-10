use super::*;

/// The MLPool2dOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLPool2dOptions {
    inner: Any,
}

impl FromVal for MLPool2dOptions {
    fn from_val(v: &Any) -> Self {
        MLPool2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLPool2dOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLPool2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLPool2dOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLPool2dOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<MLPool2dOptions> for Any {
    fn from(s: MLPool2dOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLPool2dOptions> for Any {
    fn from(s: &MLPool2dOptions) -> Any {
        s.inner.clone()
    }
}

impl MLPool2dOptions {
    /// Getter of the `windowDimensions` attribute.
    pub fn window_dimensions(&self) -> TypedArray<u32> {
        self.inner.get("windowDimensions").as_::<TypedArray<u32>>()
    }

    /// Setter of the `windowDimensions` attribute.
    pub fn set_window_dimensions(&mut self, value: TypedArray<u32>) {
        self.inner.set("windowDimensions", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `padding` attribute.
    pub fn padding(&self) -> TypedArray<u32> {
        self.inner.get("padding").as_::<TypedArray<u32>>()
    }

    /// Setter of the `padding` attribute.
    pub fn set_padding(&mut self, value: TypedArray<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `strides` attribute.
    pub fn strides(&self) -> TypedArray<u32> {
        self.inner.get("strides").as_::<TypedArray<u32>>()
    }

    /// Setter of the `strides` attribute.
    pub fn set_strides(&mut self, value: TypedArray<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `dilations` attribute.
    pub fn dilations(&self) -> TypedArray<u32> {
        self.inner.get("dilations").as_::<TypedArray<u32>>()
    }

    /// Setter of the `dilations` attribute.
    pub fn set_dilations(&mut self, value: TypedArray<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `layout` attribute.
    pub fn layout(&self) -> MLInputOperandLayout {
        self.inner.get("layout").as_::<MLInputOperandLayout>()
    }

    /// Setter of the `layout` attribute.
    pub fn set_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `roundingType` attribute.
    pub fn rounding_type(&self) -> MLRoundingType {
        self.inner.get("roundingType").as_::<MLRoundingType>()
    }

    /// Setter of the `roundingType` attribute.
    pub fn set_rounding_type(&mut self, value: &MLRoundingType) {
        self.inner.set("roundingType", value);
    }
}
impl MLPool2dOptions {
    /// Getter of the `outputSizes` attribute.
    pub fn output_sizes(&self) -> TypedArray<u32> {
        self.inner.get("outputSizes").as_::<TypedArray<u32>>()
    }

    /// Setter of the `outputSizes` attribute.
    pub fn set_output_sizes(&mut self, value: TypedArray<u32>) {
        self.inner.set("outputSizes", value);
    }
}
