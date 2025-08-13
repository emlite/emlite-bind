use super::*;




/// The MLContext class.
/// [`MLContext`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContext {
    inner: Any,
}

impl FromVal for MLContext {
    fn from_val(v: &Any) -> Self {
        MLContext { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for MLContext {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for MLContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for MLContext {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for MLContext {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<MLContext> for Any {
    fn from(s: MLContext) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&MLContext> for Any {
    fn from(s: &MLContext) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(MLContext);


impl MLContext {
    /// The dispatch method.
    /// [`MLContext.dispatch`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/dispatch)
    pub fn dispatch(&self, graph: &MLGraph, inputs: &Any, outputs: &Any) -> Undefined {
        self.inner.call("dispatch", &[graph.into(), inputs.into(), outputs.into(), ]).as_::<Undefined>()
    }
}
impl MLContext {
    /// The createTensor method.
    /// [`MLContext.createTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/createTensor)
    pub fn create_tensor(&self, descriptor: &MLTensorDescriptor) -> Promise<MLTensor> {
        self.inner.call("createTensor", &[descriptor.into(), ]).as_::<Promise<MLTensor>>()
    }
}
impl MLContext {
    /// The createConstantTensor method.
    /// [`MLContext.createConstantTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/createConstantTensor)
    pub fn create_constant_tensor(&self, descriptor: &MLOperandDescriptor, input_data: &Any) -> Promise<MLTensor> {
        self.inner.call("createConstantTensor", &[descriptor.into(), input_data.into(), ]).as_::<Promise<MLTensor>>()
    }
}
impl MLContext {
    /// The readTensor method.
    /// [`MLContext.readTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/readTensor)
    pub fn read_tensor(&self, tensor: &MLTensor, output_data: &Any) -> Promise<Undefined> {
        self.inner.call("readTensor", &[tensor.into(), output_data.into(), ]).as_::<Promise<Undefined>>()
    }
}
impl MLContext {
    /// The writeTensor method.
    /// [`MLContext.writeTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/writeTensor)
    pub fn write_tensor(&self, tensor: &MLTensor, input_data: &Any) -> Undefined {
        self.inner.call("writeTensor", &[tensor.into(), input_data.into(), ]).as_::<Undefined>()
    }
}
impl MLContext {
    /// The opSupportLimits method.
    /// [`MLContext.opSupportLimits`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/opSupportLimits)
    pub fn op_support_limits(&self, ) -> MLOpSupportLimits {
        self.inner.call("opSupportLimits", &[]).as_::<MLOpSupportLimits>()
    }
}
impl MLContext {
    /// The destroy method.
    /// [`MLContext.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/destroy)
    pub fn destroy(&self, ) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl MLContext {
    /// Getter of the `lost` attribute.
    /// [`MLContext.lost`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/lost)
    pub fn lost(&self) -> Promise<MLContextLostInfo> {
        self.inner.get("lost").as_::<Promise<MLContextLostInfo>>()
    }

}
