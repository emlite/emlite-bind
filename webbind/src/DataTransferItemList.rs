use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransferItemList {
    inner: emlite::Val,
}
impl FromVal for DataTransferItemList {
    fn from_val(v: &emlite::Val) -> Self {
        DataTransferItemList {
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
impl core::ops::Deref for DataTransferItemList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransferItemList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DataTransferItemList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DataTransferItemList {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DataTransferItemList> for emlite::Val {
    fn from(s: DataTransferItemList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
impl From<&DataTransferItemList> for emlite::Val {
    fn from(s: &DataTransferItemList) -> emlite::Val {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataTransferItemList);

impl DataTransferItemList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DataTransferItemList {
    pub fn add(&self, data: File) -> DataTransferItem {
        self.inner
            .call("add", &[data.into()])
            .as_::<DataTransferItem>()
    }
}
impl DataTransferItemList {
    pub fn remove(&self, index: u32) -> Undefined {
        self.inner
            .call("remove", &[index.into()])
            .as_::<Undefined>()
    }
}
impl DataTransferItemList {
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
