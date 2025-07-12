use super::*;

#[derive(Clone, Debug)]
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
impl std::ops::Deref for DataTransfer {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for DataTransfer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<DataTransfer> for emlite::Val {
    fn from(s: DataTransfer) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

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
    pub fn drop_effect(&self) -> jsbind::DOMString {
        self.inner.get("dropEffect").as_::<jsbind::DOMString>()
    }

    pub fn set_drop_effect(&mut self, value: jsbind::DOMString) {
        self.inner.set("dropEffect", value);
    }
}
impl DataTransfer {
    pub fn effect_allowed(&self) -> jsbind::DOMString {
        self.inner.get("effectAllowed").as_::<jsbind::DOMString>()
    }

    pub fn set_effect_allowed(&mut self, value: jsbind::DOMString) {
        self.inner.set("effectAllowed", value);
    }
}
impl DataTransfer {
    pub fn items(&self) -> DataTransferItemList {
        self.inner.get("items").as_::<DataTransferItemList>()
    }
}
impl DataTransfer {
    pub fn set_drag_image(&self, image: Element, x: i32, y: i32) -> jsbind::Undefined {
        self.inner
            .call("setDragImage", &[image.into(), x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DataTransfer {
    pub fn types(&self) -> jsbind::FrozenArray<jsbind::DOMString> {
        self.inner
            .get("types")
            .as_::<jsbind::FrozenArray<jsbind::DOMString>>()
    }
}
impl DataTransfer {
    pub fn get_data(&self, format: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("getData", &[format.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl DataTransfer {
    pub fn set_data(
        &self,
        format: jsbind::DOMString,
        data: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("setData", &[format.into(), data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DataTransfer {
    pub fn clear_data0(&self) -> jsbind::Undefined {
        self.inner.call("clearData", &[]).as_::<jsbind::Undefined>()
    }

    pub fn clear_data1(&self, format: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("clearData", &[format.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl DataTransfer {
    pub fn files(&self) -> FileList {
        self.inner.get("files").as_::<FileList>()
    }
}
