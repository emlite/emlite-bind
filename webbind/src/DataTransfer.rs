use super::*;

/// The DataTransfer class.
/// [`DataTransfer`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransfer {
    inner: Any,
}
impl FromVal for DataTransfer {
    fn from_val(v: &Any) -> Self {
        DataTransfer {
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
impl core::ops::Deref for DataTransfer {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransfer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DataTransfer {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DataTransfer {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DataTransfer> for Any {
    fn from(s: DataTransfer) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DataTransfer> for Any {
    fn from(s: &DataTransfer) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataTransfer);

impl DataTransfer {
    /// The `new DataTransfer(..)` constructor, creating a new DataTransfer instance
    pub fn new() -> DataTransfer {
        Self {
            inner: Any::global("DataTransfer").new(&[]).as_::<Any>(),
        }
    }
}
impl DataTransfer {
    /// Getter of the `dropEffect` attribute.
    /// [`DataTransfer.dropEffect`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)
    pub fn drop_effect(&self) -> String {
        self.inner.get("dropEffect").as_::<String>()
    }

    /// Setter of the `dropEffect` attribute.
    /// [`DataTransfer.dropEffect`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/dropEffect)
    pub fn set_drop_effect(&mut self, value: &str) {
        self.inner.set("dropEffect", value);
    }
}
impl DataTransfer {
    /// Getter of the `effectAllowed` attribute.
    /// [`DataTransfer.effectAllowed`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)
    pub fn effect_allowed(&self) -> String {
        self.inner.get("effectAllowed").as_::<String>()
    }

    /// Setter of the `effectAllowed` attribute.
    /// [`DataTransfer.effectAllowed`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/effectAllowed)
    pub fn set_effect_allowed(&mut self, value: &str) {
        self.inner.set("effectAllowed", value);
    }
}
impl DataTransfer {
    /// Getter of the `items` attribute.
    /// [`DataTransfer.items`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/items)
    pub fn items(&self) -> DataTransferItemList {
        self.inner.get("items").as_::<DataTransferItemList>()
    }
}
impl DataTransfer {
    /// The setDragImage method.
    /// [`DataTransfer.setDragImage`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setDragImage)
    pub fn set_drag_image(&self, image: &Element, x: i32, y: i32) -> Undefined {
        self.inner
            .call("setDragImage", &[image.into(), x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    /// Getter of the `types` attribute.
    /// [`DataTransfer.types`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/types)
    pub fn types(&self) -> FrozenArray<String> {
        self.inner.get("types").as_::<FrozenArray<String>>()
    }
}
impl DataTransfer {
    /// The getData method.
    /// [`DataTransfer.getData`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/getData)
    pub fn get_data(&self, format: &str) -> String {
        self.inner.call("getData", &[format.into()]).as_::<String>()
    }
}
impl DataTransfer {
    /// The setData method.
    /// [`DataTransfer.setData`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/setData)
    pub fn set_data(&self, format: &str, data: &str) -> Undefined {
        self.inner
            .call("setData", &[format.into(), data.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    /// The clearData method.
    /// [`DataTransfer.clearData`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)
    pub fn clear_data0(&self) -> Undefined {
        self.inner.call("clearData", &[]).as_::<Undefined>()
    }
    /// The clearData method.
    /// [`DataTransfer.clearData`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/clearData)
    pub fn clear_data1(&self, format: &str) -> Undefined {
        self.inner
            .call("clearData", &[format.into()])
            .as_::<Undefined>()
    }
}
impl DataTransfer {
    /// Getter of the `files` attribute.
    /// [`DataTransfer.files`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransfer/files)
    pub fn files(&self) -> FileList {
        self.inner.get("files").as_::<FileList>()
    }
}
