use super::*;




/// The SVGStringList class.
/// [`SVGStringList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGStringList {
    inner: Any,
}

impl FromVal for SVGStringList {
    fn from_val(v: &Any) -> Self {
        SVGStringList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGStringList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGStringList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGStringList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGStringList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGStringList> for Any {
    fn from(s: SVGStringList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGStringList> for Any {
    fn from(s: &SVGStringList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGStringList);


impl SVGStringList {
    /// Getter of the `length` attribute.
    /// [`SVGStringList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SVGStringList {
    /// Getter of the `numberOfItems` attribute.
    /// [`SVGStringList.numberOfItems`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/numberOfItems)
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }

}
impl SVGStringList {
    /// The clear method.
    /// [`SVGStringList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/clear)
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGStringList {
    /// The initialize method.
    /// [`SVGStringList.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/initialize)
    pub fn initialize(&self, new_item: &JsString) -> JsString {
        self.inner.call("initialize", &[new_item.into(), ]).as_::<JsString>()
    }
}
impl SVGStringList {
    /// The getItem method.
    /// [`SVGStringList.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/getItem)
    pub fn get_item(&self, index: u32) -> JsString {
        self.inner.call("getItem", &[index.into(), ]).as_::<JsString>()
    }
}
impl SVGStringList {
    /// The insertItemBefore method.
    /// [`SVGStringList.insertItemBefore`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/insertItemBefore)
    pub fn insert_item_before(&self, new_item: &JsString, index: u32) -> JsString {
        self.inner.call("insertItemBefore", &[new_item.into(), index.into(), ]).as_::<JsString>()
    }
}
impl SVGStringList {
    /// The replaceItem method.
    /// [`SVGStringList.replaceItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/replaceItem)
    pub fn replace_item(&self, new_item: &JsString, index: u32) -> JsString {
        self.inner.call("replaceItem", &[new_item.into(), index.into(), ]).as_::<JsString>()
    }
}
impl SVGStringList {
    /// The removeItem method.
    /// [`SVGStringList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/removeItem)
    pub fn remove_item(&self, index: u32) -> JsString {
        self.inner.call("removeItem", &[index.into(), ]).as_::<JsString>()
    }
}
impl SVGStringList {
    /// The appendItem method.
    /// [`SVGStringList.appendItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGStringList/appendItem)
    pub fn append_item(&self, new_item: &JsString) -> JsString {
        self.inner.call("appendItem", &[new_item.into(), ]).as_::<JsString>()
    }
}
