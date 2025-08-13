use super::*;




/// The Screen class.
/// [`Screen`](https://developer.mozilla.org/en-US/docs/Web/API/Screen)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Screen {
    inner: Any,
}

impl FromVal for Screen {
    fn from_val(v: &Any) -> Self {
        Screen { inner: Any::from_val(v) }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}

impl core::ops::Deref for Screen {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl core::ops::DerefMut for Screen {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl AsRef<Any> for Screen {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Screen {
    fn as_mut(&mut self) -> &mut Any {
      &mut self.inner
  }
}

impl From<Screen> for Any {
    fn from(s: Screen) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Screen> for Any {
    fn from(s: &Screen) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Screen);


impl Screen {
    /// Getter of the `availWidth` attribute.
    /// [`Screen.availWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availWidth)
    pub fn avail_width(&self) -> i32 {
        self.inner.get("availWidth").as_::<i32>()
    }

}
impl Screen {
    /// Getter of the `availHeight` attribute.
    /// [`Screen.availHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/availHeight)
    pub fn avail_height(&self) -> i32 {
        self.inner.get("availHeight").as_::<i32>()
    }

}
impl Screen {
    /// Getter of the `width` attribute.
    /// [`Screen.width`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/width)
    pub fn width(&self) -> i32 {
        self.inner.get("width").as_::<i32>()
    }

}
impl Screen {
    /// Getter of the `height` attribute.
    /// [`Screen.height`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/height)
    pub fn height(&self) -> i32 {
        self.inner.get("height").as_::<i32>()
    }

}
impl Screen {
    /// Getter of the `colorDepth` attribute.
    /// [`Screen.colorDepth`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/colorDepth)
    pub fn color_depth(&self) -> u32 {
        self.inner.get("colorDepth").as_::<u32>()
    }

}
impl Screen {
    /// Getter of the `pixelDepth` attribute.
    /// [`Screen.pixelDepth`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/pixelDepth)
    pub fn pixel_depth(&self) -> u32 {
        self.inner.get("pixelDepth").as_::<u32>()
    }

}
impl Screen {
    /// Getter of the `orientation` attribute.
    /// [`Screen.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/orientation)
    pub fn orientation(&self) -> ScreenOrientation {
        self.inner.get("orientation").as_::<ScreenOrientation>()
    }

}
impl Screen {
    /// Getter of the `isExtended` attribute.
    /// [`Screen.isExtended`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/isExtended)
    pub fn is_extended(&self) -> bool {
        self.inner.get("isExtended").as_::<bool>()
    }

}
impl Screen {
    /// Getter of the `onchange` attribute.
    /// [`Screen.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)
    pub fn onchange(&self) -> Any {
        self.inner.get("onchange").as_::<Any>()
    }

    /// Setter of the `onchange` attribute.
    /// [`Screen.onchange`](https://developer.mozilla.org/en-US/docs/Web/API/Screen/onchange)
    pub fn set_onchange(&mut self, value: &Any) {
        self.inner.set("onchange", value);
    }
}
