use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensorDescriptor {
    inner: Any,
}
impl FromVal for MLTensorDescriptor {
    fn from_val(v: &Any) -> Self {
        MLTensorDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTensorDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTensorDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLTensorDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLTensorDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLTensorDescriptor> for Any {
    fn from(s: MLTensorDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLTensorDescriptor> for Any {
    fn from(s: &MLTensorDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MLTensorDescriptor {
    pub fn readable(&self) -> bool {
        self.inner.get("readable").as_::<bool>()
    }

    pub fn set_readable(&mut self, value: bool) {
        self.inner.set("readable", value);
    }
}
impl MLTensorDescriptor {
    pub fn writable(&self) -> bool {
        self.inner.get("writable").as_::<bool>()
    }

    pub fn set_writable(&mut self, value: bool) {
        self.inner.set("writable", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOperandDescriptor {
    inner: Any,
}
impl FromVal for MLOperandDescriptor {
    fn from_val(v: &Any) -> Self {
        MLOperandDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOperandDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperandDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLOperandDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLOperandDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLOperandDescriptor> for Any {
    fn from(s: MLOperandDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLOperandDescriptor> for Any {
    fn from(s: &MLOperandDescriptor) -> Any {
        s.inner.clone()
    }
}

impl MLOperandDescriptor {
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }

    pub fn set_data_type(&mut self, value: &MLOperandDataType) {
        self.inner.set("dataType", value);
    }
}
impl MLOperandDescriptor {
    pub fn shape(&self) -> Sequence<u32> {
        self.inner.get("shape").as_::<Sequence<u32>>()
    }

    pub fn set_shape(&mut self, value: Sequence<u32>) {
        self.inner.set("shape", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLOpSupportLimits {
    inner: Any,
}
impl FromVal for MLOpSupportLimits {
    fn from_val(v: &Any) -> Self {
        MLOpSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOpSupportLimits {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOpSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLOpSupportLimits {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLOpSupportLimits {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLOpSupportLimits> for Any {
    fn from(s: MLOpSupportLimits) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLOpSupportLimits> for Any {
    fn from(s: &MLOpSupportLimits) -> Any {
        s.inner.clone()
    }
}

impl MLOpSupportLimits {
    pub fn where_(&self) -> Any {
        self.inner.get("where").as_::<Any>()
    }

    pub fn set_where_(&mut self, value: &Any) {
        self.inner.set("where", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContextLostInfo {
    inner: Any,
}
impl FromVal for MLContextLostInfo {
    fn from_val(v: &Any) -> Self {
        MLContextLostInfo { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLContextLostInfo {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLContextLostInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLContextLostInfo {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLContextLostInfo {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLContextLostInfo> for Any {
    fn from(s: MLContextLostInfo) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLContextLostInfo> for Any {
    fn from(s: &MLContextLostInfo) -> Any {
        s.inner.clone()
    }
}

impl MLContextLostInfo {
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }

    pub fn set_message(&mut self, value: &DOMString) {
        self.inner.set("message", value);
    }
}
/// The MLContext class.
/// [`MLContext`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContext {
    inner: Any,
}
impl FromVal for MLContext {
    fn from_val(v: &Any) -> Self {
        MLContext {
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
        self.inner
            .call("dispatch", &[graph.into(), inputs.into(), outputs.into()])
            .as_::<Undefined>()
    }
}
impl MLContext {
    /// The createTensor method.
    /// [`MLContext.createTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/createTensor)
    pub fn create_tensor(&self, descriptor: &MLTensorDescriptor) -> Promise<MLTensor> {
        self.inner
            .call("createTensor", &[descriptor.into()])
            .as_::<Promise<MLTensor>>()
    }
}
impl MLContext {
    /// The createConstantTensor method.
    /// [`MLContext.createConstantTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/createConstantTensor)
    pub fn create_constant_tensor(
        &self,
        descriptor: &MLOperandDescriptor,
        input_data: &Any,
    ) -> Promise<MLTensor> {
        self.inner
            .call(
                "createConstantTensor",
                &[descriptor.into(), input_data.into()],
            )
            .as_::<Promise<MLTensor>>()
    }
}
impl MLContext {
    /// The readTensor method.
    /// [`MLContext.readTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/readTensor)
    pub fn read_tensor(&self, tensor: &MLTensor, output_data: &Any) -> Promise<Undefined> {
        self.inner
            .call("readTensor", &[tensor.into(), output_data.into()])
            .as_::<Promise<Undefined>>()
    }
}
impl MLContext {
    /// The writeTensor method.
    /// [`MLContext.writeTensor`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/writeTensor)
    pub fn write_tensor(&self, tensor: &MLTensor, input_data: &Any) -> Undefined {
        self.inner
            .call("writeTensor", &[tensor.into(), input_data.into()])
            .as_::<Undefined>()
    }
}
impl MLContext {
    /// The opSupportLimits method.
    /// [`MLContext.opSupportLimits`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/opSupportLimits)
    pub fn op_support_limits(&self) -> MLOpSupportLimits {
        self.inner
            .call("opSupportLimits", &[])
            .as_::<MLOpSupportLimits>()
    }
}
impl MLContext {
    /// The destroy method.
    /// [`MLContext.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/MLContext/destroy)
    pub fn destroy(&self) -> Undefined {
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
