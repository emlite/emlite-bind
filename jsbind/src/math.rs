use alloc::vec::Vec;

macro_rules! unary {
    ($( $(#[$meta:meta])* $name:ident => $js:literal ),* $(,)?) => {$(
        $(#[$meta])*
        #[inline]
        pub fn $name(x: f64) -> f64 {
            Self::obj().call($js, &[x.into()]).as_::<f64>()
        }
    )*};
}

/// Namespace only; no value is ever instantiated.
pub struct Math;

impl Math {
    #[inline(always)]
    fn obj() -> emlite::Val {
        emlite::Val::global("Math")
    }

    pub const E: f64 = core::f64::consts::E;
    pub const LN2: f64 = core::f64::consts::LN_2;
    pub const LN10: f64 = core::f64::consts::LN_10;
    pub const LOG2E: f64 = core::f64::consts::LOG2_E;
    pub const LOG10E: f64 = core::f64::consts::LOG10_E;
    pub const PI: f64 = core::f64::consts::PI;
    pub const SQRT1_2: f64 = core::f64::consts::FRAC_1_SQRT_2; // 1/√2
    pub const SQRT2: f64 = core::f64::consts::SQRT_2;

    unary! {
        /// `Math.abs`
        abs => "abs",
        /// `Math.acos`
        /// `Math.acos`
        acos => "acos",
        /// `Math.acosh`
        acosh => "acosh",
        /// `Math.asin`
        asin  => "asin",
        /// `Math.asinh`
        asinh => "asinh",
        /// `Math.atan`
        atan  => "atan",
        /// `Math.atanh`
        atanh => "atanh",
        /// `Math.cbrt`
        cbrt  => "cbrt",
        /// `Math.ceil`
        ceil  => "ceil",
        /// `Math.clz32`
        clz32 => "clz32",
        /// `Math.cos`
        cos   => "cos",
        /// `Math.cosh`
        cosh  => "cosh",
        /// `Math.exp`
        exp   => "exp",
        /// `Math.expm1`
        expm1 => "expm1",
        /// `Math.floor`
        floor => "floor",
        /// `Math.fround`
        fround => "fround",
        /// `Math.log`
        log    => "log",
        /// `Math.log1p`
        log1p  => "log1p",
        /// `Math.log2`
        log2   => "log2",
        /// `Math.log10`
        log10  => "log10",
        /// `Math.round`
        round  => "round",
        /// `Math.sign`
        sign   => "sign",
        /// `Math.sin`
        sin    => "sin",
        /// `Math.sinh`
        sinh   => "sinh",
        /// `Math.sqrt`
        sqrt   => "sqrt",
        /// `Math.tan`
        tan    => "tan",
        /// `Math.tanh`
        tanh   => "tanh",
        /// `Math.trunc`
        trunc  => "trunc",
    }

    /// `Math.atan2`
    #[inline]
    pub fn atan2(y: f64, x: f64) -> f64 {
        Self::obj()
            .call("atan2", &[y.into(), x.into()])
            .as_::<f64>()
    }

    /// `Math.pow`
    #[inline]
    pub fn pow(x: f64, y: f64) -> f64 {
        Self::obj().call("pow", &[x.into(), y.into()]).as_::<f64>()
    }

    /// `Math.imul`
    #[inline]
    pub fn imul(a: i32, b: i32) -> i32 {
        Self::obj().call("imul", &[a.into(), b.into()]).as_::<i32>()
    }

    /// `Math.max`
    #[inline]
    pub fn max(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "max",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// `Math.min`
    #[inline]
    pub fn min(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "min",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// `Math.hypot`
    #[inline]
    pub fn hypot(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "hypot",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// `Math.random`
    #[inline]
    pub fn random() -> f64 {
        Self::obj().call("random", &[]).as_::<f64>()
    }
}
