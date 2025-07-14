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

    pub const E: f64 = std::f64::consts::E;
    pub const LN2: f64 = std::f64::consts::LN_2;
    pub const LN10: f64 = std::f64::consts::LN_10;
    pub const LOG2E: f64 = std::f64::consts::LOG2_E;
    pub const LOG10E: f64 = std::f64::consts::LOG10_E;
    pub const PI: f64 = std::f64::consts::PI;
    pub const SQRT1_2: f64 = 0.707_106_781_186_547_6; // 1/√2
    pub const SQRT2: f64 = std::f64::consts::SQRT_2;

    unary! {
        /// [`Math.abs`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/abs)
        abs => "abs",
        /// [`Math.acos`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/acos)
        acos => "acos",
        acosh => "acosh",
        asin  => "asin",
        asinh => "asinh",
        atan  => "atan",
        atanh => "atanh",
        cbrt  => "cbrt",
        ceil  => "ceil",
        clz32 => "clz32",
        cos   => "cos",
        cosh  => "cosh",
        exp   => "exp",
        expm1 => "expm1",
        floor => "floor",
        fround => "fround",
        log    => "log",
        log1p  => "log1p",
        log2   => "log2",
        log10  => "log10",
        round  => "round",
        sign   => "sign",
        sin    => "sin",
        sinh   => "sinh",
        sqrt   => "sqrt",
        tan    => "tan",
        tanh   => "tanh",
        trunc  => "trunc",
    }

    /// [`Math.atan2`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/atan2)
    #[inline]
    pub fn atan2(y: f64, x: f64) -> f64 {
        Self::obj()
            .call("atan2", &[y.into(), x.into()])
            .as_::<f64>()
    }

    /// [`Math.pow`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/pow)
    #[inline]
    pub fn pow(x: f64, y: f64) -> f64 {
        Self::obj().call("pow", &[x.into(), y.into()]).as_::<f64>()
    }

    /// [`Math.imul`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/imul)
    #[inline]
    pub fn imul(a: i32, b: i32) -> i32 {
        Self::obj().call("imul", &[a.into(), b.into()]).as_::<i32>()
    }

    /// [`Math.max`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/max)
    #[inline]
    pub fn max(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "max",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// [`Math.min`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/min)
    #[inline]
    pub fn min(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "min",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// [`Math.hypot`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/hypot)
    #[inline]
    pub fn hypot(nums: &[f64]) -> f64 {
        Self::obj()
            .call(
                "hypot",
                &nums.iter().copied().map(Into::into).collect::<Vec<_>>(),
            )
            .as_::<f64>()
    }

    /// [`Math.random`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math/random)
    #[inline]
    pub fn random() -> f64 {
        Self::obj().call("random", &[]).as_::<f64>()
    }
}
