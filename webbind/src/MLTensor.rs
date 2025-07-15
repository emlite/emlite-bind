use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct MLTensor {
    inner: emlite::Val,
}
impl FromVal for MLTensor {
    fn from_val(v: &emlite::Val) -> Self {
        MLTensor {
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
impl core::ops::Deref for MLTensor {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for MLTensor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for MLTensor {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for MLTensor {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<MLTensor> for emlite::Val {
    fn from(s: MLTensor) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(MLTensor);

impl MLTensor {
    pub fn data_type(&self) -> MLOperandDataType {
        self.inner.get("dataType").as_::<MLOperandDataType>()
    }
}
impl MLTensor {
    pub fn shape(&self) -> FrozenArray<u32> {
        self.inner.get("shape").as_::<FrozenArray<u32>>()
    }
}
impl MLTensor {
    pub fn readable(&self) -> bool {
        self.inner.get("readable").as_::<bool>()
    }
}
impl MLTensor {
    pub fn writable(&self) -> bool {
        self.inner.get("writable").as_::<bool>()
    }
}
impl MLTensor {
    pub fn constant(&self) -> bool {
        self.inner.get("constant").as_::<bool>()
    }
}
impl MLTensor {
    pub fn destroy(&self) -> Undefined {
        self.inner.call("destroy", &[]).as_::<Undefined>()
    }
}
