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
        MLGraphBuilder { inner: Any::from_val(v) }
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
            inner: Any::global("MLGraphBuilder").new(&[context.into()]).as_::<Any>(),
        }
    }

}
impl MLGraphBuilder {
    /// The input method.
    /// [`MLGraphBuilder.input`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/input)
    pub fn input(&self, name: &JsString, descriptor: &MLOperandDescriptor) -> MLOperand {
        self.inner.call("input", &[name.into(), descriptor.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The constant method.
    /// [`MLGraphBuilder.constant`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/constant)
    pub fn constant(&self, tensor: &MLTensor) -> MLOperand {
        self.inner.call("constant", &[tensor.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The build method.
    /// [`MLGraphBuilder.build`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/build)
    pub fn build(&self, outputs: &Any) -> Promise<MLGraph> {
        self.inner.call("build", &[outputs.into(), ]).as_::<Promise<MLGraph>>()
    }
}
impl MLGraphBuilder {
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner.call("argMin", &[input.into(), axis.into(), ]).as_::<MLOperand>()
    }
    /// The argMin method.
    /// [`MLGraphBuilder.argMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMin)
    pub fn arg_min1(&self, input: &MLOperand, axis: u32, options: &MLArgMinMaxOptions) -> MLOperand {
        self.inner.call("argMin", &[input.into(), axis.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The argMax method.
    /// [`MLGraphBuilder.argMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMax)
    pub fn arg_max0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner.call("argMax", &[input.into(), axis.into(), ]).as_::<MLOperand>()
    }
    /// The argMax method.
    /// [`MLGraphBuilder.argMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/argMax)
    pub fn arg_max1(&self, input: &MLOperand, axis: u32, options: &MLArgMinMaxOptions) -> MLOperand {
        self.inner.call("argMax", &[input.into(), axis.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The batchNormalization method.
    /// [`MLGraphBuilder.batchNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/batchNormalization)
    pub fn batch_normalization0(&self, input: &MLOperand, mean: &MLOperand, variance: &MLOperand) -> MLOperand {
        self.inner.call("batchNormalization", &[input.into(), mean.into(), variance.into(), ]).as_::<MLOperand>()
    }
    /// The batchNormalization method.
    /// [`MLGraphBuilder.batchNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/batchNormalization)
    pub fn batch_normalization1(&self, input: &MLOperand, mean: &MLOperand, variance: &MLOperand, options: &MLBatchNormalizationOptions) -> MLOperand {
        self.inner.call("batchNormalization", &[input.into(), mean.into(), variance.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cast method.
    /// [`MLGraphBuilder.cast`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cast)
    pub fn cast0(&self, input: &MLOperand, type_: &MLOperandDataType) -> MLOperand {
        self.inner.call("cast", &[input.into(), type_.into(), ]).as_::<MLOperand>()
    }
    /// The cast method.
    /// [`MLGraphBuilder.cast`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cast)
    pub fn cast1(&self, input: &MLOperand, type_: &MLOperandDataType, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("cast", &[input.into(), type_.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The clamp method.
    /// [`MLGraphBuilder.clamp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/clamp)
    pub fn clamp0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("clamp", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The clamp method.
    /// [`MLGraphBuilder.clamp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/clamp)
    pub fn clamp1(&self, input: &MLOperand, options: &MLClampOptions) -> MLOperand {
        self.inner.call("clamp", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat0(&self, inputs: &TypedArray<MLOperand>, axis: u32) -> MLOperand {
        self.inner.call("concat", &[inputs.into(), axis.into(), ]).as_::<MLOperand>()
    }
    /// The concat method.
    /// [`MLGraphBuilder.concat`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/concat)
    pub fn concat1(&self, inputs: &TypedArray<MLOperand>, axis: u32, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("concat", &[inputs.into(), axis.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The conv2d method.
    /// [`MLGraphBuilder.conv2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/conv2d)
    pub fn conv2d0(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner.call("conv2d", &[input.into(), filter.into(), ]).as_::<MLOperand>()
    }
    /// The conv2d method.
    /// [`MLGraphBuilder.conv2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/conv2d)
    pub fn conv2d1(&self, input: &MLOperand, filter: &MLOperand, options: &MLConv2dOptions) -> MLOperand {
        self.inner.call("conv2d", &[input.into(), filter.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The convTranspose2d method.
    /// [`MLGraphBuilder.convTranspose2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/convTranspose2d)
    pub fn conv_transpose2d0(&self, input: &MLOperand, filter: &MLOperand) -> MLOperand {
        self.inner.call("convTranspose2d", &[input.into(), filter.into(), ]).as_::<MLOperand>()
    }
    /// The convTranspose2d method.
    /// [`MLGraphBuilder.convTranspose2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/convTranspose2d)
    pub fn conv_transpose2d1(&self, input: &MLOperand, filter: &MLOperand, options: &MLConvTranspose2dOptions) -> MLOperand {
        self.inner.call("convTranspose2d", &[input.into(), filter.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cumulativeSum method.
    /// [`MLGraphBuilder.cumulativeSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cumulativeSum)
    pub fn cumulative_sum0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner.call("cumulativeSum", &[input.into(), axis.into(), ]).as_::<MLOperand>()
    }
    /// The cumulativeSum method.
    /// [`MLGraphBuilder.cumulativeSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cumulativeSum)
    pub fn cumulative_sum1(&self, input: &MLOperand, axis: u32, options: &MLCumulativeSumOptions) -> MLOperand {
        self.inner.call("cumulativeSum", &[input.into(), axis.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The add method.
    /// [`MLGraphBuilder.add`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/add)
    pub fn add0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("add", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The add method.
    /// [`MLGraphBuilder.add`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/add)
    pub fn add1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("add", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("sub", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The sub method.
    /// [`MLGraphBuilder.sub`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sub)
    pub fn sub1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("sub", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("mul", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The mul method.
    /// [`MLGraphBuilder.mul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/mul)
    pub fn mul1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("mul", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("div", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The div method.
    /// [`MLGraphBuilder.div`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/div)
    pub fn div1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("div", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("max", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The max method.
    /// [`MLGraphBuilder.max`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/max)
    pub fn max1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("max", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("min", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The min method.
    /// [`MLGraphBuilder.min`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/min)
    pub fn min1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("min", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("pow", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The pow method.
    /// [`MLGraphBuilder.pow`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pow)
    pub fn pow1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("pow", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("equal", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The equal method.
    /// [`MLGraphBuilder.equal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/equal)
    pub fn equal1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("equal", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("notEqual", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The notEqual method.
    /// [`MLGraphBuilder.notEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/notEqual)
    pub fn not_equal1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("notEqual", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greater method.
    /// [`MLGraphBuilder.greater`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greater)
    pub fn greater0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("greater", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The greater method.
    /// [`MLGraphBuilder.greater`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greater)
    pub fn greater1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("greater", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("greaterOrEqual", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The greaterOrEqual method.
    /// [`MLGraphBuilder.greaterOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/greaterOrEqual)
    pub fn greater_or_equal1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("greaterOrEqual", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesser method.
    /// [`MLGraphBuilder.lesser`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesser)
    pub fn lesser0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("lesser", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The lesser method.
    /// [`MLGraphBuilder.lesser`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesser)
    pub fn lesser1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("lesser", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("lesserOrEqual", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The lesserOrEqual method.
    /// [`MLGraphBuilder.lesserOrEqual`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lesserOrEqual)
    pub fn lesser_or_equal1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("lesserOrEqual", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalNot method.
    /// [`MLGraphBuilder.logicalNot`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalNot)
    pub fn logical_not0(&self, a: &MLOperand) -> MLOperand {
        self.inner.call("logicalNot", &[a.into(), ]).as_::<MLOperand>()
    }
    /// The logicalNot method.
    /// [`MLGraphBuilder.logicalNot`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalNot)
    pub fn logical_not1(&self, a: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("logicalNot", &[a.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("logicalAnd", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The logicalAnd method.
    /// [`MLGraphBuilder.logicalAnd`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalAnd)
    pub fn logical_and1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("logicalAnd", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalOr method.
    /// [`MLGraphBuilder.logicalOr`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalOr)
    pub fn logical_or0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("logicalOr", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The logicalOr method.
    /// [`MLGraphBuilder.logicalOr`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalOr)
    pub fn logical_or1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("logicalOr", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The logicalXor method.
    /// [`MLGraphBuilder.logicalXor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalXor)
    pub fn logical_xor0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("logicalXor", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The logicalXor method.
    /// [`MLGraphBuilder.logicalXor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/logicalXor)
    pub fn logical_xor1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("logicalXor", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("abs", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The abs method.
    /// [`MLGraphBuilder.abs`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/abs)
    pub fn abs1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("abs", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("ceil", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The ceil method.
    /// [`MLGraphBuilder.ceil`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/ceil)
    pub fn ceil1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("ceil", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("cos", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The cos method.
    /// [`MLGraphBuilder.cos`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/cos)
    pub fn cos1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("cos", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("erf", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The erf method.
    /// [`MLGraphBuilder.erf`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/erf)
    pub fn erf1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("erf", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("exp", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The exp method.
    /// [`MLGraphBuilder.exp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/exp)
    pub fn exp1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("exp", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("floor", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The floor method.
    /// [`MLGraphBuilder.floor`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/floor)
    pub fn floor1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("floor", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("identity", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The identity method.
    /// [`MLGraphBuilder.identity`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/identity)
    pub fn identity1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("identity", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("log", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The log method.
    /// [`MLGraphBuilder.log`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/log)
    pub fn log1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("log", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("neg", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The neg method.
    /// [`MLGraphBuilder.neg`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/neg)
    pub fn neg1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("neg", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reciprocal", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reciprocal method.
    /// [`MLGraphBuilder.reciprocal`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reciprocal)
    pub fn reciprocal1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("reciprocal", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sin", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The sin method.
    /// [`MLGraphBuilder.sin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sin)
    pub fn sin1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("sin", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sign", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The sign method.
    /// [`MLGraphBuilder.sign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sign)
    pub fn sign1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("sign", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sqrt", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The sqrt method.
    /// [`MLGraphBuilder.sqrt`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sqrt)
    pub fn sqrt1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("sqrt", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tan", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The tan method.
    /// [`MLGraphBuilder.tan`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tan)
    pub fn tan1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("tan", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear0(&self, input: &MLOperand, scale: &MLOperand, zero_point: &MLOperand) -> MLOperand {
        self.inner.call("dequantizeLinear", &[input.into(), scale.into(), zero_point.into(), ]).as_::<MLOperand>()
    }
    /// The dequantizeLinear method.
    /// [`MLGraphBuilder.dequantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/dequantizeLinear)
    pub fn dequantize_linear1(&self, input: &MLOperand, scale: &MLOperand, zero_point: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("dequantizeLinear", &[input.into(), scale.into(), zero_point.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The quantizeLinear method.
    /// [`MLGraphBuilder.quantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/quantizeLinear)
    pub fn quantize_linear0(&self, input: &MLOperand, scale: &MLOperand, zero_point: &MLOperand) -> MLOperand {
        self.inner.call("quantizeLinear", &[input.into(), scale.into(), zero_point.into(), ]).as_::<MLOperand>()
    }
    /// The quantizeLinear method.
    /// [`MLGraphBuilder.quantizeLinear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/quantizeLinear)
    pub fn quantize_linear1(&self, input: &MLOperand, scale: &MLOperand, zero_point: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("quantizeLinear", &[input.into(), scale.into(), zero_point.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The elu method.
    /// [`MLGraphBuilder.elu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/elu)
    pub fn elu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("elu", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The elu method.
    /// [`MLGraphBuilder.elu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/elu)
    pub fn elu1(&self, input: &MLOperand, options: &MLEluOptions) -> MLOperand {
        self.inner.call("elu", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand0(&self, input: &MLOperand, new_shape: TypedArray<u32>) -> MLOperand {
        self.inner.call("expand", &[input.into(), new_shape.into(), ]).as_::<MLOperand>()
    }
    /// The expand method.
    /// [`MLGraphBuilder.expand`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/expand)
    pub fn expand1(&self, input: &MLOperand, new_shape: TypedArray<u32>, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("expand", &[input.into(), new_shape.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gather method.
    /// [`MLGraphBuilder.gather`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gather)
    pub fn gather0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner.call("gather", &[input.into(), indices.into(), ]).as_::<MLOperand>()
    }
    /// The gather method.
    /// [`MLGraphBuilder.gather`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gather)
    pub fn gather1(&self, input: &MLOperand, indices: &MLOperand, options: &MLGatherOptions) -> MLOperand {
        self.inner.call("gather", &[input.into(), indices.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gatherElements method.
    /// [`MLGraphBuilder.gatherElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherElements)
    pub fn gather_elements0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner.call("gatherElements", &[input.into(), indices.into(), ]).as_::<MLOperand>()
    }
    /// The gatherElements method.
    /// [`MLGraphBuilder.gatherElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherElements)
    pub fn gather_elements1(&self, input: &MLOperand, indices: &MLOperand, options: &MLGatherOptions) -> MLOperand {
        self.inner.call("gatherElements", &[input.into(), indices.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gatherND method.
    /// [`MLGraphBuilder.gatherND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherND)
    pub fn gather_nd0(&self, input: &MLOperand, indices: &MLOperand) -> MLOperand {
        self.inner.call("gatherND", &[input.into(), indices.into(), ]).as_::<MLOperand>()
    }
    /// The gatherND method.
    /// [`MLGraphBuilder.gatherND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gatherND)
    pub fn gather_nd1(&self, input: &MLOperand, indices: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("gatherND", &[input.into(), indices.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gelu method.
    /// [`MLGraphBuilder.gelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gelu)
    pub fn gelu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("gelu", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The gelu method.
    /// [`MLGraphBuilder.gelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gelu)
    pub fn gelu1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("gelu", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("gemm", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The gemm method.
    /// [`MLGraphBuilder.gemm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gemm)
    pub fn gemm1(&self, a: &MLOperand, b: &MLOperand, options: &MLGemmOptions) -> MLOperand {
        self.inner.call("gemm", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru0(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, steps: u32, hidden_size: u32) -> TypedArray<MLOperand> {
        self.inner.call("gru", &[input.into(), weight.into(), recurrent_weight.into(), steps.into(), hidden_size.into(), ]).as_::<TypedArray<MLOperand>>()
    }
    /// The gru method.
    /// [`MLGraphBuilder.gru`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gru)
    pub fn gru1(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, steps: u32, hidden_size: u32, options: &MLGruOptions) -> TypedArray<MLOperand> {
        self.inner.call("gru", &[input.into(), weight.into(), recurrent_weight.into(), steps.into(), hidden_size.into(), options.into(), ]).as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell0(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, hidden_state: &MLOperand, hidden_size: u32) -> MLOperand {
        self.inner.call("gruCell", &[input.into(), weight.into(), recurrent_weight.into(), hidden_state.into(), hidden_size.into(), ]).as_::<MLOperand>()
    }
    /// The gruCell method.
    /// [`MLGraphBuilder.gruCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/gruCell)
    pub fn gru_cell1(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, hidden_state: &MLOperand, hidden_size: u32, options: &MLGruCellOptions) -> MLOperand {
        self.inner.call("gruCell", &[input.into(), weight.into(), recurrent_weight.into(), hidden_state.into(), hidden_size.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSigmoid method.
    /// [`MLGraphBuilder.hardSigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSigmoid)
    pub fn hard_sigmoid0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("hardSigmoid", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The hardSigmoid method.
    /// [`MLGraphBuilder.hardSigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSigmoid)
    pub fn hard_sigmoid1(&self, input: &MLOperand, options: &MLHardSigmoidOptions) -> MLOperand {
        self.inner.call("hardSigmoid", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("hardSwish", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The hardSwish method.
    /// [`MLGraphBuilder.hardSwish`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/hardSwish)
    pub fn hard_swish1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("hardSwish", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("instanceNormalization", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The instanceNormalization method.
    /// [`MLGraphBuilder.instanceNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/instanceNormalization)
    pub fn instance_normalization1(&self, input: &MLOperand, options: &MLInstanceNormalizationOptions) -> MLOperand {
        self.inner.call("instanceNormalization", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The layerNormalization method.
    /// [`MLGraphBuilder.layerNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/layerNormalization)
    pub fn layer_normalization0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("layerNormalization", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The layerNormalization method.
    /// [`MLGraphBuilder.layerNormalization`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/layerNormalization)
    pub fn layer_normalization1(&self, input: &MLOperand, options: &MLLayerNormalizationOptions) -> MLOperand {
        self.inner.call("layerNormalization", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The leakyRelu method.
    /// [`MLGraphBuilder.leakyRelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/leakyRelu)
    pub fn leaky_relu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("leakyRelu", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The leakyRelu method.
    /// [`MLGraphBuilder.leakyRelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/leakyRelu)
    pub fn leaky_relu1(&self, input: &MLOperand, options: &MLLeakyReluOptions) -> MLOperand {
        self.inner.call("leakyRelu", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("linear", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The linear method.
    /// [`MLGraphBuilder.linear`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/linear)
    pub fn linear1(&self, input: &MLOperand, options: &MLLinearOptions) -> MLOperand {
        self.inner.call("linear", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm0(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, steps: u32, hidden_size: u32) -> TypedArray<MLOperand> {
        self.inner.call("lstm", &[input.into(), weight.into(), recurrent_weight.into(), steps.into(), hidden_size.into(), ]).as_::<TypedArray<MLOperand>>()
    }
    /// The lstm method.
    /// [`MLGraphBuilder.lstm`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstm)
    pub fn lstm1(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, steps: u32, hidden_size: u32, options: &MLLstmOptions) -> TypedArray<MLOperand> {
        self.inner.call("lstm", &[input.into(), weight.into(), recurrent_weight.into(), steps.into(), hidden_size.into(), options.into(), ]).as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell0(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, hidden_state: &MLOperand, cell_state: &MLOperand, hidden_size: u32) -> TypedArray<MLOperand> {
        self.inner.call("lstmCell", &[input.into(), weight.into(), recurrent_weight.into(), hidden_state.into(), cell_state.into(), hidden_size.into(), ]).as_::<TypedArray<MLOperand>>()
    }
    /// The lstmCell method.
    /// [`MLGraphBuilder.lstmCell`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/lstmCell)
    pub fn lstm_cell1(&self, input: &MLOperand, weight: &MLOperand, recurrent_weight: &MLOperand, hidden_state: &MLOperand, cell_state: &MLOperand, hidden_size: u32, options: &MLLstmCellOptions) -> TypedArray<MLOperand> {
        self.inner.call("lstmCell", &[input.into(), weight.into(), recurrent_weight.into(), hidden_state.into(), cell_state.into(), hidden_size.into(), options.into(), ]).as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul0(&self, a: &MLOperand, b: &MLOperand) -> MLOperand {
        self.inner.call("matmul", &[a.into(), b.into(), ]).as_::<MLOperand>()
    }
    /// The matmul method.
    /// [`MLGraphBuilder.matmul`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/matmul)
    pub fn matmul1(&self, a: &MLOperand, b: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("matmul", &[a.into(), b.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad0(&self, input: &MLOperand, beginning_padding: TypedArray<u32>, ending_padding: TypedArray<u32>) -> MLOperand {
        self.inner.call("pad", &[input.into(), beginning_padding.into(), ending_padding.into(), ]).as_::<MLOperand>()
    }
    /// The pad method.
    /// [`MLGraphBuilder.pad`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/pad)
    pub fn pad1(&self, input: &MLOperand, beginning_padding: TypedArray<u32>, ending_padding: TypedArray<u32>, options: &MLPadOptions) -> MLOperand {
        self.inner.call("pad", &[input.into(), beginning_padding.into(), ending_padding.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The averagePool2d method.
    /// [`MLGraphBuilder.averagePool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/averagePool2d)
    pub fn average_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("averagePool2d", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The averagePool2d method.
    /// [`MLGraphBuilder.averagePool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/averagePool2d)
    pub fn average_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner.call("averagePool2d", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("l2Pool2d", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The l2Pool2d method.
    /// [`MLGraphBuilder.l2Pool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/l2Pool2d)
    pub fn l2_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner.call("l2Pool2d", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("maxPool2d", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The maxPool2d method.
    /// [`MLGraphBuilder.maxPool2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/maxPool2d)
    pub fn max_pool2d1(&self, input: &MLOperand, options: &MLPool2dOptions) -> MLOperand {
        self.inner.call("maxPool2d", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu0(&self, input: &MLOperand, slope: &MLOperand) -> MLOperand {
        self.inner.call("prelu", &[input.into(), slope.into(), ]).as_::<MLOperand>()
    }
    /// The prelu method.
    /// [`MLGraphBuilder.prelu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/prelu)
    pub fn prelu1(&self, input: &MLOperand, slope: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("prelu", &[input.into(), slope.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL1 method.
    /// [`MLGraphBuilder.reduceL1`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL1)
    pub fn reduce_l10(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceL1", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceL1 method.
    /// [`MLGraphBuilder.reduceL1`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL1)
    pub fn reduce_l11(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceL1", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l20(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceL2", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceL2 method.
    /// [`MLGraphBuilder.reduceL2`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceL2)
    pub fn reduce_l21(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceL2", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceLogSum", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceLogSum method.
    /// [`MLGraphBuilder.reduceLogSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSum)
    pub fn reduce_log_sum1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceLogSum", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceLogSumExp", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceLogSumExp method.
    /// [`MLGraphBuilder.reduceLogSumExp`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceLogSumExp)
    pub fn reduce_log_sum_exp1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceLogSumExp", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceMax", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceMax method.
    /// [`MLGraphBuilder.reduceMax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMax)
    pub fn reduce_max1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceMax", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceMean", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceMean method.
    /// [`MLGraphBuilder.reduceMean`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMean)
    pub fn reduce_mean1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceMean", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceMin", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceMin method.
    /// [`MLGraphBuilder.reduceMin`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceMin)
    pub fn reduce_min1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceMin", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceProduct", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceProduct method.
    /// [`MLGraphBuilder.reduceProduct`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceProduct)
    pub fn reduce_product1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceProduct", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceSum", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceSum method.
    /// [`MLGraphBuilder.reduceSum`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSum)
    pub fn reduce_sum1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceSum", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reduceSumSquare", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reduceSumSquare method.
    /// [`MLGraphBuilder.reduceSumSquare`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reduceSumSquare)
    pub fn reduce_sum_square1(&self, input: &MLOperand, options: &MLReduceOptions) -> MLOperand {
        self.inner.call("reduceSumSquare", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("relu", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The relu method.
    /// [`MLGraphBuilder.relu`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/relu)
    pub fn relu1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("relu", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("resample2d", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The resample2d method.
    /// [`MLGraphBuilder.resample2d`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/resample2d)
    pub fn resample2d1(&self, input: &MLOperand, options: &MLResample2dOptions) -> MLOperand {
        self.inner.call("resample2d", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape0(&self, input: &MLOperand, new_shape: TypedArray<u32>) -> MLOperand {
        self.inner.call("reshape", &[input.into(), new_shape.into(), ]).as_::<MLOperand>()
    }
    /// The reshape method.
    /// [`MLGraphBuilder.reshape`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reshape)
    pub fn reshape1(&self, input: &MLOperand, new_shape: TypedArray<u32>, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("reshape", &[input.into(), new_shape.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The reverse method.
    /// [`MLGraphBuilder.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reverse)
    pub fn reverse0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("reverse", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The reverse method.
    /// [`MLGraphBuilder.reverse`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/reverse)
    pub fn reverse1(&self, input: &MLOperand, options: &MLReverseOptions) -> MLOperand {
        self.inner.call("reverse", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements0(&self, input: &MLOperand, indices: &MLOperand, updates: &MLOperand) -> MLOperand {
        self.inner.call("scatterElements", &[input.into(), indices.into(), updates.into(), ]).as_::<MLOperand>()
    }
    /// The scatterElements method.
    /// [`MLGraphBuilder.scatterElements`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterElements)
    pub fn scatter_elements1(&self, input: &MLOperand, indices: &MLOperand, updates: &MLOperand, options: &MLScatterOptions) -> MLOperand {
        self.inner.call("scatterElements", &[input.into(), indices.into(), updates.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The scatterND method.
    /// [`MLGraphBuilder.scatterND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterND)
    pub fn scatter_nd0(&self, input: &MLOperand, indices: &MLOperand, updates: &MLOperand) -> MLOperand {
        self.inner.call("scatterND", &[input.into(), indices.into(), updates.into(), ]).as_::<MLOperand>()
    }
    /// The scatterND method.
    /// [`MLGraphBuilder.scatterND`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/scatterND)
    pub fn scatter_nd1(&self, input: &MLOperand, indices: &MLOperand, updates: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("scatterND", &[input.into(), indices.into(), updates.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The sigmoid method.
    /// [`MLGraphBuilder.sigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sigmoid)
    pub fn sigmoid0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("sigmoid", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The sigmoid method.
    /// [`MLGraphBuilder.sigmoid`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/sigmoid)
    pub fn sigmoid1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("sigmoid", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice0(&self, input: &MLOperand, starts: TypedArray<u32>, sizes: TypedArray<u32>) -> MLOperand {
        self.inner.call("slice", &[input.into(), starts.into(), sizes.into(), ]).as_::<MLOperand>()
    }
    /// The slice method.
    /// [`MLGraphBuilder.slice`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/slice)
    pub fn slice1(&self, input: &MLOperand, starts: TypedArray<u32>, sizes: TypedArray<u32>, options: &MLSliceOptions) -> MLOperand {
        self.inner.call("slice", &[input.into(), starts.into(), sizes.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softmax method.
    /// [`MLGraphBuilder.softmax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softmax)
    pub fn softmax0(&self, input: &MLOperand, axis: u32) -> MLOperand {
        self.inner.call("softmax", &[input.into(), axis.into(), ]).as_::<MLOperand>()
    }
    /// The softmax method.
    /// [`MLGraphBuilder.softmax`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softmax)
    pub fn softmax1(&self, input: &MLOperand, axis: u32, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("softmax", &[input.into(), axis.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("softplus", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The softplus method.
    /// [`MLGraphBuilder.softplus`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softplus)
    pub fn softplus1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("softplus", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("softsign", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The softsign method.
    /// [`MLGraphBuilder.softsign`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/softsign)
    pub fn softsign1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("softsign", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split0(&self, input: &MLOperand, splits: &Any) -> TypedArray<MLOperand> {
        self.inner.call("split", &[input.into(), splits.into(), ]).as_::<TypedArray<MLOperand>>()
    }
    /// The split method.
    /// [`MLGraphBuilder.split`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/split)
    pub fn split1(&self, input: &MLOperand, splits: &Any, options: &MLSplitOptions) -> TypedArray<MLOperand> {
        self.inner.call("split", &[input.into(), splits.into(), options.into(), ]).as_::<TypedArray<MLOperand>>()
    }
}
impl MLGraphBuilder {
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("tanh", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The tanh method.
    /// [`MLGraphBuilder.tanh`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tanh)
    pub fn tanh1(&self, input: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("tanh", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile0(&self, input: &MLOperand, repetitions: TypedArray<u32>) -> MLOperand {
        self.inner.call("tile", &[input.into(), repetitions.into(), ]).as_::<MLOperand>()
    }
    /// The tile method.
    /// [`MLGraphBuilder.tile`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/tile)
    pub fn tile1(&self, input: &MLOperand, repetitions: TypedArray<u32>, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("tile", &[input.into(), repetitions.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The transpose method.
    /// [`MLGraphBuilder.transpose`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/transpose)
    pub fn transpose0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("transpose", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The transpose method.
    /// [`MLGraphBuilder.transpose`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/transpose)
    pub fn transpose1(&self, input: &MLOperand, options: &MLTransposeOptions) -> MLOperand {
        self.inner.call("transpose", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular0(&self, input: &MLOperand) -> MLOperand {
        self.inner.call("triangular", &[input.into(), ]).as_::<MLOperand>()
    }
    /// The triangular method.
    /// [`MLGraphBuilder.triangular`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/triangular)
    pub fn triangular1(&self, input: &MLOperand, options: &MLTriangularOptions) -> MLOperand {
        self.inner.call("triangular", &[input.into(), options.into(), ]).as_::<MLOperand>()
    }
}
impl MLGraphBuilder {
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_0(&self, condition: &MLOperand, true_value: &MLOperand, false_value: &MLOperand) -> MLOperand {
        self.inner.call("where", &[condition.into(), true_value.into(), false_value.into(), ]).as_::<MLOperand>()
    }
    /// The where method.
    /// [`MLGraphBuilder.where`](https://developer.mozilla.org/en-US/docs/Web/API/MLGraphBuilder/where)
    pub fn where_1(&self, condition: &MLOperand, true_value: &MLOperand, false_value: &MLOperand, options: &MLOperatorOptions) -> MLOperand {
        self.inner.call("where", &[condition.into(), true_value.into(), false_value.into(), options.into(), ]).as_::<MLOperand>()
    }
}
