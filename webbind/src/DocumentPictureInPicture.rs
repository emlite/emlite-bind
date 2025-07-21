use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPictureOptions {
    inner: Any,
}
impl FromVal for DocumentPictureInPictureOptions {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPictureOptions { inner: v.clone() }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentPictureInPictureOptions {
    type Target = Any;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPictureOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DocumentPictureInPictureOptions {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DocumentPictureInPictureOptions {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DocumentPictureInPictureOptions> for Any {
    fn from(s: DocumentPictureInPictureOptions) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DocumentPictureInPictureOptions> for Any {
    fn from(s: &DocumentPictureInPictureOptions) -> Any {
        s.inner.clone()
    }
}

impl DocumentPictureInPictureOptions {
    pub fn width(&self) -> u64 {
        self.inner.get("width").as_::<u64>()
    }

    pub fn set_width(&mut self, value: u64) {
        self.inner.set("width", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn height(&self) -> u64 {
        self.inner.get("height").as_::<u64>()
    }

    pub fn set_height(&mut self, value: u64) {
        self.inner.set("height", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn disallow_return_to_opener(&self) -> bool {
        self.inner.get("disallowReturnToOpener").as_::<bool>()
    }

    pub fn set_disallow_return_to_opener(&mut self, value: bool) {
        self.inner.set("disallowReturnToOpener", value);
    }
}
impl DocumentPictureInPictureOptions {
    pub fn prefer_initial_window_placement(&self) -> bool {
        self.inner.get("preferInitialWindowPlacement").as_::<bool>()
    }

    pub fn set_prefer_initial_window_placement(&mut self, value: bool) {
        self.inner.set("preferInitialWindowPlacement", value);
    }
}
/// The DocumentPictureInPicture class.
/// [`DocumentPictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DocumentPictureInPicture {
    inner: EventTarget,
}
impl FromVal for DocumentPictureInPicture {
    fn from_val(v: &Any) -> Self {
        DocumentPictureInPicture {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: AnyHandle) -> Self {
        Self::from_val(&Any::take_ownership(v))
    }
    fn as_handle(&self) -> AnyHandle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DocumentPictureInPicture {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DocumentPictureInPicture {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<Any> for DocumentPictureInPicture {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}
impl AsMut<Any> for DocumentPictureInPicture {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}
impl From<DocumentPictureInPicture> for Any {
    fn from(s: DocumentPictureInPicture) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}
impl From<&DocumentPictureInPicture> for Any {
    fn from(s: &DocumentPictureInPicture) -> Any {
        s.inner.clone().into()
    }
}
jsbind::utils::impl_dyn_cast!(DocumentPictureInPicture);

impl DocumentPictureInPicture {
    /// The requestWindow method.
    /// [`DocumentPictureInPicture.requestWindow`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/requestWindow)
    pub fn request_window0(&self) -> Promise<Window> {
        self.inner
            .call("requestWindow", &[])
            .as_::<Promise<Window>>()
    }
    /// The requestWindow method.
    /// [`DocumentPictureInPicture.requestWindow`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/requestWindow)
    pub fn request_window1(&self, options: &DocumentPictureInPictureOptions) -> Promise<Window> {
        self.inner
            .call("requestWindow", &[options.into()])
            .as_::<Promise<Window>>()
    }
}
impl DocumentPictureInPicture {
    /// Getter of the `window` attribute.
    /// [`DocumentPictureInPicture.window`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/window)
    pub fn window(&self) -> Window {
        self.inner.get("window").as_::<Window>()
    }
}
impl DocumentPictureInPicture {
    /// Getter of the `onenter` attribute.
    /// [`DocumentPictureInPicture.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/onenter)
    pub fn onenter(&self) -> Any {
        self.inner.get("onenter").as_::<Any>()
    }

    /// Setter of the `onenter` attribute.
    /// [`DocumentPictureInPicture.onenter`](https://developer.mozilla.org/en-US/docs/Web/API/DocumentPictureInPicture/onenter)
    pub fn set_onenter(&mut self, value: &Any) {
        self.inner.set("onenter", value);
    }
}
