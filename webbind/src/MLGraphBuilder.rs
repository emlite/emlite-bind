use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLArgMinMaxOptions {
    inner: Any,
}
impl FromVal for MLArgMinMaxOptions {
    fn from_val(v: &Any) -> Self {
        MLArgMinMaxOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLArgMinMaxOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLArgMinMaxOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLArgMinMaxOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLArgMinMaxOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLArgMinMaxOptions> for Any {
    fn from(s: MLArgMinMaxOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLArgMinMaxOptions> for Any {
    fn from(s: &MLArgMinMaxOptions) -> Any {
        s.inner.clone()
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

    pub fn set_output_data_type(&mut self, value: &MLOperandDataType) {
        self.inner.set("outputDataType", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLBatchNormalizationOptions {
    inner: Any,
}
impl FromVal for MLBatchNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLBatchNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLBatchNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLBatchNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLBatchNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLBatchNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLBatchNormalizationOptions> for Any {
    fn from(s: MLBatchNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLBatchNormalizationOptions> for Any {
    fn from(s: &MLBatchNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLBatchNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLBatchNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
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
#[repr(transparent)]
pub struct MLOperatorOptions {
    inner: Any,
}
impl FromVal for MLOperatorOptions {
    fn from_val(v: &Any) -> Self {
        MLOperatorOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLOperatorOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperatorOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLOperatorOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLOperatorOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLOperatorOptions> for Any {
    fn from(s: MLOperatorOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLOperatorOptions> for Any {
    fn from(s: &MLOperatorOptions) -> Any {
        s.inner.clone()
    }
}

impl MLOperatorOptions {
    pub fn label(&self) -> String {
        self.inner.get("label").as_::<String>()
    }

    pub fn set_label(&mut self, value: &str) {
        self.inner.set("label", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLClampOptions {
    inner: Any,
}
impl FromVal for MLClampOptions {
    fn from_val(v: &Any) -> Self {
        MLClampOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLClampOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLClampOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLClampOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLClampOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLClampOptions> for Any {
    fn from(s: MLClampOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLClampOptions> for Any {
    fn from(s: &MLClampOptions) -> Any {
        s.inner.clone()
    }
}

impl MLClampOptions {
    pub fn min_value(&self) -> Any {
        self.inner.get("minValue").as_::<Any>()
    }

    pub fn set_min_value(&mut self, value: &Any) {
        self.inner.set("minValue", value);
    }
}
impl MLClampOptions {
    pub fn max_value(&self) -> Any {
        self.inner.get("maxValue").as_::<Any>()
    }

    pub fn set_max_value(&mut self, value: &Any) {
        self.inner.set("maxValue", value);
    }
}
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
    pub fn padding(&self) -> Sequence<u32> {
        self.inner.get("padding").as_::<Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLConv2dOptions {
    pub fn strides(&self) -> Sequence<u32> {
        self.inner.get("strides").as_::<Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLConv2dOptions {
    pub fn dilations(&self) -> Sequence<u32> {
        self.inner.get("dilations").as_::<Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: Sequence<u32>) {
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

    pub fn set_input_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("inputLayout", value);
    }
}
impl MLConv2dOptions {
    pub fn filter_layout(&self) -> MLConv2dFilterOperandLayout {
        self.inner
            .get("filterLayout")
            .as_::<MLConv2dFilterOperandLayout>()
    }

    pub fn set_filter_layout(&mut self, value: &MLConv2dFilterOperandLayout) {
        self.inner.set("filterLayout", value);
    }
}
impl MLConv2dOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLConvTranspose2dOptions {
    inner: Any,
}
impl FromVal for MLConvTranspose2dOptions {
    fn from_val(v: &Any) -> Self {
        MLConvTranspose2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLConvTranspose2dOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLConvTranspose2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLConvTranspose2dOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLConvTranspose2dOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLConvTranspose2dOptions> for Any {
    fn from(s: MLConvTranspose2dOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLConvTranspose2dOptions> for Any {
    fn from(s: &MLConvTranspose2dOptions) -> Any {
        s.inner.clone()
    }
}

impl MLConvTranspose2dOptions {
    pub fn padding(&self) -> Sequence<u32> {
        self.inner.get("padding").as_::<Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn strides(&self) -> Sequence<u32> {
        self.inner.get("strides").as_::<Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn dilations(&self) -> Sequence<u32> {
        self.inner.get("dilations").as_::<Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: Sequence<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn output_padding(&self) -> Sequence<u32> {
        self.inner.get("outputPadding").as_::<Sequence<u32>>()
    }

    pub fn set_output_padding(&mut self, value: Sequence<u32>) {
        self.inner.set("outputPadding", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn output_sizes(&self) -> Sequence<u32> {
        self.inner.get("outputSizes").as_::<Sequence<u32>>()
    }

    pub fn set_output_sizes(&mut self, value: Sequence<u32>) {
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

    pub fn set_input_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("inputLayout", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn filter_layout(&self) -> MLConvTranspose2dFilterOperandLayout {
        self.inner
            .get("filterLayout")
            .as_::<MLConvTranspose2dFilterOperandLayout>()
    }

    pub fn set_filter_layout(&mut self, value: &MLConvTranspose2dFilterOperandLayout) {
        self.inner.set("filterLayout", value);
    }
}
impl MLConvTranspose2dOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLCumulativeSumOptions {
    inner: Any,
}
impl FromVal for MLCumulativeSumOptions {
    fn from_val(v: &Any) -> Self {
        MLCumulativeSumOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLCumulativeSumOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLCumulativeSumOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLCumulativeSumOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLCumulativeSumOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLCumulativeSumOptions> for Any {
    fn from(s: MLCumulativeSumOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLCumulativeSumOptions> for Any {
    fn from(s: &MLCumulativeSumOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLEluOptions {
    inner: Any,
}
impl FromVal for MLEluOptions {
    fn from_val(v: &Any) -> Self {
        MLEluOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLEluOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLEluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLEluOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLEluOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLEluOptions> for Any {
    fn from(s: MLEluOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLEluOptions> for Any {
    fn from(s: &MLEluOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLGatherOptions {
    inner: Any,
}
impl FromVal for MLGatherOptions {
    fn from_val(v: &Any) -> Self {
        MLGatherOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGatherOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGatherOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGatherOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGatherOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGatherOptions> for Any {
    fn from(s: MLGatherOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGatherOptions> for Any {
    fn from(s: &MLGatherOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLGemmOptions {
    inner: Any,
}
impl FromVal for MLGemmOptions {
    fn from_val(v: &Any) -> Self {
        MLGemmOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGemmOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGemmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGemmOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGemmOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGemmOptions> for Any {
    fn from(s: MLGemmOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGemmOptions> for Any {
    fn from(s: &MLGemmOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGemmOptions {
    pub fn c(&self) -> MLOperand {
        self.inner.get("c").as_::<MLOperand>()
    }

    pub fn set_c(&mut self, value: &MLOperand) {
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
#[repr(transparent)]
pub struct MLGruOptions {
    inner: Any,
}
impl FromVal for MLGruOptions {
    fn from_val(v: &Any) -> Self {
        MLGruOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGruOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGruOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGruOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGruOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGruOptions> for Any {
    fn from(s: MLGruOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGruOptions> for Any {
    fn from(s: &MLGruOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGruOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLGruOptions {
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    pub fn set_initial_hidden_state(&mut self, value: &MLOperand) {
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

    pub fn set_direction(&mut self, value: &MLRecurrentNetworkDirection) {
        self.inner.set("direction", value);
    }
}
impl MLGruOptions {
    pub fn layout(&self) -> MLGruWeightLayout {
        self.inner.get("layout").as_::<MLGruWeightLayout>()
    }

    pub fn set_layout(&mut self, value: &MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruOptions {
    pub fn activations(&self) -> Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: &Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGruCellOptions {
    inner: Any,
}
impl FromVal for MLGruCellOptions {
    fn from_val(v: &Any) -> Self {
        MLGruCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLGruCellOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGruCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGruCellOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGruCellOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGruCellOptions> for Any {
    fn from(s: MLGruCellOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGruCellOptions> for Any {
    fn from(s: &MLGruCellOptions) -> Any {
        s.inner.clone()
    }
}

impl MLGruCellOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLGruCellOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
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

    pub fn set_layout(&mut self, value: &MLGruWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLGruCellOptions {
    pub fn activations(&self) -> Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: &Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLHardSigmoidOptions {
    inner: Any,
}
impl FromVal for MLHardSigmoidOptions {
    fn from_val(v: &Any) -> Self {
        MLHardSigmoidOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLHardSigmoidOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLHardSigmoidOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLHardSigmoidOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLHardSigmoidOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLHardSigmoidOptions> for Any {
    fn from(s: MLHardSigmoidOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLHardSigmoidOptions> for Any {
    fn from(s: &MLHardSigmoidOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLInstanceNormalizationOptions {
    inner: Any,
}
impl FromVal for MLInstanceNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLInstanceNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLInstanceNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLInstanceNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLInstanceNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLInstanceNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLInstanceNormalizationOptions> for Any {
    fn from(s: MLInstanceNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLInstanceNormalizationOptions> for Any {
    fn from(s: &MLInstanceNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLInstanceNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLInstanceNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
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

    pub fn set_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLayerNormalizationOptions {
    inner: Any,
}
impl FromVal for MLLayerNormalizationOptions {
    fn from_val(v: &Any) -> Self {
        MLLayerNormalizationOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLayerNormalizationOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLayerNormalizationOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLLayerNormalizationOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLLayerNormalizationOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLLayerNormalizationOptions> for Any {
    fn from(s: MLLayerNormalizationOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLLayerNormalizationOptions> for Any {
    fn from(s: &MLLayerNormalizationOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLayerNormalizationOptions {
    pub fn scale(&self) -> MLOperand {
        self.inner.get("scale").as_::<MLOperand>()
    }

    pub fn set_scale(&mut self, value: &MLOperand) {
        self.inner.set("scale", value);
    }
}
impl MLLayerNormalizationOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLayerNormalizationOptions {
    pub fn axes(&self) -> Sequence<u32> {
        self.inner.get("axes").as_::<Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: Sequence<u32>) {
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
#[repr(transparent)]
pub struct MLLeakyReluOptions {
    inner: Any,
}
impl FromVal for MLLeakyReluOptions {
    fn from_val(v: &Any) -> Self {
        MLLeakyReluOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLeakyReluOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLeakyReluOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLLeakyReluOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLLeakyReluOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLLeakyReluOptions> for Any {
    fn from(s: MLLeakyReluOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLLeakyReluOptions> for Any {
    fn from(s: &MLLeakyReluOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLLinearOptions {
    inner: Any,
}
impl FromVal for MLLinearOptions {
    fn from_val(v: &Any) -> Self {
        MLLinearOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLinearOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLinearOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLLinearOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLLinearOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLLinearOptions> for Any {
    fn from(s: MLLinearOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLLinearOptions> for Any {
    fn from(s: &MLLinearOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLLstmOptions {
    inner: Any,
}
impl FromVal for MLLstmOptions {
    fn from_val(v: &Any) -> Self {
        MLLstmOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLstmOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLstmOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLLstmOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLLstmOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLLstmOptions> for Any {
    fn from(s: MLLstmOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLLstmOptions> for Any {
    fn from(s: &MLLstmOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLstmOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmOptions {
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    pub fn set_peephole_weight(&mut self, value: &MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmOptions {
    pub fn initial_hidden_state(&self) -> MLOperand {
        self.inner.get("initialHiddenState").as_::<MLOperand>()
    }

    pub fn set_initial_hidden_state(&mut self, value: &MLOperand) {
        self.inner.set("initialHiddenState", value);
    }
}
impl MLLstmOptions {
    pub fn initial_cell_state(&self) -> MLOperand {
        self.inner.get("initialCellState").as_::<MLOperand>()
    }

    pub fn set_initial_cell_state(&mut self, value: &MLOperand) {
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

    pub fn set_direction(&mut self, value: &MLRecurrentNetworkDirection) {
        self.inner.set("direction", value);
    }
}
impl MLLstmOptions {
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    pub fn set_layout(&mut self, value: &MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmOptions {
    pub fn activations(&self) -> Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: &Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLLstmCellOptions {
    inner: Any,
}
impl FromVal for MLLstmCellOptions {
    fn from_val(v: &Any) -> Self {
        MLLstmCellOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLLstmCellOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLLstmCellOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLLstmCellOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLLstmCellOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLLstmCellOptions> for Any {
    fn from(s: MLLstmCellOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLLstmCellOptions> for Any {
    fn from(s: &MLLstmCellOptions) -> Any {
        s.inner.clone()
    }
}

impl MLLstmCellOptions {
    pub fn bias(&self) -> MLOperand {
        self.inner.get("bias").as_::<MLOperand>()
    }

    pub fn set_bias(&mut self, value: &MLOperand) {
        self.inner.set("bias", value);
    }
}
impl MLLstmCellOptions {
    pub fn recurrent_bias(&self) -> MLOperand {
        self.inner.get("recurrentBias").as_::<MLOperand>()
    }

    pub fn set_recurrent_bias(&mut self, value: &MLOperand) {
        self.inner.set("recurrentBias", value);
    }
}
impl MLLstmCellOptions {
    pub fn peephole_weight(&self) -> MLOperand {
        self.inner.get("peepholeWeight").as_::<MLOperand>()
    }

    pub fn set_peephole_weight(&mut self, value: &MLOperand) {
        self.inner.set("peepholeWeight", value);
    }
}
impl MLLstmCellOptions {
    pub fn layout(&self) -> MLLstmWeightLayout {
        self.inner.get("layout").as_::<MLLstmWeightLayout>()
    }

    pub fn set_layout(&mut self, value: &MLLstmWeightLayout) {
        self.inner.set("layout", value);
    }
}
impl MLLstmCellOptions {
    pub fn activations(&self) -> Sequence<MLRecurrentNetworkActivation> {
        self.inner
            .get("activations")
            .as_::<Sequence<MLRecurrentNetworkActivation>>()
    }

    pub fn set_activations(&mut self, value: &Sequence<MLRecurrentNetworkActivation>) {
        self.inner.set("activations", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLPadOptions {
    inner: Any,
}
impl FromVal for MLPadOptions {
    fn from_val(v: &Any) -> Self {
        MLPadOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLPadOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLPadOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLPadOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLPadOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLPadOptions> for Any {
    fn from(s: MLPadOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLPadOptions> for Any {
    fn from(s: &MLPadOptions) -> Any {
        s.inner.clone()
    }
}

impl MLPadOptions {
    pub fn mode(&self) -> MLPaddingMode {
        self.inner.get("mode").as_::<MLPaddingMode>()
    }

    pub fn set_mode(&mut self, value: &MLPaddingMode) {
        self.inner.set("mode", value);
    }
}
impl MLPadOptions {
    pub fn value(&self) -> Any {
        self.inner.get("value").as_::<Any>()
    }

    pub fn set_value(&mut self, value: &Any) {
        self.inner.set("value", value);
    }
}
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
    pub fn window_dimensions(&self) -> Sequence<u32> {
        self.inner.get("windowDimensions").as_::<Sequence<u32>>()
    }

    pub fn set_window_dimensions(&mut self, value: Sequence<u32>) {
        self.inner.set("windowDimensions", value);
    }
}
impl MLPool2dOptions {
    pub fn padding(&self) -> Sequence<u32> {
        self.inner.get("padding").as_::<Sequence<u32>>()
    }

    pub fn set_padding(&mut self, value: Sequence<u32>) {
        self.inner.set("padding", value);
    }
}
impl MLPool2dOptions {
    pub fn strides(&self) -> Sequence<u32> {
        self.inner.get("strides").as_::<Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
impl MLPool2dOptions {
    pub fn dilations(&self) -> Sequence<u32> {
        self.inner.get("dilations").as_::<Sequence<u32>>()
    }

    pub fn set_dilations(&mut self, value: Sequence<u32>) {
        self.inner.set("dilations", value);
    }
}
impl MLPool2dOptions {
    pub fn layout(&self) -> MLInputOperandLayout {
        self.inner.get("layout").as_::<MLInputOperandLayout>()
    }

    pub fn set_layout(&mut self, value: &MLInputOperandLayout) {
        self.inner.set("layout", value);
    }
}
impl MLPool2dOptions {
    pub fn rounding_type(&self) -> MLRoundingType {
        self.inner.get("roundingType").as_::<MLRoundingType>()
    }

    pub fn set_rounding_type(&mut self, value: &MLRoundingType) {
        self.inner.set("roundingType", value);
    }
}
impl MLPool2dOptions {
    pub fn output_sizes(&self) -> Sequence<u32> {
        self.inner.get("outputSizes").as_::<Sequence<u32>>()
    }

    pub fn set_output_sizes(&mut self, value: Sequence<u32>) {
        self.inner.set("outputSizes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLReduceOptions {
    inner: Any,
}
impl FromVal for MLReduceOptions {
    fn from_val(v: &Any) -> Self {
        MLReduceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLReduceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLReduceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLReduceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLReduceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLReduceOptions> for Any {
    fn from(s: MLReduceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLReduceOptions> for Any {
    fn from(s: &MLReduceOptions) -> Any {
        s.inner.clone()
    }
}

impl MLReduceOptions {
    pub fn axes(&self) -> Sequence<u32> {
        self.inner.get("axes").as_::<Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: Sequence<u32>) {
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
#[repr(transparent)]
pub struct MLResample2dOptions {
    inner: Any,
}
impl FromVal for MLResample2dOptions {
    fn from_val(v: &Any) -> Self {
        MLResample2dOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLResample2dOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLResample2dOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLResample2dOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLResample2dOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLResample2dOptions> for Any {
    fn from(s: MLResample2dOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLResample2dOptions> for Any {
    fn from(s: &MLResample2dOptions) -> Any {
        s.inner.clone()
    }
}

impl MLResample2dOptions {
    pub fn mode(&self) -> MLInterpolationMode {
        self.inner.get("mode").as_::<MLInterpolationMode>()
    }

    pub fn set_mode(&mut self, value: &MLInterpolationMode) {
        self.inner.set("mode", value);
    }
}
impl MLResample2dOptions {
    pub fn scales(&self) -> Sequence<f32> {
        self.inner.get("scales").as_::<Sequence<f32>>()
    }

    pub fn set_scales(&mut self, value: Sequence<f32>) {
        self.inner.set("scales", value);
    }
}
impl MLResample2dOptions {
    pub fn sizes(&self) -> Sequence<u32> {
        self.inner.get("sizes").as_::<Sequence<u32>>()
    }

    pub fn set_sizes(&mut self, value: Sequence<u32>) {
        self.inner.set("sizes", value);
    }
}
impl MLResample2dOptions {
    pub fn axes(&self) -> Sequence<u32> {
        self.inner.get("axes").as_::<Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLReverseOptions {
    inner: Any,
}
impl FromVal for MLReverseOptions {
    fn from_val(v: &Any) -> Self {
        MLReverseOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLReverseOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLReverseOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLReverseOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLReverseOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLReverseOptions> for Any {
    fn from(s: MLReverseOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLReverseOptions> for Any {
    fn from(s: &MLReverseOptions) -> Any {
        s.inner.clone()
    }
}

impl MLReverseOptions {
    pub fn axes(&self) -> Sequence<u32> {
        self.inner.get("axes").as_::<Sequence<u32>>()
    }

    pub fn set_axes(&mut self, value: Sequence<u32>) {
        self.inner.set("axes", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLScatterOptions {
    inner: Any,
}
impl FromVal for MLScatterOptions {
    fn from_val(v: &Any) -> Self {
        MLScatterOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLScatterOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLScatterOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLScatterOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLScatterOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLScatterOptions> for Any {
    fn from(s: MLScatterOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLScatterOptions> for Any {
    fn from(s: &MLScatterOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLSliceOptions {
    inner: Any,
}
impl FromVal for MLSliceOptions {
    fn from_val(v: &Any) -> Self {
        MLSliceOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLSliceOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLSliceOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLSliceOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLSliceOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLSliceOptions> for Any {
    fn from(s: MLSliceOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLSliceOptions> for Any {
    fn from(s: &MLSliceOptions) -> Any {
        s.inner.clone()
    }
}

impl MLSliceOptions {
    pub fn strides(&self) -> Sequence<u32> {
        self.inner.get("strides").as_::<Sequence<u32>>()
    }

    pub fn set_strides(&mut self, value: Sequence<u32>) {
        self.inner.set("strides", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLSplitOptions {
    inner: Any,
}
impl FromVal for MLSplitOptions {
    fn from_val(v: &Any) -> Self {
        MLSplitOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLSplitOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLSplitOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLSplitOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLSplitOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLSplitOptions> for Any {
    fn from(s: MLSplitOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLSplitOptions> for Any {
    fn from(s: &MLSplitOptions) -> Any {
        s.inner.clone()
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
#[repr(transparent)]
pub struct MLTransposeOptions {
    inner: Any,
}
impl FromVal for MLTransposeOptions {
    fn from_val(v: &Any) -> Self {
        MLTransposeOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTransposeOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTransposeOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLTransposeOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLTransposeOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLTransposeOptions> for Any {
    fn from(s: MLTransposeOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLTransposeOptions> for Any {
    fn from(s: &MLTransposeOptions) -> Any {
        s.inner.clone()
    }
}

impl MLTransposeOptions {
    pub fn permutation(&self) -> Sequence<u32> {
        self.inner.get("permutation").as_::<Sequence<u32>>()
    }

    pub fn set_permutation(&mut self, value: Sequence<u32>) {
        self.inner.set("permutation", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTriangularOptions {
    inner: Any,
}
impl FromVal for MLTriangularOptions {
    fn from_val(v: &Any) -> Self {
        MLTriangularOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for MLTriangularOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTriangularOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLTriangularOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLTriangularOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLTriangularOptions> for Any {
    fn from(s: MLTriangularOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLTriangularOptions> for Any {
    fn from(s: &MLTriangularOptions) -> Any {
        s.inner.clone()
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
/// The MLGraphBuilder class.
/// [`MLGraphBuilder`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLGraphBuilder {
    inner: Any,
}
impl FromVal for MLGraphBuilder {
    fn from_val(v: &Any) -> Self {
        MLGraphBuilder {
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
impl core::ops::Deref for MLGraphBuilder {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLGraphBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for MLGraphBuilder {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for MLGraphBuilder {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<MLGraphBuilder> for Any {
    fn from(s: MLGraphBuilder) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&MLGraphBuilder> for Any {
    fn from(s: &MLGraphBuilder) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(MLGraphBuilder);

impl MLGraphBuilder {
    /// The `new MLGraphBuilder(..)` constructor, creating a new MLGraphBuilder instance
    pub fn new(context: &MLContext) -> MLGraphBuilder {
        Self {
            inner: Any::global("MLGraphBuilder")
                .new(&[context.into()])
                .as_::<Any>(),
        }
    }
}
impl MLGraphBuilder {
    /// The input method.
    /// [`MLGraphBuilder.input`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/input)
    pub fn input(&self, name: &str, descriptor: &MLOperandDescriptor) -> MLOperand {
        self.inner
            .call("input", &[name.into(), descriptor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The constant method.
    /// [`MLGraphBuilder.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/constant)
    pub fn constant(&self, tensor: &MLTensor) -> MLOperand {
        self.inner
            .call("constant", &[tensor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The build method.
    /// [`MLGraphBuilder.build`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/build)
    pub fn build(&self, outputs: &Any) -> Promise {
        self.inner.call("build", &[outputs.into()]).as_::<Promise>()
    }
}
impl MLGraphBuilder {
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMin", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min1(
        &self,
        input: &MLOperand,
        axis: u32,
        options: &MLArgMinMaxOptions,
    ) -> MLOperand {
        self.inner
            .call("argMin", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The argMax method.
    /// [`MLGraphBuilder.argMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMax)
    pub fn arg_max0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
    /// The argMax method.
    /// [`MLGraphBuilder.argMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMax)
    pub fn arg_max1(
        &self,
        input: &MLOperand,
        axis: u32,
        options: &MLArgMinMaxOptions,
    ) -> MLOperand {
        self.inner
            .call("argMax", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The batchNormalization method.
    /// [`MLGraphBuilder.batchNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/batchNormalization)
    pub fn batch_normalization0(
        &self,
        input: &MLOperand,
        mean: &MLOperand,
        variance: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "batchNormalization",
                &[input.into(), mean.into(), variance.into()],
            )
            .as_::<MLOperand>()
    }
    /// The batchNormalization method.
    /// [`MLGraphBuilder.batchNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/batchNormalization)
    pub fn batch_normalization1(
        &self,
        input: &MLOperand,
        mean: &MLOperand,
        variance: &MLOperand,
        options: &MLBatchNormalizationOptions,
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
    /// The cast method.
    /// [`MLGraphBuilder.cast`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cast)
    pub fn cast0(&self, input: &MLOperand, type_: &MLOperandDataType) -> MLOperand {
        self.inner
            .call("cast", &[input.into(), type_.into()])
            .as_::<MLOperand>()
    }
    /// The cast method.
    /// [`MLGraphBuilder.cast`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cast)
    pub fn cast1(
        &self,
        input: &MLOperand,
        type_: &MLOperandDataType,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("cast", &[input.into(), type_.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The clamp method.
    /// [`MLGraphBuilder.clamp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/clamp)
    pub fn clamp0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("clamp", &[input.into()]).as_::<MLOperand>()
    }
    /// The clamp method.
    /// [`MLGraphBuilder.clamp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/clamp)
    pub fn clamp1(&self, input: &MLOperand, options: &MLClampOptions) -> MLOperand {
        self.inner
            .call("clamp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat0(&self, inputs: &Sequence<MLOperand>, axis: u32) -> MLOperand {
        self.inner
            .call("concat", &[inputs.into(), axis.into()])
            .as_::<MLOperand>()
    }
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat1(
        &self,
        inputs: &Sequence<MLOperand>,
        axis: u32,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("concat", &[inputs.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The conv2d method.
    /// [`MLGraphBuilder.conv2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/conv2d)
    pub fn conv2d0(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner
            .call("conv2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }
    /// The conv2d method.
    /// [`MLGraphBuilder.conv2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/conv2d)
    pub fn conv2d1(
        &self,
        input: &MLOperand,
        filter: &MLOperand,
        options: &MLConv2dOptions,
    ) -> MLOperand {
        self.inner
            .call("conv2d", &[input.into(), filter.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The convTranspose2d method.
    /// [`MLGraphBuilder.convTranspose2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/convTranspose2d)
    pub fn conv_transpose2d0(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner
            .call("convTranspose2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }
    /// The convTranspose2d method.
    /// [`MLGraphBuilder.convTranspose2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/convTranspose2d)
    pub fn conv_transpose2d1(
        &self,
        input: &MLOperand,
        filter: &MLOperand,
        options: &MLConvTranspose2dOptions,
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
    /// The cumulativeSum method.
    /// [`MLGraphBuilder.cumulativeSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cumulativeSum)
    pub fn cumulative_sum0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("cumulativeSum", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
    /// The cumulativeSum method.
    /// [`MLGraphBuilder.cumulativeSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cumulativeSum)
    pub fn cumulative_sum1(
        &self,
        input: &MLOperand,
        axis: u32,
        options: &MLCumulativeSumOptions,
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
    /// The add method.
    /// [`MLGraphBuilder.add`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/add)
    pub fn add0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The add method.
    /// [`MLGraphBuilder.add`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/add)
    pub fn add1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("notEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("notEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greater method.
    /// [`MLGraphBuilder.greater`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greater)
    pub fn greater0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The greater method.
    /// [`MLGraphBuilder.greater`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greater)
    pub fn greater1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("greaterOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("greaterOrEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesser method.
    /// [`MLGraphBuilder.lesser`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesser)
    pub fn lesser0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The lesser method.
    /// [`MLGraphBuilder.lesser`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesser)
    pub fn lesser1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("lesserOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("lesserOrEqual", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalNot method.
    /// [`MLGraphBuilder.logicalNot`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalNot)
    pub fn logical_not0(&self, a: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into()])
            .as_::<MLOperand>()
    }
    /// The logicalNot method.
    /// [`MLGraphBuilder.logicalNot`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalNot)
    pub fn logical_not1(&self, a: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalAnd", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalAnd", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalOr method.
    /// [`MLGraphBuilder.logicalOr`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalOr)
    pub fn logical_or0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalOr", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The logicalOr method.
    /// [`MLGraphBuilder.logicalOr`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalOr)
    pub fn logical_or1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalOr", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalXor method.
    /// [`MLGraphBuilder.logicalXor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalXor)
    pub fn logical_xor0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalXor", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The logicalXor method.
    /// [`MLGraphBuilder.logicalXor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalXor)
    pub fn logical_xor1(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalXor", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("abs", &[input.into()]).as_::<MLOperand>()
    }
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("abs", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("ceil", &[input.into()]).as_::<MLOperand>()
    }
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("ceil", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("cos", &[input.into()]).as_::<MLOperand>()
    }
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("cos", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("erf", &[input.into()]).as_::<MLOperand>()
    }
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("erf", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("exp", &[input.into()]).as_::<MLOperand>()
    }
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("exp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("floor", &[input.into()]).as_::<MLOperand>()
    }
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("floor", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("identity", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("identity", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("log", &[input.into()]).as_::<MLOperand>()
    }
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("log", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("neg", &[input.into()]).as_::<MLOperand>()
    }
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("neg", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sin", &[input.into()]).as_::<MLOperand>()
    }
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sign", &[input.into()]).as_::<MLOperand>()
    }
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sqrt", &[input.into()]).as_::<MLOperand>()
    }
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sqrt", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tan", &[input.into()]).as_::<MLOperand>()
    }
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tan", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear0(
        &self,
        input: &MLOperand,
        scale: &MLOperand,
        zero_point: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "dequantizeLinear",
                &[input.into(), scale.into(), zero_point.into()],
            )
            .as_::<MLOperand>()
    }
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear1(
        &self,
        input: &MLOperand,
        scale: &MLOperand,
        zero_point: &MLOperand,
        options: &MLOperatorOptions,
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
    /// The quantizeLinear method.
    /// [`MLGraphBuilder.quantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/quantizeLinear)
    pub fn quantize_linear0(
        &self,
        input: &MLOperand,
        scale: &MLOperand,
        zero_point: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "quantizeLinear",
                &[input.into(), scale.into(), zero_point.into()],
            )
            .as_::<MLOperand>()
    }
    /// The quantizeLinear method.
    /// [`MLGraphBuilder.quantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/quantizeLinear)
    pub fn quantize_linear1(
        &self,
        input: &MLOperand,
        scale: &MLOperand,
        zero_point: &MLOperand,
        options: &MLOperatorOptions,
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
    /// The elu method.
    /// [`MLGraphBuilder.elu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/elu)
    pub fn elu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("elu", &[input.into()]).as_::<MLOperand>()
    }
    /// The elu method.
    /// [`MLGraphBuilder.elu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/elu)
    pub fn elu1(&self, input: &MLOperand, options: &MLEluOptions) -> MLOperand {
        self.inner
            .call("elu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand0(&self, input: &MLOperand, new_shape: Sequence<u32>) -> MLOperand {
        self.inner
            .call("expand", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand1(
        &self,
        input: &MLOperand,
        new_shape: Sequence<u32>,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("expand", &[input.into(), new_shape.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gather method.
    /// [`MLGraphBuilder.gather`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gather)
    pub fn gather0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gather", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
    /// The gather method.
    /// [`MLGraphBuilder.gather`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gather)
    pub fn gather1(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        options: &MLGatherOptions,
    ) -> MLOperand {
        self.inner
            .call("gather", &[input.into(), indices.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gatherElements method.
    /// [`MLGraphBuilder.gatherElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherElements)
    pub fn gather_elements0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gatherElements", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
    /// The gatherElements method.
    /// [`MLGraphBuilder.gatherElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherElements)
    pub fn gather_elements1(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        options: &MLGatherOptions,
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
    /// The gatherND method.
    /// [`MLGraphBuilder.gatherND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherND)
    pub fn gather_nd0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gatherND", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
    /// The gatherND method.
    /// [`MLGraphBuilder.gatherND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherND)
    pub fn gather_nd1(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("gatherND", &[input.into(), indices.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gelu method.
    /// [`MLGraphBuilder.gelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gelu)
    pub fn gelu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("gelu", &[input.into()]).as_::<MLOperand>()
    }
    /// The gelu method.
    /// [`MLGraphBuilder.gelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gelu)
    pub fn gelu1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("gelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm1(&self, a: &MLOperand, b: &MLOperand, options: &MLGemmOptions) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru0(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru1(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
        options: &MLGruOptions,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell0(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
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
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell1(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
        hidden_size: u32,
        options: &MLGruCellOptions,
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
    /// The hardSigmoid method.
    /// [`MLGraphBuilder.hardSigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSigmoid)
    pub fn hard_sigmoid0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The hardSigmoid method.
    /// [`MLGraphBuilder.hardSigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSigmoid)
    pub fn hard_sigmoid1(&self, input: &MLOperand, options: &MLHardSigmoidOptions) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("instanceNormalization", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization1(
        &self,
        input: &MLOperand,
        options: &MLInstanceNormalizationOptions,
    ) -> MLOperand {
        self.inner
            .call("instanceNormalization", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The layerNormalization method.
    /// [`MLGraphBuilder.layerNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/layerNormalization)
    pub fn layer_normalization0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("layerNormalization", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The layerNormalization method.
    /// [`MLGraphBuilder.layerNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/layerNormalization)
    pub fn layer_normalization1(
        &self,
        input: &MLOperand,
        options: &MLLayerNormalizationOptions,
    ) -> MLOperand {
        self.inner
            .call("layerNormalization", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The leakyRelu method.
    /// [`MLGraphBuilder.leakyRelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/leakyRelu)
    pub fn leaky_relu0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The leakyRelu method.
    /// [`MLGraphBuilder.leakyRelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/leakyRelu)
    pub fn leaky_relu1(&self, input: &MLOperand, options: &MLLeakyReluOptions) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("linear", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear1(&self, input: &MLOperand, options: &MLLinearOptions) -> MLOperand {
        self.inner
            .call("linear", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm0(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm1(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
        options: &MLLstmOptions,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell0(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
        cell_state: &MLOperand,
        hidden_size: u32,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell1(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
        cell_state: &MLOperand,
        hidden_size: u32,
        options: &MLLstmCellOptions,
    ) -> Sequence<MLOperand> {
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
            .as_::<Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad0(
        &self,
        input: &MLOperand,
        beginning_padding: Sequence<u32>,
        ending_padding: Sequence<u32>,
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
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad1(
        &self,
        input: &MLOperand,
        beginning_padding: Sequence<u32>,
        ending_padding: Sequence<u32>,
        options: &MLPadOptions,
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
    /// The averagePool2d method.
    /// [`MLGraphBuilder.averagePool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/averagePool2d)
    pub fn average_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The averagePool2d method.
    /// [`MLGraphBuilder.averagePool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/averagePool2d)
    pub fn average_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu0(&self, input: &MLOperand, slope: &MLOperand) -> MLOperand {
        self.inner
            .call("prelu", &[input.into(), slope.into()])
            .as_::<MLOperand>()
    }
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu1(
        &self,
        input: &MLOperand,
        slope: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("prelu", &[input.into(), slope.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL1 method.
    /// [`MLGraphBuilder.reduceL1`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL1)
    pub fn reduce_l10(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceL1 method.
    /// [`MLGraphBuilder.reduceL1`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL1)
    pub fn reduce_l11(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l20(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l21(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("relu", &[input.into()]).as_::<MLOperand>()
    }
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("relu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d1(&self, input: &MLOperand, options: &MLResample2dOptions) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape0(&self, input: &MLOperand, new_shape: Sequence<u32>) -> MLOperand {
        self.inner
            .call("reshape", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape1(
        &self,
        input: &MLOperand,
        new_shape: Sequence<u32>,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("reshape", &[input.into(), new_shape.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reverse method.
    /// [`MLGraphBuilder.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reverse)
    pub fn reverse0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reverse", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The reverse method.
    /// [`MLGraphBuilder.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reverse)
    pub fn reverse1(&self, input: &MLOperand, options: &MLReverseOptions) -> MLOperand {
        self.inner
            .call("reverse", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements0(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        updates: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "scatterElements",
                &[input.into(), indices.into(), updates.into()],
            )
            .as_::<MLOperand>()
    }
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements1(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        updates: &MLOperand,
        options: &MLScatterOptions,
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
    /// The scatterND method.
    /// [`MLGraphBuilder.scatterND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterND)
    pub fn scatter_nd0(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        updates: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call("scatterND", &[input.into(), indices.into(), updates.into()])
            .as_::<MLOperand>()
    }
    /// The scatterND method.
    /// [`MLGraphBuilder.scatterND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterND)
    pub fn scatter_nd1(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        updates: &MLOperand,
        options: &MLOperatorOptions,
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
    /// The sigmoid method.
    /// [`MLGraphBuilder.sigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sigmoid)
    pub fn sigmoid0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The sigmoid method.
    /// [`MLGraphBuilder.sigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sigmoid)
    pub fn sigmoid1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice0(
        &self,
        input: &MLOperand,
        starts: Sequence<u32>,
        sizes: Sequence<u32>,
    ) -> MLOperand {
        self.inner
            .call("slice", &[input.into(), starts.into(), sizes.into()])
            .as_::<MLOperand>()
    }
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice1(
        &self,
        input: &MLOperand,
        starts: Sequence<u32>,
        sizes: Sequence<u32>,
        options: &MLSliceOptions,
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
    /// The softmax method.
    /// [`MLGraphBuilder.softmax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softmax)
    pub fn softmax0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
    /// The softmax method.
    /// [`MLGraphBuilder.softmax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softmax)
    pub fn softmax1(&self, input: &MLOperand, axis: u32, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("softplus", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softplus", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("softsign", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("softsign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split0(&self, input: &MLOperand, splits: &Any) -> Sequence<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into()])
            .as_::<Sequence<MLOperand>>()
    }
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split1(
        &self,
        input: &MLOperand,
        splits: &Any,
        options: &MLSplitOptions,
    ) -> Sequence<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into(), options.into()])
            .as_::<Sequence<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tanh", &[input.into()]).as_::<MLOperand>()
    }
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tanh", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile0(&self, input: &MLOperand, repetitions: Sequence<u32>) -> MLOperand {
        self.inner
            .call("tile", &[input.into(), repetitions.into()])
            .as_::<MLOperand>()
    }
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile1(
        &self,
        input: &MLOperand,
        repetitions: Sequence<u32>,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("tile", &[input.into(), repetitions.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The transpose method.
    /// [`MLGraphBuilder.transpose`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/transpose)
    pub fn transpose0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("transpose", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The transpose method.
    /// [`MLGraphBuilder.transpose`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/transpose)
    pub fn transpose1(&self, input: &MLOperand, options: &MLTransposeOptions) -> MLOperand {
        self.inner
            .call("transpose", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular0(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("triangular", &[input.into()])
            .as_::<MLOperand>()
    }
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular1(&self, input: &MLOperand, options: &MLTriangularOptions) -> MLOperand {
        self.inner
            .call("triangular", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_0(
        &self,
        condition: &MLOperand,
        true_value: &MLOperand,
        false_value: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call(
                "where",
                &[condition.into(), true_value.into(), false_value.into()],
            )
            .as_::<MLOperand>()
    }
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_1(
        &self,
        condition: &MLOperand,
        true_value: &MLOperand,
        false_value: &MLOperand,
        options: &MLOperatorOptions,
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
