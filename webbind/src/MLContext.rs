use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensorDescriptor {
    inner: emlite::Val,
}
impl FromVal for MLTensorDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        MLTensorDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTensorDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTensorDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLTensorDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLTensorDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLTensorDescriptor> for emlite::Val {
    fn from(s: MLTensorDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
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
    inner: emlite::Val,
}
impl FromVal for MLOperandDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        MLOperandDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOperandDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperandDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLOperandDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLOperandDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLOperandDescriptor> for emlite::Val {
    fn from(s: MLOperandDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOperandDescriptor {
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }

    pub fn set_data_type(&mut self, value: MLOperandDataType) {
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
    inner: emlite::Val,
}
impl FromVal for MLOpSupportLimits {
    fn from_val(v: &emlite::Val) -> Self {
        MLOpSupportLimits { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOpSupportLimits {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOpSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLOpSupportLimits {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLOpSupportLimits {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLOpSupportLimits> for emlite::Val {
    fn from(s: MLOpSupportLimits) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOpSupportLimits {
    pub fn where_(&self) -> Any {
        self.inner.get("where").as_::<Any>()
    }

    pub fn set_where_(&mut self, value: Any) {
        self.inner.set("where", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContextLostInfo {
    inner: emlite::Val,
}
impl FromVal for MLContextLostInfo {
    fn from_val(v: &emlite::Val) -> Self {
        MLContextLostInfo { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLContextLostInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLContextLostInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLContextLostInfo {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLContextLostInfo {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLContextLostInfo> for emlite::Val {
    fn from(s: MLContextLostInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLContextLostInfo {
    pub fn message(&self) -> DOMString {
        self.inner.get("message").as_::<DOMString>()
    }

    pub fn set_message(&mut self, value: DOMString) {
        self.inner.set("message", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLContext {
    inner: emlite::Val,
}
impl FromVal for MLContext {
    fn from_val(v: &emlite::Val) -> Self {
        MLContext {
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
impl core::ops::Deref for MLContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLContext {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLContext {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLContext> for emlite::Val {
    fn from(s: MLContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MLContext);

impl MLContext {
    pub fn dispatch(&self, graph: MLGraph, inputs: Any, outputs: Any) -> Undefined {
        self.inner
            .call("dispatch", &[graph.into(), inputs.into(), outputs.into()])
            .as_::<Undefined>()
    }
}
impl MLContext {
    pub fn create_tensor(&self, descriptor: MLTensorDescriptor) -> Promise {
        self.inner
            .call("createTensor", &[descriptor.into()])
            .as_::<Promise>()
    }
}
impl MLContext {
    pub fn create_constant_tensor(
        &self,
        descriptor: MLOperandDescriptor,
        input_data: Any,
    ) -> Promise {
        self.inner
            .call(
                "createConstantTensor",
                &[descriptor.into(), input_data.into()],
            )
            .as_::<Promise>()
    }
}
impl MLContext {
    pub fn read_tensor(&self, tensor: MLTensor, output_data: Any) -> Promise {
        self.inner
            .call("readTensor", &[tensor.into(), output_data.into()])
            .as_::<Promise>()
    }
}
impl MLContext {
    pub fn write_tensor(&self, tensor: MLTensor, input_data: Any) -> Undefined {
        self.inner
            .call("writeTensor", &[tensor.into(), input_data.into()])
            .as_::<Undefined>()
    }
}
impl MLContext {
    pub fn op_support_limits(&self) -> MLOpSupportLimits {
        self.inner
            .call("opSupportLimits", &[])
            .as_::<MLOpSupportLimits>()
    }
}
impl MLContext {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl MLContext {
    pub fn lost(&self) -> Promise {
        self.inner.get("lost").as_::<Promise>()
    }
}
