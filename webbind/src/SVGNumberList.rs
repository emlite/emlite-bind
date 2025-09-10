use super::*;

/// The SVGNumberList class.
/// [`SVGNumberList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGNumberList {
    inner: Any,
}

impl FromVal for SVGNumberList {
    fn from_val(v: &Any) -> Self {
        SVGNumberList {
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

impl core::ops::Deref for SVGNumberList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for SVGNumberList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for SVGNumberList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for SVGNumberList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<SVGNumberList> for Any {
    fn from(s: SVGNumberList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&SVGNumberList> for Any {
    fn from(s: &SVGNumberList) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(SVGNumberList);

impl SVGNumberList {
    /// Getter of the `length` attribute.
    /// [`SVGNumberList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGNumberList {
    /// Getter of the `numberOfItems` attribute.
    /// [`SVGNumberList.numberOfItems`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/numberOfItems)
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGNumberList {
    /// The clear method.
    /// [`SVGNumberList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGNumberList {
    /// The initialize method.
    /// [`SVGNumberList.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/initialize)
    pub fn initialize(&self, new_item: &SVGNumber) -> SVGNumber {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    /// The getItem method.
    /// [`SVGNumberList.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/getItem)
    pub fn get_item(&self, index: u32) -> SVGNumber {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    /// The insertItemBefore method.
    /// [`SVGNumberList.insertItemBefore`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/insertItemBefore)
    pub fn insert_item_before(&self, new_item: &SVGNumber, index: u32) -> SVGNumber {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    /// The replaceItem method.
    /// [`SVGNumberList.replaceItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/replaceItem)
    pub fn replace_item(&self, new_item: &SVGNumber, index: u32) -> SVGNumber {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    /// The removeItem method.
    /// [`SVGNumberList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/removeItem)
    pub fn remove_item(&self, index: u32) -> SVGNumber {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<SVGNumber>()
    }
}
impl SVGNumberList {
    /// The appendItem method.
    /// [`SVGNumberList.appendItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGNumberList/appendItem)
    pub fn append_item(&self, new_item: &SVGNumber) -> SVGNumber {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<SVGNumber>()
    }
}
