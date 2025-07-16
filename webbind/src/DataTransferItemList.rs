use super::*;

/// The DataTransferItemList class.
/// [`DataTransferItemList`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DataTransferItemList {
    inner: Any,
}
impl FromVal for DataTransferItemList {
    fn from_val(v: &Any) -> Self {
        DataTransferItemList {
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
impl core::ops::Deref for DataTransferItemList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DataTransferItemList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DataTransferItemList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DataTransferItemList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DataTransferItemList> for Any {
    fn from(s: DataTransferItemList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DataTransferItemList> for Any {
    fn from(s: &DataTransferItemList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DataTransferItemList);

impl DataTransferItemList {
    /// Getter of the `length` attribute.
    /// [`DataTransferItemList.length`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl DataTransferItemList {
    /// The add method.
    /// [`DataTransferItemList.add`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/add)
    pub fn add(&self, data: &File) -> DataTransferItem {
        self.inner
            .call("add", &[data.into()])
            .as_::<DataTransferItem>()
    }
}
impl DataTransferItemList {
    /// The remove method.
    /// [`DataTransferItemList.remove`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/remove)
    pub fn remove(&self, index: u32) -> Undefined {
        self.inner
            .call("remove", &[index.into()])
            .as_::<Undefined>()
    }
}
impl DataTransferItemList {
    /// The clear method.
    /// [`DataTransferItemList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/DataTransferItemList/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
