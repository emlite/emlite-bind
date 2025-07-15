use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SVGTransformList {
    inner: emlite::Val,
}
impl FromVal for SVGTransformList {
    fn from_val(v: &emlite::Val) -> Self {
        SVGTransformList { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SVGTransformList {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SVGTransformList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SVGTransformList {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SVGTransformList {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<SVGTransformList> for emlite::Val {
    fn from(s: SVGTransformList) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(SVGTransformList);


impl SVGTransformList {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl SVGTransformList {
    pub fn number_of_items(&self) -> u32 {
        self.inner.get("numberOfItems").as_::<u32>()
    }

}
impl SVGTransformList {
    pub fn clear(&self, ) -> Undefined {
        self.inner.call("clear", &[]).as_::<Undefined>()
    }

}
impl SVGTransformList {
    pub fn initialize(&self, new_item: SVGTransform) -> SVGTransform {
        self.inner.call("initialize", &[new_item.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn get_item(&self, index: u32) -> SVGTransform {
        self.inner.call("getItem", &[index.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn insert_item_before(&self, new_item: SVGTransform, index: u32) -> SVGTransform {
        self.inner.call("insertItemBefore", &[new_item.into(), index.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn replace_item(&self, new_item: SVGTransform, index: u32) -> SVGTransform {
        self.inner.call("replaceItem", &[new_item.into(), index.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn remove_item(&self, index: u32) -> SVGTransform {
        self.inner.call("removeItem", &[index.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn append_item(&self, new_item: SVGTransform) -> SVGTransform {
        self.inner.call("appendItem", &[new_item.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn create_svg_transform_from_matrix0(&self, ) -> SVGTransform {
        self.inner.call("createSVGTransformFromMatrix", &[]).as_::<SVGTransform>()
    }

    pub fn create_svg_transform_from_matrix1(&self, matrix: DOMMatrix2DInit) -> SVGTransform {
        self.inner.call("createSVGTransformFromMatrix", &[matrix.into(), ]).as_::<SVGTransform>()
    }

}
impl SVGTransformList {
    pub fn consolidate(&self, ) -> SVGTransform {
        self.inner.call("consolidate", &[]).as_::<SVGTransform>()
    }

}
