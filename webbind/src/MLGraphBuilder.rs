use super::*;

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
    pub fn input(&self, name: &JsString, descriptor: &MLOperandDescriptor) -> MLOperand {
        self.inner
            .call("input", &[name.into(), descriptor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The constant method.
    /// [`MLGraphBuilder.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/constant)
    pub fn constant(&self, descriptor: &MLOperandDescriptor, buffer: &Any) -> MLOperand {
        self.inner
            .call("constant", &[descriptor.into(), buffer.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The constant method.
    /// [`MLGraphBuilder.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/constant)
    pub fn constant_with_type_and_value(
        &self,
        type_: &MLOperandDataType,
        value: &Any,
    ) -> MLOperand {
        self.inner
            .call("constant", &[type_.into(), value.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The constant method.
    /// [`MLGraphBuilder.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/constant)
    pub fn constant_with_tensor(&self, tensor: &MLTensor) -> MLOperand {
        self.inner
            .call("constant", &[tensor.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The build method.
    /// [`MLGraphBuilder.build`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/build)
    pub fn build(&self, outputs: &Any) -> Promise<MLGraph> {
        self.inner
            .call("build", &[outputs.into()])
            .as_::<Promise<MLGraph>>()
    }
}
impl MLGraphBuilder {
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMin", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min_with_options(
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
    pub fn arg_max(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("argMax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The argMax method.
    /// [`MLGraphBuilder.argMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMax)
    pub fn arg_max_with_options(
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
    pub fn batch_normalization(
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
}
impl MLGraphBuilder {
    /// The batchNormalization method.
    /// [`MLGraphBuilder.batchNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/batchNormalization)
    pub fn batch_normalization_with_options(
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
    pub fn cast(&self, input: &MLOperand, type_: &MLOperandDataType) -> MLOperand {
        self.inner
            .call("cast", &[input.into(), type_.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cast method.
    /// [`MLGraphBuilder.cast`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cast)
    pub fn cast_with_options(
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
    pub fn clamp(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("clamp", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The clamp method.
    /// [`MLGraphBuilder.clamp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/clamp)
    pub fn clamp_with_options(&self, input: &MLOperand, options: &MLClampOptions) -> MLOperand {
        self.inner
            .call("clamp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat(&self, inputs: &TypedArray<MLOperand>, axis: u32) -> MLOperand {
        self.inner
            .call("concat", &[inputs.into(), axis.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat_with_options(
        &self,
        inputs: &TypedArray<MLOperand>,
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
    pub fn conv2d(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner
            .call("conv2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The conv2d method.
    /// [`MLGraphBuilder.conv2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/conv2d)
    pub fn conv2d_with_options(
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
    pub fn conv_transpose2d(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner
            .call("convTranspose2d", &[input.into(), filter.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The convTranspose2d method.
    /// [`MLGraphBuilder.convTranspose2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/convTranspose2d)
    pub fn conv_transpose2d_with_options(
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
    pub fn cumulative_sum(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("cumulativeSum", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cumulativeSum method.
    /// [`MLGraphBuilder.cumulativeSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cumulativeSum)
    pub fn cumulative_sum_with_options(
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
    pub fn add(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The add method.
    /// [`MLGraphBuilder.add`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/add)
    pub fn add_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("add", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("sub", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("mul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("div", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("max", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("min", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("pow", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("equal", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("notEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal_with_options(
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
    pub fn greater(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greater method.
    /// [`MLGraphBuilder.greater`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greater)
    pub fn greater_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("greater", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("greaterOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal_with_options(
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
    pub fn lesser(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesser method.
    /// [`MLGraphBuilder.lesser`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesser)
    pub fn lesser_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("lesser", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("lesserOrEqual", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal_with_options(
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
    pub fn logical_not(&self, a: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalNot method.
    /// [`MLGraphBuilder.logicalNot`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalNot)
    pub fn logical_not_with_options(
        &self,
        a: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("logicalNot", &[a.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalAnd", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and_with_options(
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
    pub fn logical_or(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalOr", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalOr method.
    /// [`MLGraphBuilder.logicalOr`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalOr)
    pub fn logical_or_with_options(
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
    pub fn logical_xor(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("logicalXor", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalXor method.
    /// [`MLGraphBuilder.logicalXor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalXor)
    pub fn logical_xor_with_options(
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
    /// The isNaN method.
    /// [`MLGraphBuilder.isNaN`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/isNaN)
    pub fn is_na_n(&self, a: &MLOperand) -> MLOperand {
        self.inner.call("isNaN", &[a.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The isNaN method.
    /// [`MLGraphBuilder.isNaN`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/isNaN)
    pub fn is_na_n_with_options(&self, a: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("isNaN", &[a.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The isInfinite method.
    /// [`MLGraphBuilder.isInfinite`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/isInfinite)
    pub fn is_infinite(&self, a: &MLOperand) -> MLOperand {
        self.inner
            .call("isInfinite", &[a.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The isInfinite method.
    /// [`MLGraphBuilder.isInfinite`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/isInfinite)
    pub fn is_infinite_with_options(
        &self,
        a: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("isInfinite", &[a.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("abs", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("abs", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("ceil", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("ceil", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("cos", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("cos", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("erf", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("erf", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("exp", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("exp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("floor", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("floor", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("identity", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("identity", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("log", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("log", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("neg", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("neg", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("reciprocal", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The roundEven method.
    /// [`MLGraphBuilder.roundEven`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/roundEven)
    pub fn round_even(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("roundEven", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The roundEven method.
    /// [`MLGraphBuilder.roundEven`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/roundEven)
    pub fn round_even_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("roundEven", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sin", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sign", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sqrt", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("sqrt", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tan", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tan", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear(
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
}
impl MLGraphBuilder {
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear_with_options(
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
    pub fn quantize_linear(
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
}
impl MLGraphBuilder {
    /// The quantizeLinear method.
    /// [`MLGraphBuilder.quantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/quantizeLinear)
    pub fn quantize_linear_with_options(
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
    pub fn elu(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("elu", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The elu method.
    /// [`MLGraphBuilder.elu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/elu)
    pub fn elu_with_options(&self, input: &MLOperand, options: &MLEluOptions) -> MLOperand {
        self.inner
            .call("elu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand(&self, input: &MLOperand, new_shape: TypedArray<u32>) -> MLOperand {
        self.inner
            .call("expand", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand_with_options(
        &self,
        input: &MLOperand,
        new_shape: TypedArray<u32>,
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
    pub fn gather(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gather", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gather method.
    /// [`MLGraphBuilder.gather`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gather)
    pub fn gather_with_options(
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
    pub fn gather_elements(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gatherElements", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gatherElements method.
    /// [`MLGraphBuilder.gatherElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherElements)
    pub fn gather_elements_with_options(
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
    pub fn gather_nd(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner
            .call("gatherND", &[input.into(), indices.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gatherND method.
    /// [`MLGraphBuilder.gatherND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherND)
    pub fn gather_nd_with_options(
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
    pub fn gelu(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("gelu", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gelu method.
    /// [`MLGraphBuilder.gelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gelu)
    pub fn gelu_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("gelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLGemmOptions,
    ) -> MLOperand {
        self.inner
            .call("gemm", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru_with_options(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
        options: &MLGruOptions,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell(
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
}
impl MLGraphBuilder {
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell_with_options(
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
    pub fn hard_sigmoid(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSigmoid method.
    /// [`MLGraphBuilder.hardSigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSigmoid)
    pub fn hard_sigmoid_with_options(
        &self,
        input: &MLOperand,
        options: &MLHardSigmoidOptions,
    ) -> MLOperand {
        self.inner
            .call("hardSigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("hardSwish", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("instanceNormalization", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization_with_options(
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
    pub fn layer_normalization(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("layerNormalization", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The layerNormalization method.
    /// [`MLGraphBuilder.layerNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/layerNormalization)
    pub fn layer_normalization_with_options(
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
    pub fn leaky_relu(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The leakyRelu method.
    /// [`MLGraphBuilder.leakyRelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/leakyRelu)
    pub fn leaky_relu_with_options(
        &self,
        input: &MLOperand,
        options: &MLLeakyReluOptions,
    ) -> MLOperand {
        self.inner
            .call("leakyRelu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("linear", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear_with_options(&self, input: &MLOperand, options: &MLLinearOptions) -> MLOperand {
        self.inner
            .call("linear", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm_with_options(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        steps: u32,
        hidden_size: u32,
        options: &MLLstmOptions,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
        cell_state: &MLOperand,
        hidden_size: u32,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell_with_options(
        &self,
        input: &MLOperand,
        weight: &MLOperand,
        recurrent_weight: &MLOperand,
        hidden_state: &MLOperand,
        cell_state: &MLOperand,
        hidden_size: u32,
        options: &MLLstmCellOptions,
    ) -> TypedArray<MLOperand> {
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
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul_with_options(
        &self,
        a: &MLOperand,
        b: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("matmul", &[a.into(), b.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad(
        &self,
        input: &MLOperand,
        beginning_padding: TypedArray<u32>,
        ending_padding: TypedArray<u32>,
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
}
impl MLGraphBuilder {
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad_with_options(
        &self,
        input: &MLOperand,
        beginning_padding: TypedArray<u32>,
        ending_padding: TypedArray<u32>,
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
    pub fn average_pool2d(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The averagePool2d method.
    /// [`MLGraphBuilder.averagePool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/averagePool2d)
    pub fn average_pool2d_with_options(
        &self,
        input: &MLOperand,
        options: &MLPool2dOptions,
    ) -> MLOperand {
        self.inner
            .call("averagePool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d_with_options(
        &self,
        input: &MLOperand,
        options: &MLPool2dOptions,
    ) -> MLOperand {
        self.inner
            .call("l2Pool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d_with_options(
        &self,
        input: &MLOperand,
        options: &MLPool2dOptions,
    ) -> MLOperand {
        self.inner
            .call("maxPool2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu(&self, input: &MLOperand, slope: &MLOperand) -> MLOperand {
        self.inner
            .call("prelu", &[input.into(), slope.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu_with_options(
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
    pub fn reduce_l1(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL1 method.
    /// [`MLGraphBuilder.reduceL1`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL1)
    pub fn reduce_l1_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceL1", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l2(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l2_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceL2", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceLogSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceLogSumExp", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceMax", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceMean", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceMin", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceProduct", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceSum", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square_with_options(
        &self,
        input: &MLOperand,
        options: &MLReduceOptions,
    ) -> MLOperand {
        self.inner
            .call("reduceSumSquare", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("relu", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("relu", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d_with_options(
        &self,
        input: &MLOperand,
        options: &MLResample2dOptions,
    ) -> MLOperand {
        self.inner
            .call("resample2d", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape(&self, input: &MLOperand, new_shape: TypedArray<u32>) -> MLOperand {
        self.inner
            .call("reshape", &[input.into(), new_shape.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape_with_options(
        &self,
        input: &MLOperand,
        new_shape: TypedArray<u32>,
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
    pub fn reverse(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("reverse", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reverse method.
    /// [`MLGraphBuilder.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reverse)
    pub fn reverse_with_options(&self, input: &MLOperand, options: &MLReverseOptions) -> MLOperand {
        self.inner
            .call("reverse", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements(
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
}
impl MLGraphBuilder {
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements_with_options(
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
    pub fn scatter_nd(
        &self,
        input: &MLOperand,
        indices: &MLOperand,
        updates: &MLOperand,
    ) -> MLOperand {
        self.inner
            .call("scatterND", &[input.into(), indices.into(), updates.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The scatterND method.
    /// [`MLGraphBuilder.scatterND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterND)
    pub fn scatter_nd_with_options(
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
    pub fn sigmoid(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sigmoid method.
    /// [`MLGraphBuilder.sigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sigmoid)
    pub fn sigmoid_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("sigmoid", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice(
        &self,
        input: &MLOperand,
        starts: TypedArray<u32>,
        sizes: TypedArray<u32>,
    ) -> MLOperand {
        self.inner
            .call("slice", &[input.into(), starts.into(), sizes.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice_with_options(
        &self,
        input: &MLOperand,
        starts: TypedArray<u32>,
        sizes: TypedArray<u32>,
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
    pub fn softmax(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softmax method.
    /// [`MLGraphBuilder.softmax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softmax)
    pub fn softmax_with_options(
        &self,
        input: &MLOperand,
        axis: u32,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("softmax", &[input.into(), axis.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("softplus", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("softplus", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("softsign", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign_with_options(
        &self,
        input: &MLOperand,
        options: &MLOperatorOptions,
    ) -> MLOperand {
        self.inner
            .call("softsign", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split(&self, input: &MLOperand, splits: &Any) -> TypedArray<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into()])
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split_with_options(
        &self,
        input: &MLOperand,
        splits: &Any,
        options: &MLSplitOptions,
    ) -> TypedArray<MLOperand> {
        self.inner
            .call("split", &[input.into(), splits.into(), options.into()])
            .as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tanh", &[input.into()]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh_with_options(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner
            .call("tanh", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile(&self, input: &MLOperand, repetitions: TypedArray<u32>) -> MLOperand {
        self.inner
            .call("tile", &[input.into(), repetitions.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile_with_options(
        &self,
        input: &MLOperand,
        repetitions: TypedArray<u32>,
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
    pub fn transpose(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("transpose", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The transpose method.
    /// [`MLGraphBuilder.transpose`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/transpose)
    pub fn transpose_with_options(
        &self,
        input: &MLOperand,
        options: &MLTransposeOptions,
    ) -> MLOperand {
        self.inner
            .call("transpose", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular(&self, input: &MLOperand) -> MLOperand {
        self.inner
            .call("triangular", &[input.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular_with_options(
        &self,
        input: &MLOperand,
        options: &MLTriangularOptions,
    ) -> MLOperand {
        self.inner
            .call("triangular", &[input.into(), options.into()])
            .as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_(
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
}
impl MLGraphBuilder {
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_with_options(
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
