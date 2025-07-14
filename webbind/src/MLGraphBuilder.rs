use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLArgMinMaxOptions {
    inner: emlite::Val,
}
impl FromVal for MLArgMinMaxOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLArgMinMaxOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLArgMinMaxOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLArgMinMaxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLArgMinMaxOptions> for emlite::Val {
    fn from(s: MLArgMinMaxOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLArgMinMaxOptions {
    pub fn keep_dimensions(&self) -> bool {
        self.inner.get("keepDimensions").as_::<bool>()
    }

    pub fn set_keep_dimensions(&mut self, value: bool) {
        self.inner.set("keepDimensions", value);
    }
}
impl MLArgMinMaxOptions {
    pub fn output_data_type(&self) -> MLOperandDataType {
        self.inner.get("outputDataType").as_::<MLOperandDataType>()
    }

    pub fn set_output_data_type(&mut self, value: MLOperandDataType) {
        self.inner.set("outputDataType", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLBatchNormalizationOptions {
    inner: emlite::Val,
}
impl FromVal for MLBatchNormalizationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLBatchNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLBatchNormalizationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLBatchNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLBatchNormalizationOptions> for emlite::Val {
    fn from(s: MLBatchNormalizationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLBatchNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLBatchNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLBatchNormalizationOptions {
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
impl MLBatchNormalizationOptions {
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLOperatorOptions {
    inner: emlite::Val,
}
impl FromVal for MLOperatorOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLOperatorOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOperatorOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperatorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLOperatorOptions> for emlite::Val {
    fn from(s: MLOperatorOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOperatorOptions {
    pub fn label(&self) -> jsbind::USVString {
        self.inner.get("label").as_::<jsbind::USVString>()
    }

    pub fn set_label(&mut self, value: jsbind::USVString) {
        self.inner.set("label", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLClampOptions {
    inner: emlite::Val,
}
impl FromVal for MLClampOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLClampOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLClampOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLClampOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLClampOptions> for emlite::Val {
    fn from(s: MLClampOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLClampOptions {
    pub fn min_value(&self) -> jsbind::Any {
        self.inner.get("minValue").as_::<jsbind::Any>()
    }

    pub fn set_min_value(&mut self, value: jsbind::Any) {
        self.inner.set("minValue", value);
    }
}
impl MLClampOptions {
    pub fn max_value(&self) -> jsbind::Any {
        self.inner.get("maxValue").as_::<jsbind::Any>()
    }

    pub fn set_max_value(&mut self, value: jsbind::Any) {
        self.inner.set("maxValue", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLConv2dOptions {
    inner: emlite::Val,
}
impl FromVal for MLConv2dOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLConv2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLConv2dOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLConv2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLConv2dOptions> for emlite::Val {
    fn from(s: MLConv2dOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLConv2dOptions {
    pub fn padding(&self) -> jsbind::Sequence<u32> {
        self.inner.get("padding").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLConv2dOptions {
    pub fn strides(&self) -> jsbind::Sequence<u32> {
        self.inner.get("strides").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLConv2dOptions {
    pub fn dilations(&self) -> jsbind::Sequence<u32> {
        self.inner.get("dilations").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLConv2dOptions {
    pub fn groups(&self) -> u32 {
        self.inner.get("groups").as_::<u32>()
    }

    pub fn set_groups(&mut self, value: u32) {
        self.inner.set("groups", value);
    }
}
impl MLConv2dOptions {
    pub fn input_layout(&self) -> MLInputOperandLayout {
        self.inner.get("inputLayout").as_::<MLInputOperandLayout>()
    }

    pub fn set_input_layout(&mut self, value: MLInputOperandLayout) {
        self.inner.set("inputLayout", value);
    }
}
impl MLConv2dOptions {
    pub fn filter_layout(&self) -> MLConv2dFilterOperandLayout {
        self.inner
            .get("filterLayout")
            .as_::<MLConv2dFilterOperandLayout>()
    }

    pub fn set_filter_layout(&mut self, value: MLConv2dFilterOperandLayout) {
        self.inner.set("filterLayout", value);
    }
}
impl MLConv2dOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLConvTranspose2dOptions {
    inner: emlite::Val,
}
impl FromVal for MLConvTranspose2dOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLConvTranspose2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLConvTranspose2dOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLConvTranspose2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLConvTranspose2dOptions> for emlite::Val {
    fn from(s: MLConvTranspose2dOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLConvTranspose2dOptions {
    pub fn padding(&self) -> jsbind::Sequence<u32> {
        self.inner.get("padding").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn strides(&self) -> jsbind::Sequence<u32> {
        self.inner.get("strides").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn dilations(&self) -> jsbind::Sequence<u32> {
        self.inner.get("dilations").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn output_padding(&self) -> jsbind::Sequence<u32> {
        self.inner
            .get("outputPadding")
            .as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_output_padding(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("outputPadding", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn output_sizes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("outputSizes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_output_sizes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("outputSizes", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn groups(&self) -> u32 {
        self.inner.get("groups").as_::<u32>()
    }

    pub fn set_groups(&mut self, value: u32) {
        self.inner.set("groups", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn input_layout(&self) -> MLInputOperandLayout {
        self.inner.get("inputLayout").as_::<MLInputOperandLayout>()
    }

    pub fn set_input_layout(&mut self, value: MLInputOperandLayout) {
        self.inner.set("inputLayout", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn filter_layout(&self) -> MLConvTranspose2dFilterOperandLayout {
        self.inner
            .get("filterLayout")
            .as_::<MLConvTranspose2dFilterOperandLayout>()
    }

    pub fn set_filter_layout(&mut self, value: MLConvTranspose2dFilterOperandLayout) {
        self.inner.set("filterLayout", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLCumulativeSumOptions {
    inner: emlite::Val,
}
impl FromVal for MLCumulativeSumOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLCumulativeSumOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLCumulativeSumOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLCumulativeSumOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLCumulativeSumOptions> for emlite::Val {
    fn from(s: MLCumulativeSumOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLCumulativeSumOptions {
    pub fn exclusive(&self) -> bool {
        self.inner.get("exclusive").as_::<bool>()
    }

    pub fn set_exclusive(&mut self, value: bool) {
        self.inner.set("exclusive", value);
    }
}
impl MLCumulativeSumOptions {
    pub fn reversed(&self) -> bool {
        self.inner.get("reversed").as_::<bool>()
    }

    pub fn set_reversed(&mut self, value: bool) {
        self.inner.set("reversed", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLEluOptions {
    inner: emlite::Val,
}
impl FromVal for MLEluOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLEluOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLEluOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLEluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLEluOptions> for emlite::Val {
    fn from(s: MLEluOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLEluOptions {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLGatherOptions {
    inner: emlite::Val,
}
impl FromVal for MLGatherOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGatherOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGatherOptions> for emlite::Val {
    fn from(s: MLGatherOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGatherOptions {
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLGemmOptions {
    inner: emlite::Val,
}
impl FromVal for MLGemmOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLGemmOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGemmOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGemmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGemmOptions> for emlite::Val {
    fn from(s: MLGemmOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGemmOptions {
    pub fn c(&self) -> MLOperand {
        self.inner.get("c").as_::<MLOperand>()
    }

    pub fn set_c(&mut self, value: MLOperand) {
        self.inner.set("c", value);
    }
}
impl MLGemmOptions {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLGemmOptions {
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
impl MLGemmOptions {
    pub fn a_transpose(&self) -> bool {
        self.inner.get("aTranspose").as_::<bool>()
    }

    pub fn set_a_transpose(&mut self, value: bool) {
        self.inner.set("aTranspose", value);
    }
}
impl MLGemmOptions {
    pub fn b_transpose(&self) -> bool {
        self.inner.get("bTranspose").as_::<bool>()
    }

    pub fn set_b_transpose(&mut self, value: bool) {
        self.inner.set("bTranspose", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLGruOptions {
    inner: emlite::Val,
}
impl FromVal for MLGruOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLGruOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGruOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGruOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGruOptions> for emlite::Val {
    fn from(s: MLGruOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGruOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruOptions {
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    pub fn set_initial_hidden_state(&mut self, value: MLOperand) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLGruOptions {
    pub fn reset_after(&self) -> bool {
        self.inner.get("resetAfter").as_::<bool>()
    }

    pub fn set_reset_after(&mut self, value: bool) {
        self.inner.set("resetAfter", value);
    }
}
impl MLGruOptions {
    pub fn return_sequence(&self) -> bool {
        self.inner.get("returnSequence").as_::<bool>()
    }

    pub fn set_return_sequence(&mut self, value: bool) {
        self.inner.set("returnSequence", value);
    }
}
impl MLGruOptions {
    pub fn direction(&self) -> MLRecurrentNetworkDirection {
        self.inner
            .get("direction")
            .as_::<MLRecurrentNetworkDirection>()
    }

    pub fn set_direction(&mut self, value: MLRecurrentNetworkDirection) {
        self.inner.set("direction", value);
    }
}
impl MLGruOptions {
    pub fn layout(&self) -> MLGruWeightLayout {
        self.inner.get("layout").as_::<MLGruWeightLayout>()
    }

    pub fn set_layout(&mut self, value: MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruOptions {
    pub fn activations(&self) -> jsbind::Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<jsbind::Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: jsbind::Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLGruCellOptions {
    inner: emlite::Val,
}
impl FromVal for MLGruCellOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLGruCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGruCellOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGruCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGruCellOptions> for emlite::Val {
    fn from(s: MLGruCellOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGruCellOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruCellOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruCellOptions {
    pub fn reset_after(&self) -> bool {
        self.inner.get("resetAfter").as_::<bool>()
    }

    pub fn set_reset_after(&mut self, value: bool) {
        self.inner.set("resetAfter", value);
    }
}
impl MLGruCellOptions {
    pub fn layout(&self) -> MLGruWeightLayout {
        self.inner.get("layout").as_::<MLGruWeightLayout>()
    }

    pub fn set_layout(&mut self, value: MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruCellOptions {
    pub fn activations(&self) -> jsbind::Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<jsbind::Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: jsbind::Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLHardSigmoidOptions {
    inner: emlite::Val,
}
impl FromVal for MLHardSigmoidOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLHardSigmoidOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLHardSigmoidOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLHardSigmoidOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLHardSigmoidOptions> for emlite::Val {
    fn from(s: MLHardSigmoidOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLHardSigmoidOptions {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLHardSigmoidOptions {
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLInstanceNormalizationOptions {
    inner: emlite::Val,
}
impl FromVal for MLInstanceNormalizationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLInstanceNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLInstanceNormalizationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLInstanceNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLInstanceNormalizationOptions> for emlite::Val {
    fn from(s: MLInstanceNormalizationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLInstanceNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLInstanceNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLInstanceNormalizationOptions {
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
impl MLInstanceNormalizationOptions {
    pub fn layout(&self) -> MLInputOperandLayout {
        self.inner.get("layout").as_::<MLInputOperandLayout>()
    }

    pub fn set_layout(&mut self, value: MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLLayerNormalizationOptions {
    inner: emlite::Val,
}
impl FromVal for MLLayerNormalizationOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLLayerNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLayerNormalizationOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLayerNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLayerNormalizationOptions> for emlite::Val {
    fn from(s: MLLayerNormalizationOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLayerNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLLayerNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLayerNormalizationOptions {
    pub fn axes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("axes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
impl MLLayerNormalizationOptions {
    pub fn epsilon(&self) -> f64 {
        self.inner.get("epsilon").as_::<f64>()
    }

    pub fn set_epsilon(&mut self, value: f64) {
        self.inner.set("epsilon", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLLeakyReluOptions {
    inner: emlite::Val,
}
impl FromVal for MLLeakyReluOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLLeakyReluOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLeakyReluOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLeakyReluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLeakyReluOptions> for emlite::Val {
    fn from(s: MLLeakyReluOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLeakyReluOptions {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLLinearOptions {
    inner: emlite::Val,
}
impl FromVal for MLLinearOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLLinearOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLinearOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLinearOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLinearOptions> for emlite::Val {
    fn from(s: MLLinearOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLinearOptions {
    pub fn alpha(&self) -> f64 {
        self.inner.get("alpha").as_::<f64>()
    }

    pub fn set_alpha(&mut self, value: f64) {
        self.inner.set("alpha", value);
    }
}
impl MLLinearOptions {
    pub fn beta(&self) -> f64 {
        self.inner.get("beta").as_::<f64>()
    }

    pub fn set_beta(&mut self, value: f64) {
        self.inner.set("beta", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLLstmOptions {
    inner: emlite::Val,
}
impl FromVal for MLLstmOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLLstmOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLstmOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLstmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLstmOptions> for emlite::Val {
    fn from(s: MLLstmOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLstmOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmOptions {
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    pub fn set_peephole_weight(&mut self, value: MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmOptions {
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    pub fn set_initial_hidden_state(&mut self, value: MLOperand) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLLstmOptions {
    pub fn initial_cell_state(&self) -> MLOperand {
        self.inner.get("initialCellState").as_::<MLOperand>()
    }

    pub fn set_initial_cell_state(&mut self, value: MLOperand) {
        self.inner.set("initialCellState", value);
    }
}
impl MLLstmOptions {
    pub fn return_sequence(&self) -> bool {
        self.inner.get("returnSequence").as_::<bool>()
    }

    pub fn set_return_sequence(&mut self, value: bool) {
        self.inner.set("returnSequence", value);
    }
}
impl MLLstmOptions {
    pub fn direction(&self) -> MLRecurrentNetworkDirection {
        self.inner
            .get("direction")
            .as_::<MLRecurrentNetworkDirection>()
    }

    pub fn set_direction(&mut self, value: MLRecurrentNetworkDirection) {
        self.inner.set("direction", value);
    }
}
impl MLLstmOptions {
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    pub fn set_layout(&mut self, value: MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmOptions {
    pub fn activations(&self) -> jsbind::Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<jsbind::Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: jsbind::Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLLstmCellOptions {
    inner: emlite::Val,
}
impl FromVal for MLLstmCellOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLLstmCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLstmCellOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLstmCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLLstmCellOptions> for emlite::Val {
    fn from(s: MLLstmCellOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLLstmCellOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmCellOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmCellOptions {
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    pub fn set_peephole_weight(&mut self, value: MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmCellOptions {
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    pub fn set_layout(&mut self, value: MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmCellOptions {
    pub fn activations(&self) -> jsbind::Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<jsbind::Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: jsbind::Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLPadOptions {
    inner: emlite::Val,
}
impl FromVal for MLPadOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLPadOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLPadOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLPadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLPadOptions> for emlite::Val {
    fn from(s: MLPadOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLPadOptions {
    pub fn mode(&self) -> MLPaddingMode {
        self.inner.get("mode").as_::<MLPaddingMode>()
    }

    pub fn set_mode(&mut self, value: MLPaddingMode) {
        self.inner.set("mode", value);
    }
}
impl MLPadOptions {
    pub fn value(&self) -> jsbind::Any {
        self.inner.get("value").as_::<jsbind::Any>()
    }

    pub fn set_value(&mut self, value: jsbind::Any) {
        self.inner.set("value", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLPool2dOptions {
    inner: emlite::Val,
}
impl FromVal for MLPool2dOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLPool2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLPool2dOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLPool2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLPool2dOptions> for emlite::Val {
    fn from(s: MLPool2dOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLPool2dOptions {
    pub fn window_dimensions(&self) -> jsbind::Sequence<u32> {
        self.inner
            .get("windowDimensions")
            .as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_window_dimensions(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("windowDimensions", value);
    }
}
impl MLPool2dOptions {
    pub fn padding(&self) -> jsbind::Sequence<u32> {
        self.inner.get("padding").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLPool2dOptions {
    pub fn strides(&self) -> jsbind::Sequence<u32> {
        self.inner.get("strides").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLPool2dOptions {
    pub fn dilations(&self) -> jsbind::Sequence<u32> {
        self.inner.get("dilations").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLPool2dOptions {
    pub fn layout(&self) -> MLInputOperandLayout {
        self.inner.get("layout").as_::<MLInputOperandLayout>()
    }

    pub fn set_layout(&mut self, value: MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
impl MLPool2dOptions {
    pub fn rounding_type(&self) -> MLRoundingType {
        self.inner.get("roundingType").as_::<MLRoundingType>()
    }

    pub fn set_rounding_type(&mut self, value: MLRoundingType) {
        self.inner.set("roundingType", value);
    }
}
impl MLPool2dOptions {
    pub fn output_sizes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("outputSizes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_output_sizes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("outputSizes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLReduceOptions {
    inner: emlite::Val,
}
impl FromVal for MLReduceOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLReduceOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLReduceOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLReduceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLReduceOptions> for emlite::Val {
    fn from(s: MLReduceOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLReduceOptions {
    pub fn axes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("axes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
impl MLReduceOptions {
    pub fn keep_dimensions(&self) -> bool {
        self.inner.get("keepDimensions").as_::<bool>()
    }

    pub fn set_keep_dimensions(&mut self, value: bool) {
        self.inner.set("keepDimensions", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLResample2dOptions {
    inner: emlite::Val,
}
impl FromVal for MLResample2dOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLResample2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLResample2dOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLResample2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLResample2dOptions> for emlite::Val {
    fn from(s: MLResample2dOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLResample2dOptions {
    pub fn mode(&self) -> MLInterpolationMode {
        self.inner.get("mode").as_::<MLInterpolationMode>()
    }

    pub fn set_mode(&mut self, value: MLInterpolationMode) {
        self.inner.set("mode", value);
    }
}
impl MLResample2dOptions {
    pub fn scales(&self) -> jsbind::Sequence<f32> {
        self.inner.get("scales").as_::<jsbind::Sequence<f32>>()
    }

    pub fn set_scales(&mut self, value: jsbind::Sequence<f32>) {
        self.inner.set("scales", value);
    }
}
impl MLResample2dOptions {
    pub fn sizes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("sizes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_sizes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("sizes", value);
    }
}
impl MLResample2dOptions {
    pub fn axes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("axes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLReverseOptions {
    inner: emlite::Val,
}
impl FromVal for MLReverseOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLReverseOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLReverseOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLReverseOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLReverseOptions> for emlite::Val {
    fn from(s: MLReverseOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLReverseOptions {
    pub fn axes(&self) -> jsbind::Sequence<u32> {
        self.inner.get("axes").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLScatterOptions {
    inner: emlite::Val,
}
impl FromVal for MLScatterOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLScatterOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLScatterOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLScatterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLScatterOptions> for emlite::Val {
    fn from(s: MLScatterOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLScatterOptions {
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLSliceOptions {
    inner: emlite::Val,
}
impl FromVal for MLSliceOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLSliceOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLSliceOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLSliceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLSliceOptions> for emlite::Val {
    fn from(s: MLSliceOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLSliceOptions {
    pub fn strides(&self) -> jsbind::Sequence<u32> {
        self.inner.get("strides").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLSplitOptions {
    inner: emlite::Val,
}
impl FromVal for MLSplitOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLSplitOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLSplitOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLSplitOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLSplitOptions> for emlite::Val {
    fn from(s: MLSplitOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLSplitOptions {
    pub fn axis(&self) -> u32 {
        self.inner.get("axis").as_::<u32>()
    }

    pub fn set_axis(&mut self, value: u32) {
        self.inner.set("axis", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLTransposeOptions {
    inner: emlite::Val,
}
impl FromVal for MLTransposeOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLTransposeOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTransposeOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTransposeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLTransposeOptions> for emlite::Val {
    fn from(s: MLTransposeOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLTransposeOptions {
    pub fn permutation(&self) -> jsbind::Sequence<u32> {
        self.inner.get("permutation").as_::<jsbind::Sequence<u32>>()
    }

    pub fn set_permutation(&mut self, value: jsbind::Sequence<u32>) {
        self.inner.set("permutation", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLTriangularOptions {
    inner: emlite::Val,
}
impl FromVal for MLTriangularOptions {
    fn from_val(v: &emlite::Val) -> Self {
        MLTriangularOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTriangularOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTriangularOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLTriangularOptions> for emlite::Val {
    fn from(s: MLTriangularOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLTriangularOptions {
    pub fn upper(&self) -> bool {
        self.inner.get("upper").as_::<bool>()
    }

    pub fn set_upper(&mut self, value: bool) {
        self.inner.set("upper", value);
    }
}
impl MLTriangularOptions {
    pub fn diagonal(&self) -> i32 {
        self.inner.get("diagonal").as_::<i32>()
    }

    pub fn set_diagonal(&mut self, value: i32) {
        self.inner.set("diagonal", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLGraphBuilder {
    inner: emlite::Val,
}
impl FromVal for MLGraphBuilder {
    fn from_val(v: &emlite::Val) -> Self {
        MLGraphBuilder {
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
impl core::ops::Deref for MLGraphBuilder {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGraphBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLGraphBuilder> for emlite::Val {
    fn from(s: MLGraphBuilder) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLGraphBuilder {
    pub fn new(context: MLContext) -> MLGraphBuilder {
        Self {
            inner: emlite::Val::global("MLGraphBuilder")
                .new(&[context.into()])
                .as_::<emlite::Val>(),
        }
    }
}
impl MLGraphBuilder {
    pub fn input(&self, name: jsbind::USVString, descriptor: MLOperandDescriptor) -> MLOperand {
        self.inner
            .call("input", &[name.into(), descriptor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn constant(&self, tensor: MLTensor) -> MLOperand {
        self.inner
            .call("constant", &[tensor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn build(&self, outputs: jsbind::Any) -> jsbind::Promise {
        self.inner
            .call("build", &[outputs.into()])
            .as_::<jsbind::Promise>()
    }
}
impl MLGraphBuilder {
    pub fn arg_min0(&self, input: MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMin", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }

    pub fn arg_min1(&self, input: MLOperand, axis: u32, options: MLArgMinMaxOptions) -> MLOperand {
        self.inner
            .call("argMin", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn arg_max0(&self, input: MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }

    pub fn arg_max1(&self, input: MLOperand, axis: u32, options: MLArgMinMaxOptions) -> MLOperand {
        self.inner
            .call("argMax", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn batch_normalization0(
        &self,
        input: MLOperand,
        mean: MLOperand,
        variance: MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "batchNormalization",
                &[input.into(), mean.into(), variance.into()],
            )
            .as_::<MLOperand>()
    }

    pub fn batch_normalization1(
        &self,
        input: MLOperand,
        mean: MLOperand,
        variance: MLOperand,
        options: MLBatchNormalizationOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "batchNormalization",
                &[input.into(), mean.into(), variance.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn cast0(&self, input: MLOperand, type_: MLOperandDataType) -> MLOperand {
        self.inner
            .call("cast", &[input.into(), type_.into()])
            .as_::<MLOperand>()
    }

    pub fn cast1(
        &self,
        input: MLOperand,
        type_: MLOperandDataType,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("cast", &[input.into(), type_.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn clamp0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("clamp", &[input.into()]).as_::<MLOperand>()
    }

    pub fn clamp1(&self, input: MLOperand, options: MLClampOptions) -> MLOperand {
        self.inner
            .call("clamp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn concat0(&self, inputs: jsbind::Sequence<MLOperand>, axis: u32) -> MLOperand {
        self.inner
            .call("concat", &[inputs.into(), axis.into()])
            .as_::<MLOperand>()
    }

    pub fn concat1(
        &self,
        inputs: jsbind::Sequence<MLOperand>,
        axis: u32,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("concat", &[inputs.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn conv2d0(&self, input: MLOperand, filter: MLOperand) -> MLOperand {
        self.inner
            .call("conv2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }

    pub fn conv2d1(
        &self,
        input: MLOperand,
        filter: MLOperand,
        options: MLConv2dOptions,
    ) -> MLOperand {
        self.inner
            .call("conv2d", &[input.into(), filter.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn conv_transpose2d0(&self, input: MLOperand, filter: MLOperand) -> MLOperand {
        self.inner
            .call("convTranspose2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }

    pub fn conv_transpose2d1(
        &self,
        input: MLOperand,
        filter: MLOperand,
        options: MLConvTranspose2dOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "convTranspose2d",
                &[input.into(), filter.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn cumulative_sum0(&self, input: MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("cumulativeSum", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }

    pub fn cumulative_sum1(
        &self,
        input: MLOperand,
        axis: u32,
        options: MLCumulativeSumOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "cumulativeSum",
                &[input.into(), axis.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn add0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn add1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn sub0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn sub1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn mul0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn mul1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn div0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn div1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn max0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn max1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn min0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn min1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn pow0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn pow1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn equal0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn equal1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn not_equal0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("notEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn not_equal1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("notEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn greater0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn greater1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn greater_or_equal0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("greaterOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn greater_or_equal1(
        &self,
        a: MLOperand,
        b: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("greaterOrEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn lesser0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn lesser1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn lesser_or_equal0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("lesserOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn lesser_or_equal1(
        &self,
        a: MLOperand,
        b: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("lesserOrEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn logical_not0(&self, a: MLOperand) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into()])
            .as_::<MLOperand>()
    }

    pub fn logical_not1(&self, a: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn logical_and0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("logicalAnd", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn logical_and1(
        &self,
        a: MLOperand,
        b: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalAnd", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn logical_or0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("logicalOr", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn logical_or1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("logicalOr", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn logical_xor0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("logicalXor", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn logical_xor1(
        &self,
        a: MLOperand,
        b: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalXor", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn abs0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("abs", &[input.into()]).as_::<MLOperand>()
    }

    pub fn abs1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("abs", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn ceil0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("ceil", &[input.into()]).as_::<MLOperand>()
    }

    pub fn ceil1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("ceil", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn cos0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("cos", &[input.into()]).as_::<MLOperand>()
    }

    pub fn cos1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("cos", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn erf0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("erf", &[input.into()]).as_::<MLOperand>()
    }

    pub fn erf1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("erf", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn exp0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("exp", &[input.into()]).as_::<MLOperand>()
    }

    pub fn exp1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("exp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn floor0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("floor", &[input.into()]).as_::<MLOperand>()
    }

    pub fn floor1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("floor", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn identity0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("identity", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn identity1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("identity", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn log0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("log", &[input.into()]).as_::<MLOperand>()
    }

    pub fn log1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("log", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn neg0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("neg", &[input.into()]).as_::<MLOperand>()
    }

    pub fn neg1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("neg", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reciprocal0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reciprocal1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn sin0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("sin", &[input.into()]).as_::<MLOperand>()
    }

    pub fn sin1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn sign0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("sign", &[input.into()]).as_::<MLOperand>()
    }

    pub fn sign1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn sqrt0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("sqrt", &[input.into()]).as_::<MLOperand>()
    }

    pub fn sqrt1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sqrt", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn tan0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("tan", &[input.into()]).as_::<MLOperand>()
    }

    pub fn tan1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tan", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn dequantize_linear0(
        &self,
        input: MLOperand,
        scale: MLOperand,
        zero_point: MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "dequantizeLinear",
                &[input.into(), scale.into(), zero_point.into()],
            )
            .as_::<MLOperand>()
    }

    pub fn dequantize_linear1(
        &self,
        input: MLOperand,
        scale: MLOperand,
        zero_point: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "dequantizeLinear",
                &[
                    input.into(),
                    scale.into(),
                    zero_point.into(),
                    options.into(),
                ],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn quantize_linear0(
        &self,
        input: MLOperand,
        scale: MLOperand,
        zero_point: MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "quantizeLinear",
                &[input.into(), scale.into(), zero_point.into()],
            )
            .as_::<MLOperand>()
    }

    pub fn quantize_linear1(
        &self,
        input: MLOperand,
        scale: MLOperand,
        zero_point: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "quantizeLinear",
                &[
                    input.into(),
                    scale.into(),
                    zero_point.into(),
                    options.into(),
                ],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn elu0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("elu", &[input.into()]).as_::<MLOperand>()
    }

    pub fn elu1(&self, input: MLOperand, options: MLEluOptions) -> MLOperand {
        self.inner
            .call("elu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn expand0(&self, input: MLOperand, new_shape: jsbind::Sequence<u32>) -> MLOperand {
        self.inner
            .call("expand", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }

    pub fn expand1(
        &self,
        input: MLOperand,
        new_shape: jsbind::Sequence<u32>,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("expand", &[input.into(), new_shape.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gather0(&self, input: MLOperand, indices: MLOperand) -> MLOperand {
        self.inner
            .call("gather", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }

    pub fn gather1(
        &self,
        input: MLOperand,
        indices: MLOperand,
        options: MLGatherOptions,
    ) -> MLOperand {
        self.inner
            .call("gather", &[input.into(), indices.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gather_elements0(&self, input: MLOperand, indices: MLOperand) -> MLOperand {
        self.inner
            .call("gatherElements", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }

    pub fn gather_elements1(
        &self,
        input: MLOperand,
        indices: MLOperand,
        options: MLGatherOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "gatherElements",
                &[input.into(), indices.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gather_nd0(&self, input: MLOperand, indices: MLOperand) -> MLOperand {
        self.inner
            .call("gatherND", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }

    pub fn gather_nd1(
        &self,
        input: MLOperand,
        indices: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("gatherND", &[input.into(), indices.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gelu0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("gelu", &[input.into()]).as_::<MLOperand>()
    }

    pub fn gelu1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("gelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gemm0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn gemm1(&self, a: MLOperand, b: MLOperand, options: MLGemmOptions) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn gru0(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "gru",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    steps.into(),
                    hidden_size.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }

    pub fn gru1(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        steps: u32,
        hidden_size: u32,
        options: MLGruOptions,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "gru",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    steps.into(),
                    hidden_size.into(),
                    options.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    pub fn gru_cell0(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        hidden_state: MLOperand,
        hidden_size: u32,
    ) -> MLOperand {
        self.inner
            .call(
                "gruCell",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    hidden_state.into(),
                    hidden_size.into(),
                ],
            )
            .as_::<MLOperand>()
    }

    pub fn gru_cell1(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        hidden_state: MLOperand,
        hidden_size: u32,
        options: MLGruCellOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "gruCell",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    hidden_state.into(),
                    hidden_size.into(),
                    options.into(),
                ],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn hard_sigmoid0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn hard_sigmoid1(&self, input: MLOperand, options: MLHardSigmoidOptions) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn hard_swish0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn hard_swish1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn instance_normalization0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("instanceNormalization", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn instance_normalization1(
        &self,
        input: MLOperand,
        options: MLInstanceNormalizationOptions,
    ) -> MLOperand {
        self.inner
            .call("instanceNormalization", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn layer_normalization0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("layerNormalization", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn layer_normalization1(
        &self,
        input: MLOperand,
        options: MLLayerNormalizationOptions,
    ) -> MLOperand {
        self.inner
            .call("layerNormalization", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn leaky_relu0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn leaky_relu1(&self, input: MLOperand, options: MLLeakyReluOptions) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn linear0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("linear", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn linear1(&self, input: MLOperand, options: MLLinearOptions) -> MLOperand {
        self.inner
            .call("linear", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn lstm0(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "lstm",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    steps.into(),
                    hidden_size.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }

    pub fn lstm1(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        steps: u32,
        hidden_size: u32,
        options: MLLstmOptions,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "lstm",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    steps.into(),
                    hidden_size.into(),
                    options.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    pub fn lstm_cell0(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        hidden_state: MLOperand,
        cell_state: MLOperand,
        hidden_size: u32,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "lstmCell",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    hidden_state.into(),
                    cell_state.into(),
                    hidden_size.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }

    pub fn lstm_cell1(
        &self,
        input: MLOperand,
        weight: MLOperand,
        recurrent_weight: MLOperand,
        hidden_state: MLOperand,
        cell_state: MLOperand,
        hidden_size: u32,
        options: MLLstmCellOptions,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call(
                "lstmCell",
                &[
                    input.into(),
                    weight.into(),
                    recurrent_weight.into(),
                    hidden_state.into(),
                    cell_state.into(),
                    hidden_size.into(),
                    options.into(),
                ],
            )
            .as_::<jsbind::Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    pub fn matmul0(&self, a: MLOperand, b: MLOperand) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }

    pub fn matmul1(&self, a: MLOperand, b: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn pad0(
        &self,
        input: MLOperand,
        beginning_padding: jsbind::Sequence<u32>,
        ending_padding: jsbind::Sequence<u32>,
    ) -> MLOperand {
        self.inner
            .call(
                "pad",
                &[
                    input.into(),
                    beginning_padding.into(),
                    ending_padding.into(),
                ],
            )
            .as_::<MLOperand>()
    }

    pub fn pad1(
        &self,
        input: MLOperand,
        beginning_padding: jsbind::Sequence<u32>,
        ending_padding: jsbind::Sequence<u32>,
        options: MLPadOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "pad",
                &[
                    input.into(),
                    beginning_padding.into(),
                    ending_padding.into(),
                    options.into(),
                ],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn average_pool2d0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn average_pool2d1(&self, input: MLOperand, options: MLPool2dOptions) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn l2_pool2d0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn l2_pool2d1(&self, input: MLOperand, options: MLPool2dOptions) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn max_pool2d0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn max_pool2d1(&self, input: MLOperand, options: MLPool2dOptions) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn prelu0(&self, input: MLOperand, slope: MLOperand) -> MLOperand {
        self.inner
            .call("prelu", &[input.into(), slope.into()])
            .as_::<MLOperand>()
    }

    pub fn prelu1(
        &self,
        input: MLOperand,
        slope: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("prelu", &[input.into(), slope.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_l10(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_l11(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_l20(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_l21(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_log_sum0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_log_sum1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_log_sum_exp0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_log_sum_exp1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_max0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_max1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_mean0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_mean1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_min0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_min1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_product0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_product1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_sum0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_sum1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reduce_sum_square0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reduce_sum_square1(&self, input: MLOperand, options: MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn relu0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("relu", &[input.into()]).as_::<MLOperand>()
    }

    pub fn relu1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("relu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn resample2d0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn resample2d1(&self, input: MLOperand, options: MLResample2dOptions) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reshape0(&self, input: MLOperand, new_shape: jsbind::Sequence<u32>) -> MLOperand {
        self.inner
            .call("reshape", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }

    pub fn reshape1(
        &self,
        input: MLOperand,
        new_shape: jsbind::Sequence<u32>,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("reshape", &[input.into(), new_shape.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn reverse0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("reverse", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn reverse1(&self, input: MLOperand, options: MLReverseOptions) -> MLOperand {
        self.inner
            .call("reverse", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn scatter_elements0(
        &self,
        input: MLOperand,
        indices: MLOperand,
        updates: MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "scatterElements",
                &[input.into(), indices.into(), updates.into()],
            )
            .as_::<MLOperand>()
    }

    pub fn scatter_elements1(
        &self,
        input: MLOperand,
        indices: MLOperand,
        updates: MLOperand,
        options: MLScatterOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "scatterElements",
                &[input.into(), indices.into(), updates.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn scatter_nd0(
        &self,
        input: MLOperand,
        indices: MLOperand,
        updates: MLOperand,
    ) -> MLOperand {
        self.inner
            .call("scatterND", &[input.into(), indices.into(), updates.into()])
            .as_::<MLOperand>()
    }

    pub fn scatter_nd1(
        &self,
        input: MLOperand,
        indices: MLOperand,
        updates: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "scatterND",
                &[input.into(), indices.into(), updates.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn sigmoid0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn sigmoid1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn slice0(
        &self,
        input: MLOperand,
        starts: jsbind::Sequence<u32>,
        sizes: jsbind::Sequence<u32>,
    ) -> MLOperand {
        self.inner
            .call("slice", &[input.into(), starts.into(), sizes.into()])
            .as_::<MLOperand>()
    }

    pub fn slice1(
        &self,
        input: MLOperand,
        starts: jsbind::Sequence<u32>,
        sizes: jsbind::Sequence<u32>,
        options: MLSliceOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "slice",
                &[input.into(), starts.into(), sizes.into(), options.into()],
            )
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn softmax0(&self, input: MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }

    pub fn softmax1(&self, input: MLOperand, axis: u32, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn softplus0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("softplus", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn softplus1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softplus", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn softsign0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("softsign", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn softsign1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softsign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn split0(&self, input: MLOperand, splits: jsbind::Any) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into()])
            .as_::<jsbind::Sequence<MLOperand>>()
    }

    pub fn split1(
        &self,
        input: MLOperand,
        splits: jsbind::Any,
        options: MLSplitOptions,
    ) -> jsbind::Sequence<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into(), options.into()])
            .as_::<jsbind::Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    pub fn tanh0(&self, input: MLOperand) -> MLOperand {
        self.inner.call("tanh", &[input.into()]).as_::<MLOperand>()
    }

    pub fn tanh1(&self, input: MLOperand, options: MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tanh", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn tile0(&self, input: MLOperand, repetitions: jsbind::Sequence<u32>) -> MLOperand {
        self.inner
            .call("tile", &[input.into(), repetitions.into()])
            .as_::<MLOperand>()
    }

    pub fn tile1(
        &self,
        input: MLOperand,
        repetitions: jsbind::Sequence<u32>,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("tile", &[input.into(), repetitions.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn transpose0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("transpose", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn transpose1(&self, input: MLOperand, options: MLTransposeOptions) -> MLOperand {
        self.inner
            .call("transpose", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn triangular0(&self, input: MLOperand) -> MLOperand {
        self.inner
            .call("triangular", &[input.into()])
            .as_::<MLOperand>()
    }

    pub fn triangular1(&self, input: MLOperand, options: MLTriangularOptions) -> MLOperand {
        self.inner
            .call("triangular", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    pub fn where_0(
        &self,
        condition: MLOperand,
        true_value: MLOperand,
        false_value: MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "where",
                &[condition.into(), true_value.into(), false_value.into()],
            )
            .as_::<MLOperand>()
    }

    pub fn where_1(
        &self,
        condition: MLOperand,
        true_value: MLOperand,
        false_value: MLOperand,
        options: MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call(
                "where",
                &[
                    condition.into(),
                    true_value.into(),
                    false_value.into(),
                    options.into(),
                ],
            )
            .as_::<MLOperand>()
    }
}
