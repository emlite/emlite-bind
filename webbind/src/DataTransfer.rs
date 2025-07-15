use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransfer {
    inner: emlite::Val,
}
impl FromVal for DataTransfer {
    fn from_val(v: &emlite::Val) -> Self {
        DataTransfer {
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
impl core::ops::Deref for DataTransfer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransfer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DataTransfer {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DataTransfer {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DataTransfer> for emlite::Val {
    fn from(s: DataTransfer) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(DataTransfer);

impl DataTransfer {
    pub fn new() -> DataTransfer {
        Self {
            inner: emlite::Val::global("DataTransfer")
                .new(&[])
                .as_::<emlite::Val>(),
        }
    }
}
impl DataTransfer {
    pub fn drop_effect(&self) -> DOMString {
        self.inner.get("dropEffect").as_::<DOMString>()
    }

    pub fn set_drop_effect(&mut self, value: DOMString) {
        self.inner.set("dropEffect", value);
    }
}
impl DataTransfer {
    pub fn effect_allowed(&self) -> DOMString {
        self.inner.get("effectAllowed").as_::<DOMString>()
    }

    pub fn set_effect_allowed(&mut self, value: DOMString) {
        self.inner.set("effectAllowed", value);
    }
}
impl DataTransfer {
    pub fn items(&self) -> DataTransferItemList {
        self.inner.get("items").as_::<DataTransferItemList>()
    }
}
impl DataTransfer {
    pub fn set_drag_image(&self, image: Element, x: i32, y: i32) -> Undefined {
        self.inner
            .call("setDragImage", &[image.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    pub fn types(&self) -> FrozenArray<DOMString> {
        self.inner.get("types").as_::<FrozenArray<DOMString>>()
    }
}
impl DataTransfer {
    pub fn get_data(&self, format: DOMString) -> DOMString {
        self.inner
            .call("getData", &[format.into()])
            .as_::<DOMString>()
    }
}
impl DataTransfer {
    pub fn set_data(&self, format: DOMString, data: DOMString) -> Undefined {
        self.inner
            .call("setData", &[format.into(), data.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    pub fn clear_data0(&self) -> Undefined {
        self.inner.call("clearData", &[]).as_::<Undefined>()
    }

    pub fn clear_data1(&self, format: DOMString) -> Undefined {
        self.inner
            .call("clearData", &[format.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    pub fn files(&self) -> FileList {
        self.inner.get("files").as_::<FileList>()
    }
}
