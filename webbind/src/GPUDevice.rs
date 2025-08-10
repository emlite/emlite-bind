use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBufferDescriptor {
    inner: Any,
}
impl FromVal for GPUBufferDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBufferDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBufferDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBufferDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBufferDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBufferDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBufferDescriptor> for Any {
    fn from(s: GPUBufferDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBufferDescriptor> for Any {
    fn from(s: &GPUBufferDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBufferDescriptor {
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }

    pub fn set_size(&mut self, value: &Any) {
        self.inner.set("size", value);
    }
}
impl GPUBufferDescriptor {
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUBufferDescriptor {
    pub fn mapped_at_creation(&self) -> bool {
        self.inner.get("mappedAtCreation").as_::<bool>()
    }

    pub fn set_mapped_at_creation(&mut self, value: bool) {
        self.inner.set("mappedAtCreation", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUTextureDescriptor {
    inner: Any,
}
impl FromVal for GPUTextureDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTextureDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUTextureDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUTextureDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUTextureDescriptor> for Any {
    fn from(s: GPUTextureDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUTextureDescriptor> for Any {
    fn from(s: &GPUTextureDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUTextureDescriptor {
    pub fn size(&self) -> Any {
        self.inner.get("size").as_::<Any>()
    }

    pub fn set_size(&mut self, value: &Any) {
        self.inner.set("size", value);
    }
}
impl GPUTextureDescriptor {
    pub fn mip_level_count(&self) -> Any {
        self.inner.get("mipLevelCount").as_::<Any>()
    }

    pub fn set_mip_level_count(&mut self, value: &Any) {
        self.inner.set("mipLevelCount", value);
    }
}
impl GPUTextureDescriptor {
    pub fn sample_count(&self) -> Any {
        self.inner.get("sampleCount").as_::<Any>()
    }

    pub fn set_sample_count(&mut self, value: &Any) {
        self.inner.set("sampleCount", value);
    }
}
impl GPUTextureDescriptor {
    pub fn dimension(&self) -> GPUTextureDimension {
        self.inner.get("dimension").as_::<GPUTextureDimension>()
    }

    pub fn set_dimension(&mut self, value: &GPUTextureDimension) {
        self.inner.set("dimension", value);
    }
}
impl GPUTextureDescriptor {
    pub fn format(&self) -> GPUTextureFormat {
        self.inner.get("format").as_::<GPUTextureFormat>()
    }

    pub fn set_format(&mut self, value: &GPUTextureFormat) {
        self.inner.set("format", value);
    }
}
impl GPUTextureDescriptor {
    pub fn usage(&self) -> Any {
        self.inner.get("usage").as_::<Any>()
    }

    pub fn set_usage(&mut self, value: &Any) {
        self.inner.set("usage", value);
    }
}
impl GPUTextureDescriptor {
    pub fn view_formats(&self) -> TypedArray<GPUTextureFormat> {
        self.inner
            .get("viewFormats")
            .as_::<TypedArray<GPUTextureFormat>>()
    }

    pub fn set_view_formats(&mut self, value: &TypedArray<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSamplerDescriptor {
    inner: Any,
}
impl FromVal for GPUSamplerDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUSamplerDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUSamplerDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUSamplerDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUSamplerDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUSamplerDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUSamplerDescriptor> for Any {
    fn from(s: GPUSamplerDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUSamplerDescriptor> for Any {
    fn from(s: &GPUSamplerDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUSamplerDescriptor {
    pub fn address_mode_u(&self) -> GPUAddressMode {
        self.inner.get("addressModeU").as_::<GPUAddressMode>()
    }

    pub fn set_address_mode_u(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeU", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn address_mode_v(&self) -> GPUAddressMode {
        self.inner.get("addressModeV").as_::<GPUAddressMode>()
    }

    pub fn set_address_mode_v(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeV", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn address_mode_w(&self) -> GPUAddressMode {
        self.inner.get("addressModeW").as_::<GPUAddressMode>()
    }

    pub fn set_address_mode_w(&mut self, value: &GPUAddressMode) {
        self.inner.set("addressModeW", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn mag_filter(&self) -> GPUFilterMode {
        self.inner.get("magFilter").as_::<GPUFilterMode>()
    }

    pub fn set_mag_filter(&mut self, value: &GPUFilterMode) {
        self.inner.set("magFilter", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn min_filter(&self) -> GPUFilterMode {
        self.inner.get("minFilter").as_::<GPUFilterMode>()
    }

    pub fn set_min_filter(&mut self, value: &GPUFilterMode) {
        self.inner.set("minFilter", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn mipmap_filter(&self) -> GPUMipmapFilterMode {
        self.inner.get("mipmapFilter").as_::<GPUMipmapFilterMode>()
    }

    pub fn set_mipmap_filter(&mut self, value: &GPUMipmapFilterMode) {
        self.inner.set("mipmapFilter", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn lod_min_clamp(&self) -> f32 {
        self.inner.get("lodMinClamp").as_::<f32>()
    }

    pub fn set_lod_min_clamp(&mut self, value: f32) {
        self.inner.set("lodMinClamp", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn lod_max_clamp(&self) -> f32 {
        self.inner.get("lodMaxClamp").as_::<f32>()
    }

    pub fn set_lod_max_clamp(&mut self, value: f32) {
        self.inner.set("lodMaxClamp", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn compare(&self) -> GPUCompareFunction {
        self.inner.get("compare").as_::<GPUCompareFunction>()
    }

    pub fn set_compare(&mut self, value: &GPUCompareFunction) {
        self.inner.set("compare", value);
    }
}
impl GPUSamplerDescriptor {
    pub fn max_anisotropy(&self) -> u16 {
        self.inner.get("maxAnisotropy").as_::<u16>()
    }

    pub fn set_max_anisotropy(&mut self, value: u16) {
        self.inner.set("maxAnisotropy", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUExternalTextureDescriptor {
    inner: Any,
}
impl FromVal for GPUExternalTextureDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUExternalTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUExternalTextureDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUExternalTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUExternalTextureDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUExternalTextureDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUExternalTextureDescriptor> for Any {
    fn from(s: GPUExternalTextureDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUExternalTextureDescriptor> for Any {
    fn from(s: &GPUExternalTextureDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUExternalTextureDescriptor {
    pub fn source(&self) -> Any {
        self.inner.get("source").as_::<Any>()
    }

    pub fn set_source(&mut self, value: &Any) {
        self.inner.set("source", value);
    }
}
impl GPUExternalTextureDescriptor {
    pub fn color_space(&self) -> PredefinedColorSpace {
        self.inner.get("colorSpace").as_::<PredefinedColorSpace>()
    }

    pub fn set_color_space(&mut self, value: &PredefinedColorSpace) {
        self.inner.set("colorSpace", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupLayoutDescriptor {
    inner: Any,
}
impl FromVal for GPUBindGroupLayoutDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBindGroupLayoutDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroupLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBindGroupLayoutDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBindGroupLayoutDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBindGroupLayoutDescriptor> for Any {
    fn from(s: GPUBindGroupLayoutDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBindGroupLayoutDescriptor> for Any {
    fn from(s: &GPUBindGroupLayoutDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupLayoutDescriptor {
    pub fn entries(&self) -> TypedArray<GPUBindGroupLayoutEntry> {
        self.inner
            .get("entries")
            .as_::<TypedArray<GPUBindGroupLayoutEntry>>()
    }

    pub fn set_entries(&mut self, value: &TypedArray<GPUBindGroupLayoutEntry>) {
        self.inner.set("entries", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineLayoutDescriptor {
    inner: Any,
}
impl FromVal for GPUPipelineLayoutDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUPipelineLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUPipelineLayoutDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUPipelineLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUPipelineLayoutDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUPipelineLayoutDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUPipelineLayoutDescriptor> for Any {
    fn from(s: GPUPipelineLayoutDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUPipelineLayoutDescriptor> for Any {
    fn from(s: &GPUPipelineLayoutDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUPipelineLayoutDescriptor {
    pub fn bind_group_layouts(&self) -> TypedArray<GPUBindGroupLayout> {
        self.inner
            .get("bindGroupLayouts")
            .as_::<TypedArray<GPUBindGroupLayout>>()
    }

    pub fn set_bind_group_layouts(&mut self, value: &TypedArray<GPUBindGroupLayout>) {
        self.inner.set("bindGroupLayouts", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupDescriptor {
    inner: Any,
}
impl FromVal for GPUBindGroupDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUBindGroupDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBindGroupDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroupDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUBindGroupDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUBindGroupDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUBindGroupDescriptor> for Any {
    fn from(s: GPUBindGroupDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUBindGroupDescriptor> for Any {
    fn from(s: &GPUBindGroupDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUBindGroupDescriptor {
    pub fn layout(&self) -> GPUBindGroupLayout {
        self.inner.get("layout").as_::<GPUBindGroupLayout>()
    }

    pub fn set_layout(&mut self, value: &GPUBindGroupLayout) {
        self.inner.set("layout", value);
    }
}
impl GPUBindGroupDescriptor {
    pub fn entries(&self) -> TypedArray<GPUBindGroupEntry> {
        self.inner
            .get("entries")
            .as_::<TypedArray<GPUBindGroupEntry>>()
    }

    pub fn set_entries(&mut self, value: &TypedArray<GPUBindGroupEntry>) {
        self.inner.set("entries", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUShaderModuleDescriptor {
    inner: Any,
}
impl FromVal for GPUShaderModuleDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUShaderModuleDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUShaderModuleDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUShaderModuleDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUShaderModuleDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUShaderModuleDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUShaderModuleDescriptor> for Any {
    fn from(s: GPUShaderModuleDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUShaderModuleDescriptor> for Any {
    fn from(s: &GPUShaderModuleDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUShaderModuleDescriptor {
    pub fn code(&self) -> JsString {
        self.inner.get("code").as_::<JsString>()
    }

    pub fn set_code(&mut self, value: &JsString) {
        self.inner.set("code", value);
    }
}
impl GPUShaderModuleDescriptor {
    pub fn compilation_hints(&self) -> TypedArray<GPUShaderModuleCompilationHint> {
        self.inner
            .get("compilationHints")
            .as_::<TypedArray<GPUShaderModuleCompilationHint>>()
    }

    pub fn set_compilation_hints(&mut self, value: &TypedArray<GPUShaderModuleCompilationHint>) {
        self.inner.set("compilationHints", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePipelineDescriptor {
    inner: Any,
}
impl FromVal for GPUComputePipelineDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUComputePipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUComputePipelineDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUComputePipelineDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUComputePipelineDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUComputePipelineDescriptor> for Any {
    fn from(s: GPUComputePipelineDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUComputePipelineDescriptor> for Any {
    fn from(s: &GPUComputePipelineDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUComputePipelineDescriptor {
    pub fn compute(&self) -> GPUProgrammableStage {
        self.inner.get("compute").as_::<GPUProgrammableStage>()
    }

    pub fn set_compute(&mut self, value: &GPUProgrammableStage) {
        self.inner.set("compute", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPipelineDescriptor {
    inner: Any,
}
impl FromVal for GPURenderPipelineDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderPipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPipelineDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURenderPipelineDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURenderPipelineDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURenderPipelineDescriptor> for Any {
    fn from(s: GPURenderPipelineDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURenderPipelineDescriptor> for Any {
    fn from(s: &GPURenderPipelineDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPURenderPipelineDescriptor {
    pub fn vertex(&self) -> GPUVertexState {
        self.inner.get("vertex").as_::<GPUVertexState>()
    }

    pub fn set_vertex(&mut self, value: &GPUVertexState) {
        self.inner.set("vertex", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn primitive(&self) -> GPUPrimitiveState {
        self.inner.get("primitive").as_::<GPUPrimitiveState>()
    }

    pub fn set_primitive(&mut self, value: &GPUPrimitiveState) {
        self.inner.set("primitive", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn depth_stencil(&self) -> GPUDepthStencilState {
        self.inner.get("depthStencil").as_::<GPUDepthStencilState>()
    }

    pub fn set_depth_stencil(&mut self, value: &GPUDepthStencilState) {
        self.inner.set("depthStencil", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn multisample(&self) -> GPUMultisampleState {
        self.inner.get("multisample").as_::<GPUMultisampleState>()
    }

    pub fn set_multisample(&mut self, value: &GPUMultisampleState) {
        self.inner.set("multisample", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn fragment(&self) -> GPUFragmentState {
        self.inner.get("fragment").as_::<GPUFragmentState>()
    }

    pub fn set_fragment(&mut self, value: &GPUFragmentState) {
        self.inner.set("fragment", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandEncoderDescriptor {
    inner: Any,
}
impl FromVal for GPUCommandEncoderDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUCommandEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCommandEncoderDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUCommandEncoderDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUCommandEncoderDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUCommandEncoderDescriptor> for Any {
    fn from(s: GPUCommandEncoderDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUCommandEncoderDescriptor> for Any {
    fn from(s: &GPUCommandEncoderDescriptor) -> Any {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundleEncoderDescriptor {
    inner: Any,
}
impl FromVal for GPURenderBundleEncoderDescriptor {
    fn from_val(v: &Any) -> Self {
        GPURenderBundleEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderBundleEncoderDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderBundleEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPURenderBundleEncoderDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPURenderBundleEncoderDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPURenderBundleEncoderDescriptor> for Any {
    fn from(s: GPURenderBundleEncoderDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPURenderBundleEncoderDescriptor> for Any {
    fn from(s: &GPURenderBundleEncoderDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPURenderBundleEncoderDescriptor {
    pub fn depth_read_only(&self) -> bool {
        self.inner.get("depthReadOnly").as_::<bool>()
    }

    pub fn set_depth_read_only(&mut self, value: bool) {
        self.inner.set("depthReadOnly", value);
    }
}
impl GPURenderBundleEncoderDescriptor {
    pub fn stencil_read_only(&self) -> bool {
        self.inner.get("stencilReadOnly").as_::<bool>()
    }

    pub fn set_stencil_read_only(&mut self, value: bool) {
        self.inner.set("stencilReadOnly", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUQuerySetDescriptor {
    inner: Any,
}
impl FromVal for GPUQuerySetDescriptor {
    fn from_val(v: &Any) -> Self {
        GPUQuerySetDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUQuerySetDescriptor {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUQuerySetDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUQuerySetDescriptor {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUQuerySetDescriptor {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUQuerySetDescriptor> for Any {
    fn from(s: GPUQuerySetDescriptor) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUQuerySetDescriptor> for Any {
    fn from(s: &GPUQuerySetDescriptor) -> Any {
        s.inner.clone()
    }
}

impl GPUQuerySetDescriptor {
    pub fn type_(&self) -> GPUQueryType {
        self.inner.get("type").as_::<GPUQueryType>()
    }

    pub fn set_type_(&mut self, value: &GPUQueryType) {
        self.inner.set("type", value);
    }
}
impl GPUQuerySetDescriptor {
    pub fn count(&self) -> Any {
        self.inner.get("count").as_::<Any>()
    }

    pub fn set_count(&mut self, value: &Any) {
        self.inner.set("count", value);
    }
}
/// The GPUDevice class.
/// [`GPUDevice`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDevice {
    inner: EventTarget,
}
impl FromVal for GPUDevice {
    fn from_val(v: &Any) -> Self {
        GPUDevice {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUDevice {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUDevice {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for GPUDevice {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for GPUDevice {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<GPUDevice> for Any {
    fn from(s: GPUDevice) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&GPUDevice> for Any {
    fn from(s: &GPUDevice) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUDevice);

impl GPUDevice {
    /// Getter of the `features` attribute.
    /// [`GPUDevice.features`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/features)
    pub fn features(&self) -> GPUSupportedFeatures {
        self.inner.get("features").as_::<GPUSupportedFeatures>()
    }
}
impl GPUDevice {
    /// Getter of the `limits` attribute.
    /// [`GPUDevice.limits`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/limits)
    pub fn limits(&self) -> GPUSupportedLimits {
        self.inner.get("limits").as_::<GPUSupportedLimits>()
    }
}
impl GPUDevice {
    /// Getter of the `adapterInfo` attribute.
    /// [`GPUDevice.adapterInfo`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/adapterInfo)
    pub fn adapter_info(&self) -> GPUAdapterInfo {
        self.inner.get("adapterInfo").as_::<GPUAdapterInfo>()
    }
}
impl GPUDevice {
    /// Getter of the `queue` attribute.
    /// [`GPUDevice.queue`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/queue)
    pub fn queue(&self) -> GPUQueue {
        self.inner.get("queue").as_::<GPUQueue>()
    }
}
impl GPUDevice {
    /// The destroy method.
    /// [`GPUDevice.destroy`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/destroy)
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUDevice {
    /// The createBuffer method.
    /// [`GPUDevice.createBuffer`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBuffer)
    pub fn create_buffer(&self, descriptor: &GPUBufferDescriptor) -> GPUBuffer {
        self.inner
            .call("createBuffer", &[descriptor.into()])
            .as_::<GPUBuffer>()
    }
}
impl GPUDevice {
    /// The createTexture method.
    /// [`GPUDevice.createTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createTexture)
    pub fn create_texture(&self, descriptor: &GPUTextureDescriptor) -> GPUTexture {
        self.inner
            .call("createTexture", &[descriptor.into()])
            .as_::<GPUTexture>()
    }
}
impl GPUDevice {
    /// The createSampler method.
    /// [`GPUDevice.createSampler`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    pub fn create_sampler0(&self) -> GPUSampler {
        self.inner.call("createSampler", &[]).as_::<GPUSampler>()
    }
    /// The createSampler method.
    /// [`GPUDevice.createSampler`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createSampler)
    pub fn create_sampler1(&self, descriptor: &GPUSamplerDescriptor) -> GPUSampler {
        self.inner
            .call("createSampler", &[descriptor.into()])
            .as_::<GPUSampler>()
    }
}
impl GPUDevice {
    /// The importExternalTexture method.
    /// [`GPUDevice.importExternalTexture`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/importExternalTexture)
    pub fn import_external_texture(
        &self,
        descriptor: &GPUExternalTextureDescriptor,
    ) -> GPUExternalTexture {
        self.inner
            .call("importExternalTexture", &[descriptor.into()])
            .as_::<GPUExternalTexture>()
    }
}
impl GPUDevice {
    /// The createBindGroupLayout method.
    /// [`GPUDevice.createBindGroupLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroupLayout)
    pub fn create_bind_group_layout(
        &self,
        descriptor: &GPUBindGroupLayoutDescriptor,
    ) -> GPUBindGroupLayout {
        self.inner
            .call("createBindGroupLayout", &[descriptor.into()])
            .as_::<GPUBindGroupLayout>()
    }
}
impl GPUDevice {
    /// The createPipelineLayout method.
    /// [`GPUDevice.createPipelineLayout`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createPipelineLayout)
    pub fn create_pipeline_layout(
        &self,
        descriptor: &GPUPipelineLayoutDescriptor,
    ) -> GPUPipelineLayout {
        self.inner
            .call("createPipelineLayout", &[descriptor.into()])
            .as_::<GPUPipelineLayout>()
    }
}
impl GPUDevice {
    /// The createBindGroup method.
    /// [`GPUDevice.createBindGroup`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createBindGroup)
    pub fn create_bind_group(&self, descriptor: &GPUBindGroupDescriptor) -> GPUBindGroup {
        self.inner
            .call("createBindGroup", &[descriptor.into()])
            .as_::<GPUBindGroup>()
    }
}
impl GPUDevice {
    /// The createShaderModule method.
    /// [`GPUDevice.createShaderModule`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createShaderModule)
    pub fn create_shader_module(&self, descriptor: &GPUShaderModuleDescriptor) -> GPUShaderModule {
        self.inner
            .call("createShaderModule", &[descriptor.into()])
            .as_::<GPUShaderModule>()
    }
}
impl GPUDevice {
    /// The createComputePipeline method.
    /// [`GPUDevice.createComputePipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipeline)
    pub fn create_compute_pipeline(
        &self,
        descriptor: &GPUComputePipelineDescriptor,
    ) -> GPUComputePipeline {
        self.inner
            .call("createComputePipeline", &[descriptor.into()])
            .as_::<GPUComputePipeline>()
    }
}
impl GPUDevice {
    /// The createRenderPipeline method.
    /// [`GPUDevice.createRenderPipeline`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipeline)
    pub fn create_render_pipeline(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> GPURenderPipeline {
        self.inner
            .call("createRenderPipeline", &[descriptor.into()])
            .as_::<GPURenderPipeline>()
    }
}
impl GPUDevice {
    /// The createComputePipelineAsync method.
    /// [`GPUDevice.createComputePipelineAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createComputePipelineAsync)
    pub fn create_compute_pipeline_async(
        &self,
        descriptor: &GPUComputePipelineDescriptor,
    ) -> Promise<GPUComputePipeline> {
        self.inner
            .call("createComputePipelineAsync", &[descriptor.into()])
            .as_::<Promise<GPUComputePipeline>>()
    }
}
impl GPUDevice {
    /// The createRenderPipelineAsync method.
    /// [`GPUDevice.createRenderPipelineAsync`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderPipelineAsync)
    pub fn create_render_pipeline_async(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> Promise<GPURenderPipeline> {
        self.inner
            .call("createRenderPipelineAsync", &[descriptor.into()])
            .as_::<Promise<GPURenderPipeline>>()
    }
}
impl GPUDevice {
    /// The createCommandEncoder method.
    /// [`GPUDevice.createCommandEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    pub fn create_command_encoder0(&self) -> GPUCommandEncoder {
        self.inner
            .call("createCommandEncoder", &[])
            .as_::<GPUCommandEncoder>()
    }
    /// The createCommandEncoder method.
    /// [`GPUDevice.createCommandEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createCommandEncoder)
    pub fn create_command_encoder1(
        &self,
        descriptor: &GPUCommandEncoderDescriptor,
    ) -> GPUCommandEncoder {
        self.inner
            .call("createCommandEncoder", &[descriptor.into()])
            .as_::<GPUCommandEncoder>()
    }
}
impl GPUDevice {
    /// The createRenderBundleEncoder method.
    /// [`GPUDevice.createRenderBundleEncoder`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createRenderBundleEncoder)
    pub fn create_render_bundle_encoder(
        &self,
        descriptor: &GPURenderBundleEncoderDescriptor,
    ) -> GPURenderBundleEncoder {
        self.inner
            .call("createRenderBundleEncoder", &[descriptor.into()])
            .as_::<GPURenderBundleEncoder>()
    }
}
impl GPUDevice {
    /// The createQuerySet method.
    /// [`GPUDevice.createQuerySet`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/createQuerySet)
    pub fn create_query_set(&self, descriptor: &GPUQuerySetDescriptor) -> GPUQuerySet {
        self.inner
            .call("createQuerySet", &[descriptor.into()])
            .as_::<GPUQuerySet>()
    }
}
impl GPUDevice {
    /// Getter of the `lost` attribute.
    /// [`GPUDevice.lost`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/lost)
    pub fn lost(&self) -> Promise<GPUDeviceLostInfo> {
        self.inner.get("lost").as_::<Promise<GPUDeviceLostInfo>>()
    }
}
impl GPUDevice {
    /// The pushErrorScope method.
    /// [`GPUDevice.pushErrorScope`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/pushErrorScope)
    pub fn push_error_scope(&self, filter: &GPUErrorFilter) -> Undefined {
        self.inner
            .call("pushErrorScope", &[filter.into()])
            .as_::<Undefined>()
    }
}
impl GPUDevice {
    /// The popErrorScope method.
    /// [`GPUDevice.popErrorScope`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/popErrorScope)
    pub fn pop_error_scope(&self) -> Promise<GPUError> {
        self.inner
            .call("popErrorScope", &[])
            .as_::<Promise<GPUError>>()
    }
}
impl GPUDevice {
    /// Getter of the `onuncapturederror` attribute.
    /// [`GPUDevice.onuncapturederror`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    pub fn onuncapturederror(&self) -> Any {
        self.inner.get("onuncapturederror").as_::<Any>()
    }

    /// Setter of the `onuncapturederror` attribute.
    /// [`GPUDevice.onuncapturederror`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/onuncapturederror)
    pub fn set_onuncapturederror(&mut self, value: &Any) {
        self.inner.set("onuncapturederror", value);
    }
}
impl GPUDevice {
    /// Getter of the `label` attribute.
    /// [`GPUDevice.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    pub fn label(&self) -> JsString {
        self.inner.get("label").as_::<JsString>()
    }

    /// Setter of the `label` attribute.
    /// [`GPUDevice.label`](https://developer.mozilla.org/en-US/docs/Web/API/GPUDevice/label)
    pub fn set_label(&mut self, value: &JsString) {
        self.inner.set("label", value);
    }
}
