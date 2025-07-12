use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for MLTensorDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLTensorDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLTensorDescriptor> for emlite::Val {
    fn from(s: MLTensorDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MLOperandDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLOperandDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLOperandDescriptor> for emlite::Val {
    fn from(s: MLOperandDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
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
    pub fn shape(&self) -> jsbind::Sequence<u32> {
        self.inner.get("shape").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_shape(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("shape", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MLOpSupportLimits {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLOpSupportLimits {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLOpSupportLimits> for emlite::Val {
    fn from(s: MLOpSupportLimits) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOpSupportLimits {
    pub fn where_(&self) -> jsbind::Any {
        self.inner.get("where").as_::<jsbind::Any>()
    }

    pub fn set_where_(&mut self, value: jsbind::Any) {
        self.inner.set("where", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MLContextLostInfo {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLContextLostInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLContextLostInfo> for emlite::Val {
    fn from(s: MLContextLostInfo) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLContextLostInfo {
    pub fn message(&self) -> jsbind::DOMString {
        self.inner.get("message").as_::<jsbind::DOMString>()
    }

    pub fn set_message(&mut self, value: jsbind::DOMString) {
        self.inner.set("message", value);
    }
}
#[derive(Clone, Debug)]
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
impl std::ops::Deref for MLContext {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for MLContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLContext> for emlite::Val {
    fn from(s: MLContext) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLContext {
    pub fn dispatch(
        &self,
        graph: MLGraph,
        inputs: jsbind::Any,
        outputs: jsbind::Any,
    ) -> jsbind::Undefined {
        self.inner
            .call("dispatch", &[graph.into(), inputs.into(), outputs.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl MLContext {
    pub fn create_tensor(&self, descriptor: MLTensorDescriptor) -> jsbind::Promise {
        self.inner
            .call("createTensor", &[descriptor.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MLContext {
    pub fn create_constant_tensor(
        &self,
        descriptor: MLOperandDescriptor,
        input_data: jsbind::Any,
    ) -> jsbind::Promise {
        self.inner
            .call(
                "createConstantTensor",
                &[descriptor.into(), input_data.into()],
            )
            .as_::<jsbind::Promise>()
    }
}
impl MLContext {
    pub fn read_tensor(&self, tensor: MLTensor, output_data: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("readTensor", &[tensor.into(), output_data.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MLContext {
    pub fn write_tensor(&self, tensor: MLTensor, input_data: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("writeTensor", &[tensor.into(), input_data.into()])
            .as_::<jsbind::Undefined>()
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
    pub fn destroy(&self) -> jsbind::Undefined {
        self.inner.call("destroy", &[]).as_::<jsbind::Undefined>()
    }
}
impl MLContext {
    pub fn lost(&self) -> jsbind::Promise {
        self.inner.get("lost").as_::<jsbind::Promise>()
    }
}
