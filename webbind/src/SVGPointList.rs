use super::*;




/// The SVGPointList class.
/// [`SVGPointList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGPointList {
    inner: Any,
}

impl FromVal for SVGPointList {
    fn from_val(v: &Any) -> Self {
        SVGPointList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGPointList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGPointList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGPointList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGPointList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGPointList> for Any {
    fn from(s: SVGPointList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGPointList> for Any {
    fn from(s: &SVGPointList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGPointList);


impl SVGPointList {
    /// Getter of the `length` attribute.
    /// [`SVGPointList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SVGPointList {
    /// Getter of the `numberOfItems` attribute.
    /// [`SVGPointList.numberOfItems`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/numberOfItems)
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }

}
impl SVGPointList {
    /// The clear method.
    /// [`SVGPointList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/clear)
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGPointList {
    /// The initialize method.
    /// [`SVGPointList.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/initialize)
    pub fn initialize(&self, new_item: &DOMPoint) -> DOMPoint {
        self.inner.call("initialize", &[new_item.into(), ]).as_::<DOMPoint>()
    }
}
impl SVGPointList {
    /// The getItem method.
    /// [`SVGPointList.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/getItem)
    pub fn get_item(&self, index: u32) -> DOMPoint {
        self.inner.call("getItem", &[index.into(), ]).as_::<DOMPoint>()
    }
}
impl SVGPointList {
    /// The insertItemBefore method.
    /// [`SVGPointList.insertItemBefore`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/insertItemBefore)
    pub fn insert_item_before(&self, new_item: &DOMPoint, index: u32) -> DOMPoint {
        self.inner.call("insertItemBefore", &[new_item.into(), index.into(), ]).as_::<DOMPoint>()
    }
}
impl SVGPointList {
    /// The replaceItem method.
    /// [`SVGPointList.replaceItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/replaceItem)
    pub fn replace_item(&self, new_item: &DOMPoint, index: u32) -> DOMPoint {
        self.inner.call("replaceItem", &[new_item.into(), index.into(), ]).as_::<DOMPoint>()
    }
}
impl SVGPointList {
    /// The removeItem method.
    /// [`SVGPointList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/removeItem)
    pub fn remove_item(&self, index: u32) -> DOMPoint {
        self.inner.call("removeItem", &[index.into(), ]).as_::<DOMPoint>()
    }
}
impl SVGPointList {
    /// The appendItem method.
    /// [`SVGPointList.appendItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGPointList/appendItem)
    pub fn append_item(&self, new_item: &DOMPoint) -> DOMPoint {
        self.inner.call("appendItem", &[new_item.into(), ]).as_::<DOMPoint>()
    }
}
