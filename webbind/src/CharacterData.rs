use super::*;




/// The CharacterData class.
/// [`CharacterData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct CharacterData {
    inner: Node,
}

impl FromVal for CharacterData {
    fn from_val(v: &Any) -> Self {
        CharacterData { inner: Node::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for CharacterData {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for CharacterData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for CharacterData {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for CharacterData {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<CharacterData> for Any {
    fn from(s: CharacterData) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&CharacterData> for Any {
    fn from(s: &CharacterData) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(CharacterData);


impl CharacterData {
    /// Getter of the `data` attribute.
    /// [`CharacterData.data`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)
    pub fn data(&self) -> JsString {
        self.inner.get("data").as_::<JsString>()
    }

    /// Setter of the `data` attribute.
    /// [`CharacterData.data`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/data)
    pub fn set_data(&mut self, value: &JsString) {
        self.inner.set("data", value);
    }
}
impl CharacterData {
    /// Getter of the `length` attribute.
    /// [`CharacterData.length`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl CharacterData {
    /// The substringData method.
    /// [`CharacterData.substringData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/substringData)
    pub fn substring_data(&self, offset: u32, count: u32) -> JsString {
        self.inner.call("substringData", &[offset.into(), count.into(), ]).as_::<JsString>()
    }
}
impl CharacterData {
    /// The appendData method.
    /// [`CharacterData.appendData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/appendData)
    pub fn append_data(&self, data: &JsString) -> Undefined {
        self.inner.call("appendData", &[data.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The insertData method.
    /// [`CharacterData.insertData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/insertData)
    pub fn insert_data(&self, offset: u32, data: &JsString) -> Undefined {
        self.inner.call("insertData", &[offset.into(), data.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The deleteData method.
    /// [`CharacterData.deleteData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/deleteData)
    pub fn delete_data(&self, offset: u32, count: u32) -> Undefined {
        self.inner.call("deleteData", &[offset.into(), count.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The replaceData method.
    /// [`CharacterData.replaceData`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceData)
    pub fn replace_data(&self, offset: u32, count: u32, data: &JsString) -> Undefined {
        self.inner.call("replaceData", &[offset.into(), count.into(), data.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// Getter of the `previousElementSibling` attribute.
    /// [`CharacterData.previousElementSibling`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/previousElementSibling)
    pub fn previous_element_sibling(&self) -> Element {
        self.inner.get("previousElementSibling").as_::<Element>()
    }

}
impl CharacterData {
    /// Getter of the `nextElementSibling` attribute.
    /// [`CharacterData.nextElementSibling`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/nextElementSibling)
    pub fn next_element_sibling(&self) -> Element {
        self.inner.get("nextElementSibling").as_::<Element>()
    }

}
impl CharacterData {
    /// The before method.
    /// [`CharacterData.before`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/before)
    pub fn before(&self, nodes: &Any) -> Undefined {
        self.inner.call("before", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The after method.
    /// [`CharacterData.after`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/after)
    pub fn after(&self, nodes: &Any) -> Undefined {
        self.inner.call("after", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The replaceWith method.
    /// [`CharacterData.replaceWith`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/replaceWith)
    pub fn replace_with(&self, nodes: &Any) -> Undefined {
        self.inner.call("replaceWith", &[nodes.into(), ]).as_::<Undefined>()
    }
}
impl CharacterData {
    /// The remove method.
    /// [`CharacterData.remove`](https://developer.mozilla.org/en-US/docs/Web/API/CharacterData/remove)
    pub fn remove(&self, ) -> Undefined {
        self.inner.call("remove", &[]).as_::<Undefined>()
    }
}
