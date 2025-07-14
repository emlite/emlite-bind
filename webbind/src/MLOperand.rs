use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MLOperand {
    inner: emlite::Val,
}
impl FromVal for MLOperand {
    fn from_val(v: &emlite::Val) -> Self {
        MLOperand {
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
impl core::ops::Deref for MLOperand {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLOperand {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<MLOperand> for emlite::Val {
    fn from(s: MLOperand) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl MLOperand {
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }
}
impl MLOperand {
    pub fn shape(&self) -> jsbind::FrozenArray<u32> {
        self.inner.get("shape").as_::<jsbind::FrozenArray<u32>>()
    }
}
