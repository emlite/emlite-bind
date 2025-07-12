use super::*;

#[derive(Clone, Debug)]
pub struct CharacterData {
    inner: Node,
}
impl FromVal for CharacterData {
    fn from_val(v: &emlite::Val) -> Self {
        CharacterData {
            inner: Node::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl std::ops::Deref for CharacterData {
    type Target = Node;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl std::ops::DerefMut for CharacterData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl From<CharacterData> for emlite::Val {
    fn from(s: CharacterData) -> emlite::Val {
        let handle = s.inner.as_handle();
        std::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl CharacterData {
    pub fn data(&self) -> jsbind::DOMString {
        self.inner.get("data").as_::<jsbind::DOMString>()
    }

    pub fn set_data(&mut self, value: jsbind::DOMString) {
        self.inner.set("data", value);
    }
}
impl CharacterData {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl CharacterData {
    pub fn substring_data(&self, offset: u32, count: u32) -> jsbind::DOMString {
        self.inner
            .call("substringData", &[offset.into(), count.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl CharacterData {
    pub fn append_data(&self, data: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("appendData", &[data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn insert_data(&self, offset: u32, data: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("insertData", &[offset.into(), data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn delete_data(&self, offset: u32, count: u32) -> jsbind::Undefined {
        self.inner
            .call("deleteData", &[offset.into(), count.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn replace_data(
        &self,
        offset: u32,
        count: u32,
        data: jsbind::DOMString,
    ) -> jsbind::Undefined {
        self.inner
            .call("replaceData", &[offset.into(), count.into(), data.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn previous_element_sibling(&self) -> Element {
        self.inner.get("previousElementSibling").as_::<Element>()
    }
}
impl CharacterData {
    pub fn next_element_sibling(&self) -> Element {
        self.inner.get("nextElementSibling").as_::<Element>()
    }
}
impl CharacterData {
    pub fn before(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("before", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn after(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("after", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn replace_with(&self, nodes: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("replaceWith", &[nodes.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl CharacterData {
    pub fn remove(&self) -> jsbind::Undefined {
        self.inner.call("remove", &[]).as_::<jsbind::Undefined>()
    }
}
