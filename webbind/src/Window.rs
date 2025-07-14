use super::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct WindowPostMessageOptions {
    inner: emlite::Val,
}
impl FromVal for WindowPostMessageOptions {
    fn from_val(v: &emlite::Val) -> Self {
        WindowPostMessageOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for WindowPostMessageOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for WindowPostMessageOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for WindowPostMessageOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for WindowPostMessageOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<WindowPostMessageOptions> for emlite::Val {
    fn from(s: WindowPostMessageOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl WindowPostMessageOptions {
    pub fn target_origin(&self) -> jsbind::USVString {
        self.inner.get("targetOrigin").as_::<jsbind::USVString>()
    }

    pub fn set_target_origin(&mut self, value: jsbind::USVString) {
        self.inner.set("targetOrigin", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct OpenFilePickerOptions {
    inner: emlite::Val,
}
impl FromVal for OpenFilePickerOptions {
    fn from_val(v: &emlite::Val) -> Self {
        OpenFilePickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for OpenFilePickerOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for OpenFilePickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for OpenFilePickerOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for OpenFilePickerOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<OpenFilePickerOptions> for emlite::Val {
    fn from(s: OpenFilePickerOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl OpenFilePickerOptions {
    pub fn multiple(&self) -> bool {
        self.inner.get("multiple").as_::<bool>()
    }

    pub fn set_multiple(&mut self, value: bool) {
        self.inner.set("multiple", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct SaveFilePickerOptions {
    inner: emlite::Val,
}
impl FromVal for SaveFilePickerOptions {
    fn from_val(v: &emlite::Val) -> Self {
        SaveFilePickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for SaveFilePickerOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for SaveFilePickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for SaveFilePickerOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for SaveFilePickerOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<SaveFilePickerOptions> for emlite::Val {
    fn from(s: SaveFilePickerOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl SaveFilePickerOptions {
    pub fn suggested_name(&self) -> jsbind::USVString {
        self.inner.get("suggestedName").as_::<jsbind::USVString>()
    }

    pub fn set_suggested_name(&mut self, value: jsbind::USVString) {
        self.inner.set("suggestedName", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct DirectoryPickerOptions {
    inner: emlite::Val,
}
impl FromVal for DirectoryPickerOptions {
    fn from_val(v: &emlite::Val) -> Self {
        DirectoryPickerOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for DirectoryPickerOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for DirectoryPickerOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for DirectoryPickerOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for DirectoryPickerOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<DirectoryPickerOptions> for emlite::Val {
    fn from(s: DirectoryPickerOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl DirectoryPickerOptions {
    pub fn id(&self) -> jsbind::DOMString {
        self.inner.get("id").as_::<jsbind::DOMString>()
    }

    pub fn set_id(&mut self, value: jsbind::DOMString) {
        self.inner.set("id", value);
    }
}
impl DirectoryPickerOptions {
    pub fn start_in(&self) -> jsbind::Any {
        self.inner.get("startIn").as_::<jsbind::Any>()
    }

    pub fn set_start_in(&mut self, value: jsbind::Any) {
        self.inner.set("startIn", value);
    }
}
impl DirectoryPickerOptions {
    pub fn mode(&self) -> FileSystemPermissionMode {
        self.inner.get("mode").as_::<FileSystemPermissionMode>()
    }

    pub fn set_mode(&mut self, value: FileSystemPermissionMode) {
        self.inner.set("mode", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QueryOptions {
    inner: emlite::Val,
}
impl FromVal for QueryOptions {
    fn from_val(v: &emlite::Val) -> Self {
        QueryOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for QueryOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for QueryOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for QueryOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for QueryOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<QueryOptions> for emlite::Val {
    fn from(s: QueryOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl QueryOptions {
    pub fn postscript_names(&self) -> jsbind::Sequence<jsbind::DOMString> {
        self.inner
            .get("postscriptNames")
            .as_::<jsbind::Sequence<jsbind::DOMString>>()
    }

    pub fn set_postscript_names(&mut self, value: jsbind::Sequence<jsbind::DOMString>) {
        self.inner.set("postscriptNames", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct IdleRequestOptions {
    inner: emlite::Val,
}
impl FromVal for IdleRequestOptions {
    fn from_val(v: &emlite::Val) -> Self {
        IdleRequestOptions { inner: v.clone() }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for IdleRequestOptions {
    type Target = emlite::Val;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for IdleRequestOptions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for IdleRequestOptions {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for IdleRequestOptions {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<IdleRequestOptions> for emlite::Val {
    fn from(s: IdleRequestOptions) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}

impl IdleRequestOptions {
    pub fn timeout(&self) -> u32 {
        self.inner.get("timeout").as_::<u32>()
    }

    pub fn set_timeout(&mut self, value: u32) {
        self.inner.set("timeout", value);
    }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Window {
    inner: EventTarget,
}
impl FromVal for Window {
    fn from_val(v: &emlite::Val) -> Self {
        Window {
            inner: EventTarget::from_val(v),
        }
    }
    fn take_ownership(v: emlite::env::Handle) -> Self {
        Self::from_val(&emlite::Val::take_ownership(v))
    }
    fn as_handle(&self) -> emlite::env::Handle {
        self.inner.as_handle()
    }
}
impl core::ops::Deref for Window {
    type Target = EventTarget;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl core::ops::DerefMut for Window {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
impl AsRef<emlite::Val> for Window {
    fn as_ref(&self) -> &emlite::Val {
        &self.inner
    }
}
impl AsMut<emlite::Val> for Window {
    fn as_mut(&mut self) -> &mut emlite::Val {
        &mut self.inner
    }
}
impl From<Window> for emlite::Val {
    fn from(s: Window) -> emlite::Val {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        emlite::Val::take_ownership(handle)
    }
}
jsbind::utils::impl_dyn_cast!(Window);

impl Window {
    pub fn window(&self) -> jsbind::Any {
        self.inner.get("window").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn self_(&self) -> jsbind::Any {
        self.inner.get("self").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn document(&self) -> Document {
        self.inner.get("document").as_::<Document>()
    }
}
impl Window {
    pub fn name(&self) -> jsbind::DOMString {
        self.inner.get("name").as_::<jsbind::DOMString>()
    }

    pub fn set_name(&mut self, value: jsbind::DOMString) {
        self.inner.set("name", value);
    }
}
impl Window {
    pub fn location(&self) -> jsbind::Any {
        self.inner.get("location").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn history(&self) -> History {
        self.inner.get("history").as_::<History>()
    }
}
impl Window {
    pub fn navigation(&self) -> Navigation {
        self.inner.get("navigation").as_::<Navigation>()
    }
}
impl Window {
    pub fn custom_elements(&self) -> CustomElementRegistry {
        self.inner
            .get("customElements")
            .as_::<CustomElementRegistry>()
    }
}
impl Window {
    pub fn locationbar(&self) -> BarProp {
        self.inner.get("locationbar").as_::<BarProp>()
    }
}
impl Window {
    pub fn menubar(&self) -> BarProp {
        self.inner.get("menubar").as_::<BarProp>()
    }
}
impl Window {
    pub fn personalbar(&self) -> BarProp {
        self.inner.get("personalbar").as_::<BarProp>()
    }
}
impl Window {
    pub fn scrollbars(&self) -> BarProp {
        self.inner.get("scrollbars").as_::<BarProp>()
    }
}
impl Window {
    pub fn statusbar(&self) -> BarProp {
        self.inner.get("statusbar").as_::<BarProp>()
    }
}
impl Window {
    pub fn toolbar(&self) -> BarProp {
        self.inner.get("toolbar").as_::<BarProp>()
    }
}
impl Window {
    pub fn status(&self) -> jsbind::DOMString {
        self.inner.get("status").as_::<jsbind::DOMString>()
    }

    pub fn set_status(&mut self, value: jsbind::DOMString) {
        self.inner.set("status", value);
    }
}
impl Window {
    pub fn close(&self) -> jsbind::Undefined {
        self.inner.call("close", &[]).as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn closed(&self) -> bool {
        self.inner.get("closed").as_::<bool>()
    }
}
impl Window {
    pub fn stop(&self) -> jsbind::Undefined {
        self.inner.call("stop", &[]).as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn focus(&self) -> jsbind::Undefined {
        self.inner.call("focus", &[]).as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn blur(&self) -> jsbind::Undefined {
        self.inner.call("blur", &[]).as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn frames(&self) -> jsbind::Any {
        self.inner.get("frames").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl Window {
    pub fn top(&self) -> jsbind::Any {
        self.inner.get("top").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn opener(&self) -> jsbind::Any {
        self.inner.get("opener").as_::<jsbind::Any>()
    }

    pub fn set_opener(&mut self, value: jsbind::Any) {
        self.inner.set("opener", value);
    }
}
impl Window {
    pub fn parent(&self) -> jsbind::Any {
        self.inner.get("parent").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn frame_element(&self) -> Element {
        self.inner.get("frameElement").as_::<Element>()
    }
}
impl Window {
    pub fn open0(&self) -> jsbind::Any {
        self.inner.call("open", &[]).as_::<jsbind::Any>()
    }

    pub fn open1(&self, url: jsbind::USVString) -> jsbind::Any {
        self.inner.call("open", &[url.into()]).as_::<jsbind::Any>()
    }

    pub fn open2(&self, url: jsbind::USVString, target: jsbind::DOMString) -> jsbind::Any {
        self.inner
            .call("open", &[url.into(), target.into()])
            .as_::<jsbind::Any>()
    }

    pub fn open3(
        &self,
        url: jsbind::USVString,
        target: jsbind::DOMString,
        features: jsbind::DOMString,
    ) -> jsbind::Any {
        self.inner
            .call("open", &[url.into(), target.into(), features.into()])
            .as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn navigator(&self) -> Navigator {
        self.inner.get("navigator").as_::<Navigator>()
    }
}
impl Window {
    pub fn client_information(&self) -> Navigator {
        self.inner.get("clientInformation").as_::<Navigator>()
    }
}
impl Window {
    pub fn origin_agent_cluster(&self) -> bool {
        self.inner.get("originAgentCluster").as_::<bool>()
    }
}
impl Window {
    pub fn alert(&self, message: jsbind::DOMString) -> jsbind::Undefined {
        self.inner
            .call("alert", &[message.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn confirm0(&self) -> bool {
        self.inner.call("confirm", &[]).as_::<bool>()
    }

    pub fn confirm1(&self, message: jsbind::DOMString) -> bool {
        self.inner.call("confirm", &[message.into()]).as_::<bool>()
    }
}
impl Window {
    pub fn prompt0(&self) -> jsbind::DOMString {
        self.inner.call("prompt", &[]).as_::<jsbind::DOMString>()
    }

    pub fn prompt1(&self, message: jsbind::DOMString) -> jsbind::DOMString {
        self.inner
            .call("prompt", &[message.into()])
            .as_::<jsbind::DOMString>()
    }

    pub fn prompt2(
        &self,
        message: jsbind::DOMString,
        default: jsbind::DOMString,
    ) -> jsbind::DOMString {
        self.inner
            .call("prompt", &[message.into(), default.into()])
            .as_::<jsbind::DOMString>()
    }
}
impl Window {
    pub fn print(&self) -> jsbind::Undefined {
        self.inner.call("print", &[]).as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn post_message0(&self, message: jsbind::Any) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<jsbind::Undefined>()
    }

    pub fn post_message1(
        &self,
        message: jsbind::Any,
        options: WindowPostMessageOptions,
    ) -> jsbind::Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn credentialless(&self) -> bool {
        self.inner.get("credentialless").as_::<bool>()
    }
}
impl Window {
    pub fn orientation(&self) -> i16 {
        self.inner.get("orientation").as_::<i16>()
    }
}
impl Window {
    pub fn onorientationchange(&self) -> jsbind::Any {
        self.inner.get("onorientationchange").as_::<jsbind::Any>()
    }

    pub fn set_onorientationchange(&mut self, value: jsbind::Any) {
        self.inner.set("onorientationchange", value);
    }
}
impl Window {
    pub fn cookie_store(&self) -> CookieStore {
        self.inner.get("cookieStore").as_::<CookieStore>()
    }
}
impl Window {
    pub fn navigate(&self, dir: SpatialNavigationDirection) -> jsbind::Undefined {
        self.inner
            .call("navigate", &[dir.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn viewport(&self) -> Viewport {
        self.inner.get("viewport").as_::<Viewport>()
    }
}
impl Window {
    pub fn match_media(&self, query: jsbind::CSSOMString) -> MediaQueryList {
        self.inner
            .call("matchMedia", &[query.into()])
            .as_::<MediaQueryList>()
    }
}
impl Window {
    pub fn screen(&self) -> Screen {
        self.inner.get("screen").as_::<Screen>()
    }
}
impl Window {
    pub fn visual_viewport(&self) -> VisualViewport {
        self.inner.get("visualViewport").as_::<VisualViewport>()
    }
}
impl Window {
    pub fn move_to(&self, x: i32, y: i32) -> jsbind::Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn move_by(&self, x: i32, y: i32) -> jsbind::Undefined {
        self.inner
            .call("moveBy", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn resize_to(&self, width: i32, height: i32) -> jsbind::Undefined {
        self.inner
            .call("resizeTo", &[width.into(), height.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn resize_by(&self, x: i32, y: i32) -> jsbind::Undefined {
        self.inner
            .call("resizeBy", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn inner_width(&self) -> i32 {
        self.inner.get("innerWidth").as_::<i32>()
    }
}
impl Window {
    pub fn inner_height(&self) -> i32 {
        self.inner.get("innerHeight").as_::<i32>()
    }
}
impl Window {
    pub fn scroll_x(&self) -> f64 {
        self.inner.get("scrollX").as_::<f64>()
    }
}
impl Window {
    pub fn page_x_offset(&self) -> f64 {
        self.inner.get("pageXOffset").as_::<f64>()
    }
}
impl Window {
    pub fn scroll_y(&self) -> f64 {
        self.inner.get("scrollY").as_::<f64>()
    }
}
impl Window {
    pub fn page_y_offset(&self) -> f64 {
        self.inner.get("pageYOffset").as_::<f64>()
    }
}
impl Window {
    pub fn scroll(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scroll", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn scroll_to(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scrollTo", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn scroll_by(&self, x: f64, y: f64) -> jsbind::Undefined {
        self.inner
            .call("scrollBy", &[x.into(), y.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }
}
impl Window {
    pub fn screen_left(&self) -> i32 {
        self.inner.get("screenLeft").as_::<i32>()
    }
}
impl Window {
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }
}
impl Window {
    pub fn screen_top(&self) -> i32 {
        self.inner.get("screenTop").as_::<i32>()
    }
}
impl Window {
    pub fn outer_width(&self) -> i32 {
        self.inner.get("outerWidth").as_::<i32>()
    }
}
impl Window {
    pub fn outer_height(&self) -> i32 {
        self.inner.get("outerHeight").as_::<i32>()
    }
}
impl Window {
    pub fn device_pixel_ratio(&self) -> f64 {
        self.inner.get("devicePixelRatio").as_::<f64>()
    }
}
impl Window {
    pub fn get_computed_style0(&self, elt: Element) -> CSSStyleDeclaration {
        self.inner
            .call("getComputedStyle", &[elt.into()])
            .as_::<CSSStyleDeclaration>()
    }

    pub fn get_computed_style1(
        &self,
        elt: Element,
        pseudo_elt: jsbind::CSSOMString,
    ) -> CSSStyleDeclaration {
        self.inner
            .call("getComputedStyle", &[elt.into(), pseudo_elt.into()])
            .as_::<CSSStyleDeclaration>()
    }
}
impl Window {
    pub fn get_digital_goods_service(
        &self,
        service_provider: jsbind::DOMString,
    ) -> jsbind::Promise {
        self.inner
            .call("getDigitalGoodsService", &[service_provider.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn document_picture_in_picture(&self) -> DocumentPictureInPicture {
        self.inner
            .get("documentPictureInPicture")
            .as_::<DocumentPictureInPicture>()
    }
}
impl Window {
    pub fn event(&self) -> jsbind::Any {
        self.inner.get("event").as_::<jsbind::Any>()
    }
}
impl Window {
    pub fn fence(&self) -> Fence {
        self.inner.get("fence").as_::<Fence>()
    }
}
impl Window {
    pub fn show_open_file_picker0(&self) -> jsbind::Promise {
        self.inner
            .call("showOpenFilePicker", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn show_open_file_picker1(&self, options: OpenFilePickerOptions) -> jsbind::Promise {
        self.inner
            .call("showOpenFilePicker", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn show_save_file_picker0(&self) -> jsbind::Promise {
        self.inner
            .call("showSaveFilePicker", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn show_save_file_picker1(&self, options: SaveFilePickerOptions) -> jsbind::Promise {
        self.inner
            .call("showSaveFilePicker", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn show_directory_picker0(&self) -> jsbind::Promise {
        self.inner
            .call("showDirectoryPicker", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn show_directory_picker1(&self, options: DirectoryPickerOptions) -> jsbind::Promise {
        self.inner
            .call("showDirectoryPicker", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn capture_events(&self) -> jsbind::Undefined {
        self.inner
            .call("captureEvents", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn release_events(&self) -> jsbind::Undefined {
        self.inner
            .call("releaseEvents", &[])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn external(&self) -> External {
        self.inner.get("external").as_::<External>()
    }
}
impl Window {
    pub fn query_local_fonts0(&self) -> jsbind::Promise {
        self.inner
            .call("queryLocalFonts", &[])
            .as_::<jsbind::Promise>()
    }

    pub fn query_local_fonts1(&self, options: QueryOptions) -> jsbind::Promise {
        self.inner
            .call("queryLocalFonts", &[options.into()])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn onappinstalled(&self) -> jsbind::Any {
        self.inner.get("onappinstalled").as_::<jsbind::Any>()
    }

    pub fn set_onappinstalled(&mut self, value: jsbind::Any) {
        self.inner.set("onappinstalled", value);
    }
}
impl Window {
    pub fn onbeforeinstallprompt(&self) -> jsbind::Any {
        self.inner.get("onbeforeinstallprompt").as_::<jsbind::Any>()
    }

    pub fn set_onbeforeinstallprompt(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforeinstallprompt", value);
    }
}
impl Window {
    pub fn ondeviceorientation(&self) -> jsbind::Any {
        self.inner.get("ondeviceorientation").as_::<jsbind::Any>()
    }

    pub fn set_ondeviceorientation(&mut self, value: jsbind::Any) {
        self.inner.set("ondeviceorientation", value);
    }
}
impl Window {
    pub fn ondeviceorientationabsolute(&self) -> jsbind::Any {
        self.inner
            .get("ondeviceorientationabsolute")
            .as_::<jsbind::Any>()
    }

    pub fn set_ondeviceorientationabsolute(&mut self, value: jsbind::Any) {
        self.inner.set("ondeviceorientationabsolute", value);
    }
}
impl Window {
    pub fn ondevicemotion(&self) -> jsbind::Any {
        self.inner.get("ondevicemotion").as_::<jsbind::Any>()
    }

    pub fn set_ondevicemotion(&mut self, value: jsbind::Any) {
        self.inner.set("ondevicemotion", value);
    }
}
impl Window {
    pub fn portal_host(&self) -> PortalHost {
        self.inner.get("portalHost").as_::<PortalHost>()
    }
}
impl Window {
    pub fn request_idle_callback0(&self, callback: jsbind::Function) -> u32 {
        self.inner
            .call("requestIdleCallback", &[callback.into()])
            .as_::<u32>()
    }

    pub fn request_idle_callback1(
        &self,
        callback: jsbind::Function,
        options: IdleRequestOptions,
    ) -> u32 {
        self.inner
            .call("requestIdleCallback", &[callback.into(), options.into()])
            .as_::<u32>()
    }
}
impl Window {
    pub fn cancel_idle_callback(&self, handle: u32) -> jsbind::Undefined {
        self.inner
            .call("cancelIdleCallback", &[handle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn get_selection(&self) -> Selection {
        self.inner.call("getSelection", &[]).as_::<Selection>()
    }
}
impl Window {
    pub fn shared_storage(&self) -> SharedStorage {
        self.inner.get("sharedStorage").as_::<SharedStorage>()
    }
}
impl Window {
    pub fn speech_synthesis(&self) -> SpeechSynthesis {
        self.inner.get("speechSynthesis").as_::<SpeechSynthesis>()
    }
}
impl Window {
    pub fn launch_queue(&self) -> LaunchQueue {
        self.inner.get("launchQueue").as_::<LaunchQueue>()
    }
}
impl Window {
    pub fn get_screen_details(&self) -> jsbind::Promise {
        self.inner
            .call("getScreenDetails", &[])
            .as_::<jsbind::Promise>()
    }
}
impl Window {
    pub fn onbeforexrselect(&self) -> jsbind::Any {
        self.inner.get("onbeforexrselect").as_::<jsbind::Any>()
    }

    pub fn set_onbeforexrselect(&mut self, value: jsbind::Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl Window {
    pub fn onportalactivate(&self) -> jsbind::Any {
        self.inner.get("onportalactivate").as_::<jsbind::Any>()
    }

    pub fn set_onportalactivate(&mut self, value: jsbind::Any) {
        self.inner.set("onportalactivate", value);
    }
}
impl Window {
    pub fn crypto(&self) -> Crypto {
        self.inner.get("crypto").as_::<Crypto>()
    }
}
impl Window {
    pub fn request_animation_frame(&self, callback: jsbind::Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl Window {
    pub fn cancel_animation_frame(&self, handle: u32) -> jsbind::Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<jsbind::Undefined>()
    }
}
impl Window {
    pub fn session_storage(&self) -> Storage {
        self.inner.get("sessionStorage").as_::<Storage>()
    }
}
impl Window {
    pub fn local_storage(&self) -> Storage {
        self.inner.get("localStorage").as_::<Storage>()
    }
}
