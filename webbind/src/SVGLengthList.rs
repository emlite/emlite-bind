use super::*;




/// The SVGLengthList class.
/// [`SVGLengthList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGLengthList {
    inner: Any,
}

impl FromVal for SVGLengthList {
    fn from_val(v: &Any) -> Self {
        SVGLengthList { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for SVGLengthList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGLengthList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGLengthList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGLengthList {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<SVGLengthList> for Any {
    fn from(s: SVGLengthList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGLengthList> for Any {
    fn from(s: &SVGLengthList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGLengthList);


impl SVGLengthList {
    /// Getter of the `length` attribute.
    /// [`SVGLengthList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SVGLengthList {
    /// Getter of the `numberOfItems` attribute.
    /// [`SVGLengthList.numberOfItems`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/numberOfItems)
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }

}
impl SVGLengthList {
    /// The clear method.
    /// [`SVGLengthList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/clear)
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGLengthList {
    /// The initialize method.
    /// [`SVGLengthList.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/initialize)
    pub fn initialize(&self, new_item: &SVGLength) -> SVGLength {
        self.inner.call("initialize", &[new_item.into(), ]).as_::<SVGLength>()
    }
}
impl SVGLengthList {
    /// The getItem method.
    /// [`SVGLengthList.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/getItem)
    pub fn get_item(&self, index: u32) -> SVGLength {
        self.inner.call("getItem", &[index.into(), ]).as_::<SVGLength>()
    }
}
impl SVGLengthList {
    /// The insertItemBefore method.
    /// [`SVGLengthList.insertItemBefore`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/insertItemBefore)
    pub fn insert_item_before(&self, new_item: &SVGLength, index: u32) -> SVGLength {
        self.inner.call("insertItemBefore", &[new_item.into(), index.into(), ]).as_::<SVGLength>()
    }
}
impl SVGLengthList {
    /// The replaceItem method.
    /// [`SVGLengthList.replaceItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/replaceItem)
    pub fn replace_item(&self, new_item: &SVGLength, index: u32) -> SVGLength {
        self.inner.call("replaceItem", &[new_item.into(), index.into(), ]).as_::<SVGLength>()
    }
}
impl SVGLengthList {
    /// The removeItem method.
    /// [`SVGLengthList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/removeItem)
    pub fn remove_item(&self, index: u32) -> SVGLength {
        self.inner.call("removeItem", &[index.into(), ]).as_::<SVGLength>()
    }
}
impl SVGLengthList {
    /// The appendItem method.
    /// [`SVGLengthList.appendItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGLengthList/appendItem)
    pub fn append_item(&self, new_item: &SVGLength) -> SVGLength {
        self.inner.call("appendItem", &[new_item.into(), ]).as_::<SVGLength>()
    }
}
