use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct HTMLOptionsCollection {
    inner: HTMLCollection,
}
impl FromVal for HTMLOptionsCollection {
    fn from_val(v: &emlite::Val) -> Self {
        HTMLOptionsCollection { inner: HTMLCollection::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for HTMLOptionsCollection {
    type Target = HTMLCollection;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for HTMLOptionsCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for HTMLOptionsCollection {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for HTMLOptionsCollection {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<HTMLOptionsCollection> for emlite::Val {
    fn from(s: HTMLOptionsCollection) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(HTMLOptionsCollection);


impl HTMLOptionsCollection {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

    pub fn set_length(&mut self, value: u32) {
        self.inner.set("length", value);
    }

}
impl HTMLOptionsCollection {
    pub fn add0(&self, element: Any) -> Undefined {
        self.inner.call("add", &[element.into(), ]).as_::<Undefined>()
    }

    pub fn add1(&self, element: Any, before: Any) -> Undefined {
        self.inner.call("add", &[element.into(), before.into(), ]).as_::<Undefined>()
    }

}
impl HTMLOptionsCollection {
    pub fn remove(&self, index: i32) -> Undefined {
        self.inner.call("remove", &[index.into(), ]).as_::<Undefined>()
    }

}
impl HTMLOptionsCollection {
    pub fn selected_index(&self) -> i32 {
        self.inner.get("selectedIndex").as_::<i32>()
    }

    pub fn set_selected_index(&mut self, value: i32) {
        self.inner.set("selectedIndex", value);
    }

}
