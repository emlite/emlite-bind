use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBufferDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUBufferDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBufferDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBufferDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBufferDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUBufferDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUBufferDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUBufferDescriptor> for emlite::Val {
    fn from(s: GPUBufferDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUBufferDescriptor> for emlite::Val {
    fn from(s: &GPUBufferDescriptor) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for GPUTextureDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUTextureDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUTextureDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUTextureDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUTextureDescriptor> for emlite::Val {
    fn from(s: GPUTextureDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUTextureDescriptor> for emlite::Val {
    fn from(s: &GPUTextureDescriptor) -> emlite::Val {
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
    pub fn view_formats(&self) -> Sequence<GPUTextureFormat> {
        self.inner
            .get("viewFormats")
            .as_::<Sequence<GPUTextureFormat>>()
    }

    pub fn set_view_formats(&mut self, value: &Sequence<GPUTextureFormat>) {
        self.inner.set("viewFormats", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUSamplerDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUSamplerDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUSamplerDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUSamplerDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUSamplerDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUSamplerDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUSamplerDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUSamplerDescriptor> for emlite::Val {
    fn from(s: GPUSamplerDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUSamplerDescriptor> for emlite::Val {
    fn from(s: &GPUSamplerDescriptor) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for GPUExternalTextureDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUExternalTextureDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUExternalTextureDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUExternalTextureDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUExternalTextureDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUExternalTextureDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUExternalTextureDescriptor> for emlite::Val {
    fn from(s: GPUExternalTextureDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUExternalTextureDescriptor> for emlite::Val {
    fn from(s: &GPUExternalTextureDescriptor) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for GPUBindGroupLayoutDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBindGroupLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBindGroupLayoutDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroupLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUBindGroupLayoutDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUBindGroupLayoutDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUBindGroupLayoutDescriptor> for emlite::Val {
    fn from(s: GPUBindGroupLayoutDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUBindGroupLayoutDescriptor> for emlite::Val {
    fn from(s: &GPUBindGroupLayoutDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUBindGroupLayoutDescriptor {
    pub fn entries(&self) -> Sequence<Any> {
        self.inner.get("entries").as_::<Sequence<Any>>()
    }

    pub fn set_entries(&mut self, value: &Sequence<Any>) {
        self.inner.set("entries", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUPipelineLayoutDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUPipelineLayoutDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUPipelineLayoutDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUPipelineLayoutDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUPipelineLayoutDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUPipelineLayoutDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUPipelineLayoutDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUPipelineLayoutDescriptor> for emlite::Val {
    fn from(s: GPUPipelineLayoutDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUPipelineLayoutDescriptor> for emlite::Val {
    fn from(s: &GPUPipelineLayoutDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUPipelineLayoutDescriptor {
    pub fn bind_group_layouts(&self) -> Sequence<GPUBindGroupLayout> {
        self.inner
            .get("bindGroupLayouts")
            .as_::<Sequence<GPUBindGroupLayout>>()
    }

    pub fn set_bind_group_layouts(&mut self, value: &Sequence<GPUBindGroupLayout>) {
        self.inner.set("bindGroupLayouts", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUBindGroupDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUBindGroupDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUBindGroupDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUBindGroupDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUBindGroupDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUBindGroupDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUBindGroupDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUBindGroupDescriptor> for emlite::Val {
    fn from(s: GPUBindGroupDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUBindGroupDescriptor> for emlite::Val {
    fn from(s: &GPUBindGroupDescriptor) -> emlite::Val {
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
    pub fn entries(&self) -> Sequence<Any> {
        self.inner.get("entries").as_::<Sequence<Any>>()
    }

    pub fn set_entries(&mut self, value: &Sequence<Any>) {
        self.inner.set("entries", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUShaderModuleDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUShaderModuleDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUShaderModuleDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUShaderModuleDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUShaderModuleDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUShaderModuleDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUShaderModuleDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUShaderModuleDescriptor> for emlite::Val {
    fn from(s: GPUShaderModuleDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUShaderModuleDescriptor> for emlite::Val {
    fn from(s: &GPUShaderModuleDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUShaderModuleDescriptor {
    pub fn code(&self) -> String {
        self.inner.get("code").as_::<String>()
    }

    pub fn set_code(&mut self, value: &str) {
        self.inner.set("code", value);
    }
}
impl GPUShaderModuleDescriptor {
    pub fn compilation_hints(&self) -> Sequence<Any> {
        self.inner.get("compilationHints").as_::<Sequence<Any>>()
    }

    pub fn set_compilation_hints(&mut self, value: &Sequence<Any>) {
        self.inner.set("compilationHints", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUComputePipelineDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUComputePipelineDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUComputePipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUComputePipelineDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUComputePipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUComputePipelineDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUComputePipelineDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUComputePipelineDescriptor> for emlite::Val {
    fn from(s: GPUComputePipelineDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUComputePipelineDescriptor> for emlite::Val {
    fn from(s: &GPUComputePipelineDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPUComputePipelineDescriptor {
    pub fn compute(&self) -> Any {
        self.inner.get("compute").as_::<Any>()
    }

    pub fn set_compute(&mut self, value: &Any) {
        self.inner.set("compute", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderPipelineDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPURenderPipelineDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderPipelineDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderPipelineDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderPipelineDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPURenderPipelineDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPURenderPipelineDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPURenderPipelineDescriptor> for emlite::Val {
    fn from(s: GPURenderPipelineDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPURenderPipelineDescriptor> for emlite::Val {
    fn from(s: &GPURenderPipelineDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

impl GPURenderPipelineDescriptor {
    pub fn vertex(&self) -> Any {
        self.inner.get("vertex").as_::<Any>()
    }

    pub fn set_vertex(&mut self, value: &Any) {
        self.inner.set("vertex", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn primitive(&self) -> Any {
        self.inner.get("primitive").as_::<Any>()
    }

    pub fn set_primitive(&mut self, value: &Any) {
        self.inner.set("primitive", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn depth_stencil(&self) -> Any {
        self.inner.get("depthStencil").as_::<Any>()
    }

    pub fn set_depth_stencil(&mut self, value: &Any) {
        self.inner.set("depthStencil", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn multisample(&self) -> Any {
        self.inner.get("multisample").as_::<Any>()
    }

    pub fn set_multisample(&mut self, value: &Any) {
        self.inner.set("multisample", value);
    }
}
impl GPURenderPipelineDescriptor {
    pub fn fragment(&self) -> Any {
        self.inner.get("fragment").as_::<Any>()
    }

    pub fn set_fragment(&mut self, value: &Any) {
        self.inner.set("fragment", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUCommandEncoderDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPUCommandEncoderDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUCommandEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUCommandEncoderDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUCommandEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUCommandEncoderDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUCommandEncoderDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUCommandEncoderDescriptor> for emlite::Val {
    fn from(s: GPUCommandEncoderDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUCommandEncoderDescriptor> for emlite::Val {
    fn from(s: &GPUCommandEncoderDescriptor) -> emlite::Val {
        s.inner.clone()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPURenderBundleEncoderDescriptor {
    inner: emlite::Val,
}
impl FromVal for GPURenderBundleEncoderDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPURenderBundleEncoderDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPURenderBundleEncoderDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPURenderBundleEncoderDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPURenderBundleEncoderDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPURenderBundleEncoderDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPURenderBundleEncoderDescriptor> for emlite::Val {
    fn from(s: GPURenderBundleEncoderDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPURenderBundleEncoderDescriptor> for emlite::Val {
    fn from(s: &GPURenderBundleEncoderDescriptor) -> emlite::Val {
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
    inner: emlite::Val,
}
impl FromVal for GPUQuerySetDescriptor {
    fn from_val(v: &emlite::Val) -> Self {
        GPUQuerySetDescriptor { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for GPUQuerySetDescriptor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for GPUQuerySetDescriptor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for GPUQuerySetDescriptor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUQuerySetDescriptor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUQuerySetDescriptor> for emlite::Val {
    fn from(s: GPUQuerySetDescriptor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUQuerySetDescriptor> for emlite::Val {
    fn from(s: &GPUQuerySetDescriptor) -> emlite::Val {
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
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct GPUDevice {
    inner: EventTarget,
}
impl FromVal for GPUDevice {
    fn from_val(v: &emlite::Val) -> Self {
        GPUDevice {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
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
impl AsRef<emlite::Val> for GPUDevice {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for GPUDevice {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<GPUDevice> for emlite::Val {
    fn from(s: GPUDevice) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&GPUDevice> for emlite::Val {
    fn from(s: &GPUDevice) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(GPUDevice);

impl GPUDevice {
    pub fn features(&self) -> GPUSupportedFeatures {
        self.inner.get("features").as_::<GPUSupportedFeatures>()
    }
}
impl GPUDevice {
    pub fn limits(&self) -> GPUSupportedLimits {
        self.inner.get("limits").as_::<GPUSupportedLimits>()
    }
}
impl GPUDevice {
    pub fn adapter_info(&self) -> GPUAdapterInfo {
        self.inner.get("adapterInfo").as_::<GPUAdapterInfo>()
    }
}
impl GPUDevice {
    pub fn queue(&self) -> GPUQueue {
        self.inner.get("queue").as_::<GPUQueue>()
    }
}
impl GPUDevice {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
impl GPUDevice {
    pub fn create_buffer(&self, descriptor: &GPUBufferDescriptor) -> GPUBuffer {
        self.inner
            .call("createBuffer", &[descriptor.into()])
            .as_::<GPUBuffer>()
    }
}
impl GPUDevice {
    pub fn create_texture(&self, descriptor: &GPUTextureDescriptor) -> GPUTexture {
        self.inner
            .call("createTexture", &[descriptor.into()])
            .as_::<GPUTexture>()
    }
}
impl GPUDevice {
    pub fn create_sampler0(&self) -> GPUSampler {
        self.inner.call("createSampler", &[]).as_::<GPUSampler>()
    }

    pub fn create_sampler1(&self, descriptor: &GPUSamplerDescriptor) -> GPUSampler {
        self.inner
            .call("createSampler", &[descriptor.into()])
            .as_::<GPUSampler>()
    }
}
impl GPUDevice {
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
    pub fn create_bind_group(&self, descriptor: &GPUBindGroupDescriptor) -> GPUBindGroup {
        self.inner
            .call("createBindGroup", &[descriptor.into()])
            .as_::<GPUBindGroup>()
    }
}
impl GPUDevice {
    pub fn create_shader_module(&self, descriptor: &GPUShaderModuleDescriptor) -> GPUShaderModule {
        self.inner
            .call("createShaderModule", &[descriptor.into()])
            .as_::<GPUShaderModule>()
    }
}
impl GPUDevice {
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
    pub fn create_compute_pipeline_async(
        &self,
        descriptor: &GPUComputePipelineDescriptor,
    ) -> Promise {
        self.inner
            .call("createComputePipelineAsync", &[descriptor.into()])
            .as_::<Promise>()
    }
}
impl GPUDevice {
    pub fn create_render_pipeline_async(
        &self,
        descriptor: &GPURenderPipelineDescriptor,
    ) -> Promise {
        self.inner
            .call("createRenderPipelineAsync", &[descriptor.into()])
            .as_::<Promise>()
    }
}
impl GPUDevice {
    pub fn create_command_encoder0(&self) -> GPUCommandEncoder {
        self.inner
            .call("createCommandEncoder", &[])
            .as_::<GPUCommandEncoder>()
    }

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
    pub fn create_query_set(&self, descriptor: &GPUQuerySetDescriptor) -> GPUQuerySet {
        self.inner
            .call("createQuerySet", &[descriptor.into()])
            .as_::<GPUQuerySet>()
    }
}
impl GPUDevice {
    pub fn lost(&self) -> Promise {
        self.inner.get("lost").as_::<Promise>()
    }
}
impl GPUDevice {
    pub fn push_error_scope(&self, filter: &GPUErrorFilter) -> Undefined {
        self.inner
            .call("pushErrorScope", &[filter.into()])
            .as_::<Undefined>()
    }
}
impl GPUDevice {
    pub fn pop_error_scope(&self) -> Promise {
        self.inner.call("popErrorScope", &[]).as_::<Promise>()
    }
}
impl GPUDevice {
    pub fn onuncapturederror(&self) -> Any {
        self.inner.get("onuncapturederror").as_::<Any>()
    }

    pub fn set_onuncapturederror(&mut self, value: &Any) {
        self.inner.set("onuncapturederror", value);
    }
}
impl GPUDevice {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
