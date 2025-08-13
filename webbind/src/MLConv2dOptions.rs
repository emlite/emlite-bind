use super::*;




/// The MLConv2dOptions dictionary.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLConv2dOptions {
    inner: Any,
}

impl FromVal for MLConv2dOptions {
    fn from_val(v: &Any) -> Self {
        MLConv2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLConv2dOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLConv2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLConv2dOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLConv2dOptions {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLConv2dOptions> for Any {
    fn from(s: MLConv2dOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLConv2dOptions> for Any {
    fn from(s: &MLConv2dOptions) -> Any {
        s.inner.clone()
    }
}

impl MLConv2dOptions {
    /// Getter of the `padding` attribute.
    pub fn padding(&self) -> TypedArray<u32> {
        self.inner.get("padding").as_::<TypedArray<u32>>()
    }

    /// Setter of the `padding` attribute.
    pub fn set_padding(&mut self, value: TypedArray<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `strides` attribute.
    pub fn strides(&self) -> TypedArray<u32> {
        self.inner.get("strides").as_::<TypedArray<u32>>()
    }

    /// Setter of the `strides` attribute.
    pub fn set_strides(&mut self, value: TypedArray<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `dilations` attribute.
    pub fn dilations(&self) -> TypedArray<u32> {
        self.inner.get("dilations").as_::<TypedArray<u32>>()
    }

    /// Setter of the `dilations` attribute.
    pub fn set_dilations(&mut self, value: TypedArray<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `groups` attribute.
    pub fn groups(&self) -> u32 {
        self.inner.get("groups").as_::<u32>()
    }

    /// Setter of the `groups` attribute.
    pub fn set_groups(&mut self, value: u32) {
        self.inner.set("groups", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `inputLayout` attribute.
    pub fn input_layout(&self) -> MLInputOperandLayout {
        self.inner.get("inputLayout").as_::<MLInputOperandLayout>()
    }

    /// Setter of the `inputLayout` attribute.
    pub fn set_input_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("inputLayout", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `filterLayout` attribute.
    pub fn filter_layout(&self) -> MLConv2dFilterOperandLayout {
        self.inner.get("filterLayout").as_::<MLConv2dFilterOperandLayout>()
    }

    /// Setter of the `filterLayout` attribute.
    pub fn set_filter_layout(&mut self, value: &MLConv2dFilterOperandLayout) {
        self.inner.set("filterLayout", value);
    }
}
impl MLConv2dOptions {
    /// Getter of the `bias` attribute.
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    /// Setter of the `bias` attribute.
    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
