use super::*;




#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct History {
    inner: emlite::Val,
}
impl FromVal for History {
    fn from_val(v: &emlite::Val) -> Self {
        History { inner: emlite::Val::from_val(v) }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for History {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for History {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for History {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for History {
    fn as_mut(&mut self) -> &mut emlite::Val {
      &mut self.inner
  }
}
impl From<History> for emlite::Val {
    fn from(s: History) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(History);


impl History {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }

}
impl History {
    pub fn scroll_restoration(&self) -> ScrollRestoration {
        self.inner.get("scrollRestoration").as_::<ScrollRestoration>()
    }

    pub fn set_scroll_restoration(&mut self, value: ScrollRestoration) {
        self.inner.set("scrollRestoration", value);
    }

}
impl History {
    pub fn state(&self) -> Any {
        self.inner.get("state").as_::<Any>()
    }

}
impl History {
    pub fn go0(&self, ) -> Undefined {
        self.inner.call("go", &[]).as_::<Undefined>()
    }

    pub fn go1(&self, delta: i32) -> Undefined {
        self.inner.call("go", &[delta.into(), ]).as_::<Undefined>()
    }

}
impl History {
    pub fn back(&self, ) -> Undefined {
        self.inner.call("back", &[]).as_::<Undefined>()
    }

}
impl History {
    pub fn forward(&self, ) -> Undefined {
        self.inner.call("forward", &[]).as_::<Undefined>()
    }

}
impl History {
    pub fn push_state0(&self, data: Any, unused: DOMString) -> Undefined {
        self.inner.call("pushState", &[data.into(), unused.into(), ]).as_::<Undefined>()
    }

    pub fn push_state1(&self, data: Any, unused: DOMString, url: USVString) -> Undefined {
        self.inner.call("pushState", &[data.into(), unused.into(), url.into(), ]).as_::<Undefined>()
    }

}
impl History {
    pub fn replace_state0(&self, data: Any, unused: DOMString) -> Undefined {
        self.inner.call("replaceState", &[data.into(), unused.into(), ]).as_::<Undefined>()
    }

    pub fn replace_state1(&self, data: Any, unused: DOMString, url: USVString) -> Undefined {
        self.inner.call("replaceState", &[data.into(), unused.into(), url.into(), ]).as_::<Undefined>()
    }

}
