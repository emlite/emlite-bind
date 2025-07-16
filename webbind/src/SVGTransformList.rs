use super::*;

/// The SVGTransformList class.
/// [`SVGTransformList`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTransformList {
    inner: Any,
}
impl FromVal for SVGTransformList {
    fn from_val(v: &Any) -> Self {
        SVGTransformList {
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
impl core::ops::Deref for SVGTransformList {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGTransformList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for SVGTransformList {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for SVGTransformList {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<SVGTransformList> for Any {
    fn from(s: SVGTransformList) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&SVGTransformList> for Any {
    fn from(s: &SVGTransformList) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(SVGTransformList);

impl SVGTransformList {
    /// Getter of the `length` attribute.
    /// [`SVGTransformList.length`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl SVGTransformList {
    /// Getter of the `numberOfItems` attribute.
    /// [`SVGTransformList.numberOfItems`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/numberOfItems)
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }
}
impl SVGTransformList {
    /// The clear method.
    /// [`SVGTransformList.clear`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/clear)
    pub fn clear(&self) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }
}
impl SVGTransformList {
    /// The initialize method.
    /// [`SVGTransformList.initialize`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/initialize)
    pub fn initialize(&self, new_item: &SVGTransform) -> SVGTransform {
        self.inner
            .call("initialize", &[new_item.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The getItem method.
    /// [`SVGTransformList.getItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/getItem)
    pub fn get_item(&self, index: u32) -> SVGTransform {
        self.inner
            .call("getItem", &[index.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The insertItemBefore method.
    /// [`SVGTransformList.insertItemBefore`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/insertItemBefore)
    pub fn insert_item_before(&self, new_item: &SVGTransform, index: u32) -> SVGTransform {
        self.inner
            .call("insertItemBefore", &[new_item.into(), index.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The replaceItem method.
    /// [`SVGTransformList.replaceItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/replaceItem)
    pub fn replace_item(&self, new_item: &SVGTransform, index: u32) -> SVGTransform {
        self.inner
            .call("replaceItem", &[new_item.into(), index.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The removeItem method.
    /// [`SVGTransformList.removeItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/removeItem)
    pub fn remove_item(&self, index: u32) -> SVGTransform {
        self.inner
            .call("removeItem", &[index.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The appendItem method.
    /// [`SVGTransformList.appendItem`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/appendItem)
    pub fn append_item(&self, new_item: &SVGTransform) -> SVGTransform {
        self.inner
            .call("appendItem", &[new_item.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The createSVGTransformFromMatrix method.
    /// [`SVGTransformList.createSVGTransformFromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)
    pub fn create_svg_transform_from_matrix0(&self) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[])
            .as_::<SVGTransform>()
    }
    /// The createSVGTransformFromMatrix method.
    /// [`SVGTransformList.createSVGTransformFromMatrix`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/createSVGTransformFromMatrix)
    pub fn create_svg_transform_from_matrix1(&self, matrix: &DOMMatrix2DInit) -> SVGTransform {
        self.inner
            .call("createSVGTransformFromMatrix", &[matrix.into()])
            .as_::<SVGTransform>()
    }
}
impl SVGTransformList {
    /// The consolidate method.
    /// [`SVGTransformList.consolidate`](https://developer.mozilla.org/en-US/docs/Web/API/SVGTransformList/consolidate)
    pub fn consolidate(&self) -> SVGTransform {
        self.inner.call("consolidate", &[]).as_::<SVGTransform>()
    }
}
