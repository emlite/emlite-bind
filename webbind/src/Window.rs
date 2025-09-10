use super::*;

/// The Window class.
/// [`Window`](https://developer.mozilla.org/en-US/docs/Web/API/Window)
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Window {
    inner: EventTarget,
}

impl FromVal for Window {
    fn from_val(v: &Any) -> Self {
        Window {
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

impl AsRef<Any> for Window {
    fn as_ref(&self) -> &Any {
        &self.inner
    }
}

impl AsMut<Any> for Window {
    fn as_mut(&mut self) -> &mut Any {
        &mut self.inner
    }
}

impl From<Window> for Any {
    fn from(s: Window) -> Any {
        let handle = s.inner.as_handle();
        core::mem::forget(s);
        Any::take_ownership(handle)
    }
}

impl From<&Window> for Any {
    fn from(s: &Window) -> Any {
        s.inner.clone().into()
    }
}

jsbind::utils::impl_dyn_cast!(Window);

impl Window {
    /// Getter of the `window` attribute.
    /// [`Window.window`](https://developer.mozilla.org/en-US/docs/Web/API/Window/window)
    pub fn window(&self) -> Any {
        self.inner.get("window").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `self` attribute.
    /// [`Window.self`](https://developer.mozilla.org/en-US/docs/Web/API/Window/self)
    pub fn self_(&self) -> Any {
        self.inner.get("self").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `document` attribute.
    /// [`Window.document`](https://developer.mozilla.org/en-US/docs/Web/API/Window/document)
    pub fn document(&self) -> Document {
        self.inner.get("document").as_::<Document>()
    }
}
impl Window {
    /// Getter of the `name` attribute.
    /// [`Window.name`](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)
    pub fn name(&self) -> JsString {
        self.inner.get("name").as_::<JsString>()
    }

    /// Setter of the `name` attribute.
    /// [`Window.name`](https://developer.mozilla.org/en-US/docs/Web/API/Window/name)
    pub fn set_name(&mut self, value: &JsString) {
        self.inner.set("name", value);
    }
}
impl Window {
    /// Getter of the `location` attribute.
    /// [`Window.location`](https://developer.mozilla.org/en-US/docs/Web/API/Window/location)
    pub fn location(&self) -> Location {
        self.inner.get("location").as_::<Location>()
    }
}
impl Window {
    /// Getter of the `history` attribute.
    /// [`Window.history`](https://developer.mozilla.org/en-US/docs/Web/API/Window/history)
    pub fn history(&self) -> History {
        self.inner.get("history").as_::<History>()
    }
}
impl Window {
    /// Getter of the `navigation` attribute.
    /// [`Window.navigation`](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigation)
    pub fn navigation(&self) -> Navigation {
        self.inner.get("navigation").as_::<Navigation>()
    }
}
impl Window {
    /// Getter of the `customElements` attribute.
    /// [`Window.customElements`](https://developer.mozilla.org/en-US/docs/Web/API/Window/customElements)
    pub fn custom_elements(&self) -> CustomElementRegistry {
        self.inner
            .get("customElements")
            .as_::<CustomElementRegistry>()
    }
}
impl Window {
    /// Getter of the `locationbar` attribute.
    /// [`Window.locationbar`](https://developer.mozilla.org/en-US/docs/Web/API/Window/locationbar)
    pub fn locationbar(&self) -> BarProp {
        self.inner.get("locationbar").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `menubar` attribute.
    /// [`Window.menubar`](https://developer.mozilla.org/en-US/docs/Web/API/Window/menubar)
    pub fn menubar(&self) -> BarProp {
        self.inner.get("menubar").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `personalbar` attribute.
    /// [`Window.personalbar`](https://developer.mozilla.org/en-US/docs/Web/API/Window/personalbar)
    pub fn personalbar(&self) -> BarProp {
        self.inner.get("personalbar").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `scrollbars` attribute.
    /// [`Window.scrollbars`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollbars)
    pub fn scrollbars(&self) -> BarProp {
        self.inner.get("scrollbars").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `statusbar` attribute.
    /// [`Window.statusbar`](https://developer.mozilla.org/en-US/docs/Web/API/Window/statusbar)
    pub fn statusbar(&self) -> BarProp {
        self.inner.get("statusbar").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `toolbar` attribute.
    /// [`Window.toolbar`](https://developer.mozilla.org/en-US/docs/Web/API/Window/toolbar)
    pub fn toolbar(&self) -> BarProp {
        self.inner.get("toolbar").as_::<BarProp>()
    }
}
impl Window {
    /// Getter of the `status` attribute.
    /// [`Window.status`](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)
    pub fn status(&self) -> JsString {
        self.inner.get("status").as_::<JsString>()
    }

    /// Setter of the `status` attribute.
    /// [`Window.status`](https://developer.mozilla.org/en-US/docs/Web/API/Window/status)
    pub fn set_status(&mut self, value: &JsString) {
        self.inner.set("status", value);
    }
}
impl Window {
    /// The close method.
    /// [`Window.close`](https://developer.mozilla.org/en-US/docs/Web/API/Window/close)
    pub fn close(&self) -> Undefined {
        self.inner.call("close", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `closed` attribute.
    /// [`Window.closed`](https://developer.mozilla.org/en-US/docs/Web/API/Window/closed)
    pub fn closed(&self) -> bool {
        self.inner.get("closed").as_::<bool>()
    }
}
impl Window {
    /// The stop method.
    /// [`Window.stop`](https://developer.mozilla.org/en-US/docs/Web/API/Window/stop)
    pub fn stop(&self) -> Undefined {
        self.inner.call("stop", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// The focus method.
    /// [`Window.focus`](https://developer.mozilla.org/en-US/docs/Web/API/Window/focus)
    pub fn focus(&self) -> Undefined {
        self.inner.call("focus", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// The blur method.
    /// [`Window.blur`](https://developer.mozilla.org/en-US/docs/Web/API/Window/blur)
    pub fn blur(&self) -> Undefined {
        self.inner.call("blur", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `frames` attribute.
    /// [`Window.frames`](https://developer.mozilla.org/en-US/docs/Web/API/Window/frames)
    pub fn frames(&self) -> Any {
        self.inner.get("frames").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `length` attribute.
    /// [`Window.length`](https://developer.mozilla.org/en-US/docs/Web/API/Window/length)
    pub fn length(&self) -> u32 {
        self.inner.get("length").as_::<u32>()
    }
}
impl Window {
    /// Getter of the `top` attribute.
    /// [`Window.top`](https://developer.mozilla.org/en-US/docs/Web/API/Window/top)
    pub fn top(&self) -> Any {
        self.inner.get("top").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `opener` attribute.
    /// [`Window.opener`](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)
    pub fn opener(&self) -> Any {
        self.inner.get("opener").as_::<Any>()
    }

    /// Setter of the `opener` attribute.
    /// [`Window.opener`](https://developer.mozilla.org/en-US/docs/Web/API/Window/opener)
    pub fn set_opener(&mut self, value: &Any) {
        self.inner.set("opener", value);
    }
}
impl Window {
    /// Getter of the `parent` attribute.
    /// [`Window.parent`](https://developer.mozilla.org/en-US/docs/Web/API/Window/parent)
    pub fn parent(&self) -> Any {
        self.inner.get("parent").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `frameElement` attribute.
    /// [`Window.frameElement`](https://developer.mozilla.org/en-US/docs/Web/API/Window/frameElement)
    pub fn frame_element(&self) -> Element {
        self.inner.get("frameElement").as_::<Element>()
    }
}
impl Window {
    /// The open method.
    /// [`Window.open`](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    pub fn open0(&self) -> Any {
        self.inner.call("open", &[]).as_::<Any>()
    }
    /// The open method.
    /// [`Window.open`](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    pub fn open1(&self, url: &JsString) -> Any {
        self.inner.call("open", &[url.into()]).as_::<Any>()
    }
    /// The open method.
    /// [`Window.open`](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    pub fn open2(&self, url: &JsString, target: &JsString) -> Any {
        self.inner
            .call("open", &[url.into(), target.into()])
            .as_::<Any>()
    }
    /// The open method.
    /// [`Window.open`](https://developer.mozilla.org/en-US/docs/Web/API/Window/open)
    pub fn open3(&self, url: &JsString, target: &JsString, features: &JsString) -> Any {
        self.inner
            .call("open", &[url.into(), target.into(), features.into()])
            .as_::<Any>()
    }
}
impl Window {
    /// Getter of the `navigator` attribute.
    /// [`Window.navigator`](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigator)
    pub fn navigator(&self) -> Navigator {
        self.inner.get("navigator").as_::<Navigator>()
    }
}
impl Window {
    /// Getter of the `clientInformation` attribute.
    /// [`Window.clientInformation`](https://developer.mozilla.org/en-US/docs/Web/API/Window/clientInformation)
    pub fn client_information(&self) -> Navigator {
        self.inner.get("clientInformation").as_::<Navigator>()
    }
}
impl Window {
    /// Getter of the `originAgentCluster` attribute.
    /// [`Window.originAgentCluster`](https://developer.mozilla.org/en-US/docs/Web/API/Window/originAgentCluster)
    pub fn origin_agent_cluster(&self) -> bool {
        self.inner.get("originAgentCluster").as_::<bool>()
    }
}
impl Window {
    /// The alert method.
    /// [`Window.alert`](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
    pub fn alert(&self, message: &JsString) -> Undefined {
        self.inner
            .call("alert", &[message.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The confirm method.
    /// [`Window.confirm`](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    pub fn confirm0(&self) -> bool {
        self.inner.call("confirm", &[]).as_::<bool>()
    }
    /// The confirm method.
    /// [`Window.confirm`](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
    pub fn confirm1(&self, message: &JsString) -> bool {
        self.inner.call("confirm", &[message.into()]).as_::<bool>()
    }
}
impl Window {
    /// The prompt method.
    /// [`Window.prompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    pub fn prompt0(&self) -> JsString {
        self.inner.call("prompt", &[]).as_::<JsString>()
    }
    /// The prompt method.
    /// [`Window.prompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    pub fn prompt1(&self, message: &JsString) -> JsString {
        self.inner
            .call("prompt", &[message.into()])
            .as_::<JsString>()
    }
    /// The prompt method.
    /// [`Window.prompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)
    pub fn prompt2(&self, message: &JsString, default: &JsString) -> JsString {
        self.inner
            .call("prompt", &[message.into(), default.into()])
            .as_::<JsString>()
    }
}
impl Window {
    /// The print method.
    /// [`Window.print`](https://developer.mozilla.org/en-US/docs/Web/API/Window/print)
    pub fn print(&self) -> Undefined {
        self.inner.call("print", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// The postMessage method.
    /// [`Window.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)
    pub fn post_message0(&self, message: &Any) -> Undefined {
        self.inner
            .call("postMessage", &[message.into()])
            .as_::<Undefined>()
    }
    /// The postMessage method.
    /// [`Window.postMessage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/postMessage)
    pub fn post_message1(&self, message: &Any, options: &WindowPostMessageOptions) -> Undefined {
        self.inner
            .call("postMessage", &[message.into(), options.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `credentialless` attribute.
    /// [`Window.credentialless`](https://developer.mozilla.org/en-US/docs/Web/API/Window/credentialless)
    pub fn credentialless(&self) -> bool {
        self.inner.get("credentialless").as_::<bool>()
    }
}
impl Window {
    /// Getter of the `orientation` attribute.
    /// [`Window.orientation`](https://developer.mozilla.org/en-US/docs/Web/API/Window/orientation)
    pub fn orientation(&self) -> i16 {
        self.inner.get("orientation").as_::<i16>()
    }
}
impl Window {
    /// Getter of the `onorientationchange` attribute.
    /// [`Window.onorientationchange`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)
    pub fn onorientationchange(&self) -> Any {
        self.inner.get("onorientationchange").as_::<Any>()
    }

    /// Setter of the `onorientationchange` attribute.
    /// [`Window.onorientationchange`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onorientationchange)
    pub fn set_onorientationchange(&mut self, value: &Any) {
        self.inner.set("onorientationchange", value);
    }
}
impl Window {
    /// Getter of the `cookieStore` attribute.
    /// [`Window.cookieStore`](https://developer.mozilla.org/en-US/docs/Web/API/Window/cookieStore)
    pub fn cookie_store(&self) -> CookieStore {
        self.inner.get("cookieStore").as_::<CookieStore>()
    }
}
impl Window {
    /// The navigate method.
    /// [`Window.navigate`](https://developer.mozilla.org/en-US/docs/Web/API/Window/navigate)
    pub fn navigate(&self, dir: &SpatialNavigationDirection) -> Undefined {
        self.inner
            .call("navigate", &[dir.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `viewport` attribute.
    /// [`Window.viewport`](https://developer.mozilla.org/en-US/docs/Web/API/Window/viewport)
    pub fn viewport(&self) -> Viewport {
        self.inner.get("viewport").as_::<Viewport>()
    }
}
impl Window {
    /// The matchMedia method.
    /// [`Window.matchMedia`](https://developer.mozilla.org/en-US/docs/Web/API/Window/matchMedia)
    pub fn match_media(&self, query: &JsString) -> MediaQueryList {
        self.inner
            .call("matchMedia", &[query.into()])
            .as_::<MediaQueryList>()
    }
}
impl Window {
    /// Getter of the `screen` attribute.
    /// [`Window.screen`](https://developer.mozilla.org/en-US/docs/Web/API/Window/screen)
    pub fn screen(&self) -> Screen {
        self.inner.get("screen").as_::<Screen>()
    }
}
impl Window {
    /// Getter of the `visualViewport` attribute.
    /// [`Window.visualViewport`](https://developer.mozilla.org/en-US/docs/Web/API/Window/visualViewport)
    pub fn visual_viewport(&self) -> VisualViewport {
        self.inner.get("visualViewport").as_::<VisualViewport>()
    }
}
impl Window {
    /// The moveTo method.
    /// [`Window.moveTo`](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveTo)
    pub fn move_to(&self, x: i32, y: i32) -> Undefined {
        self.inner
            .call("moveTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The moveBy method.
    /// [`Window.moveBy`](https://developer.mozilla.org/en-US/docs/Web/API/Window/moveBy)
    pub fn move_by(&self, x: i32, y: i32) -> Undefined {
        self.inner
            .call("moveBy", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The resizeTo method.
    /// [`Window.resizeTo`](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeTo)
    pub fn resize_to(&self, width: i32, height: i32) -> Undefined {
        self.inner
            .call("resizeTo", &[width.into(), height.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The resizeBy method.
    /// [`Window.resizeBy`](https://developer.mozilla.org/en-US/docs/Web/API/Window/resizeBy)
    pub fn resize_by(&self, x: i32, y: i32) -> Undefined {
        self.inner
            .call("resizeBy", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `innerWidth` attribute.
    /// [`Window.innerWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerWidth)
    pub fn inner_width(&self) -> i32 {
        self.inner.get("innerWidth").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `innerHeight` attribute.
    /// [`Window.innerHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Window/innerHeight)
    pub fn inner_height(&self) -> i32 {
        self.inner.get("innerHeight").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `scrollX` attribute.
    /// [`Window.scrollX`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollX)
    pub fn scroll_x(&self) -> f64 {
        self.inner.get("scrollX").as_::<f64>()
    }
}
impl Window {
    /// Getter of the `pageXOffset` attribute.
    /// [`Window.pageXOffset`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageXOffset)
    pub fn page_x_offset(&self) -> f64 {
        self.inner.get("pageXOffset").as_::<f64>()
    }
}
impl Window {
    /// Getter of the `scrollY` attribute.
    /// [`Window.scrollY`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollY)
    pub fn scroll_y(&self) -> f64 {
        self.inner.get("scrollY").as_::<f64>()
    }
}
impl Window {
    /// Getter of the `pageYOffset` attribute.
    /// [`Window.pageYOffset`](https://developer.mozilla.org/en-US/docs/Web/API/Window/pageYOffset)
    pub fn page_y_offset(&self) -> f64 {
        self.inner.get("pageYOffset").as_::<f64>()
    }
}
impl Window {
    /// The scroll method.
    /// [`Window.scroll`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scroll)
    pub fn scroll(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scroll", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The scrollTo method.
    /// [`Window.scrollTo`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollTo)
    pub fn scroll_to(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scrollTo", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The scrollBy method.
    /// [`Window.scrollBy`](https://developer.mozilla.org/en-US/docs/Web/API/Window/scrollBy)
    pub fn scroll_by(&self, x: f64, y: f64) -> Undefined {
        self.inner
            .call("scrollBy", &[x.into(), y.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `screenX` attribute.
    /// [`Window.screenX`](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenX)
    pub fn screen_x(&self) -> i32 {
        self.inner.get("screenX").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `screenLeft` attribute.
    /// [`Window.screenLeft`](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenLeft)
    pub fn screen_left(&self) -> i32 {
        self.inner.get("screenLeft").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `screenY` attribute.
    /// [`Window.screenY`](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenY)
    pub fn screen_y(&self) -> i32 {
        self.inner.get("screenY").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `screenTop` attribute.
    /// [`Window.screenTop`](https://developer.mozilla.org/en-US/docs/Web/API/Window/screenTop)
    pub fn screen_top(&self) -> i32 {
        self.inner.get("screenTop").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `outerWidth` attribute.
    /// [`Window.outerWidth`](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerWidth)
    pub fn outer_width(&self) -> i32 {
        self.inner.get("outerWidth").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `outerHeight` attribute.
    /// [`Window.outerHeight`](https://developer.mozilla.org/en-US/docs/Web/API/Window/outerHeight)
    pub fn outer_height(&self) -> i32 {
        self.inner.get("outerHeight").as_::<i32>()
    }
}
impl Window {
    /// Getter of the `devicePixelRatio` attribute.
    /// [`Window.devicePixelRatio`](https://developer.mozilla.org/en-US/docs/Web/API/Window/devicePixelRatio)
    pub fn device_pixel_ratio(&self) -> f64 {
        self.inner.get("devicePixelRatio").as_::<f64>()
    }
}
impl Window {
    /// The getComputedStyle method.
    /// [`Window.getComputedStyle`](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)
    pub fn get_computed_style0(&self, elt: &Element) -> CSSStyleProperties {
        self.inner
            .call("getComputedStyle", &[elt.into()])
            .as_::<CSSStyleProperties>()
    }
    /// The getComputedStyle method.
    /// [`Window.getComputedStyle`](https://developer.mozilla.org/en-US/docs/Web/API/Window/getComputedStyle)
    pub fn get_computed_style1(&self, elt: &Element, pseudo_elt: &JsString) -> CSSStyleProperties {
        self.inner
            .call("getComputedStyle", &[elt.into(), pseudo_elt.into()])
            .as_::<CSSStyleProperties>()
    }
}
impl Window {
    /// The getDigitalGoodsService method.
    /// [`Window.getDigitalGoodsService`](https://developer.mozilla.org/en-US/docs/Web/API/Window/getDigitalGoodsService)
    pub fn get_digital_goods_service(
        &self,
        service_provider: &JsString,
    ) -> Promise<DigitalGoodsService> {
        self.inner
            .call("getDigitalGoodsService", &[service_provider.into()])
            .as_::<Promise<DigitalGoodsService>>()
    }
}
impl Window {
    /// Getter of the `documentPictureInPicture` attribute.
    /// [`Window.documentPictureInPicture`](https://developer.mozilla.org/en-US/docs/Web/API/Window/documentPictureInPicture)
    pub fn document_picture_in_picture(&self) -> DocumentPictureInPicture {
        self.inner
            .get("documentPictureInPicture")
            .as_::<DocumentPictureInPicture>()
    }
}
impl Window {
    /// Getter of the `event` attribute.
    /// [`Window.event`](https://developer.mozilla.org/en-US/docs/Web/API/Window/event)
    pub fn event(&self) -> Any {
        self.inner.get("event").as_::<Any>()
    }
}
impl Window {
    /// Getter of the `fence` attribute.
    /// [`Window.fence`](https://developer.mozilla.org/en-US/docs/Web/API/Window/fence)
    pub fn fence(&self) -> Fence {
        self.inner.get("fence").as_::<Fence>()
    }
}
impl Window {
    /// The fetchLater method.
    /// [`Window.fetchLater`](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetchLater)
    pub fn fetch_later0(&self, input: &Any) -> FetchLaterResult {
        self.inner
            .call("fetchLater", &[input.into()])
            .as_::<FetchLaterResult>()
    }
    /// The fetchLater method.
    /// [`Window.fetchLater`](https://developer.mozilla.org/en-US/docs/Web/API/Window/fetchLater)
    pub fn fetch_later1(&self, input: &Any, init: &DeferredRequestInit) -> FetchLaterResult {
        self.inner
            .call("fetchLater", &[input.into(), init.into()])
            .as_::<FetchLaterResult>()
    }
}
impl Window {
    /// The showOpenFilePicker method.
    /// [`Window.showOpenFilePicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)
    pub fn show_open_file_picker0(&self) -> Promise<TypedArray<FileSystemFileHandle>> {
        self.inner
            .call("showOpenFilePicker", &[])
            .as_::<Promise<TypedArray<FileSystemFileHandle>>>()
    }
    /// The showOpenFilePicker method.
    /// [`Window.showOpenFilePicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showOpenFilePicker)
    pub fn show_open_file_picker1(
        &self,
        options: &OpenFilePickerOptions,
    ) -> Promise<TypedArray<FileSystemFileHandle>> {
        self.inner
            .call("showOpenFilePicker", &[options.into()])
            .as_::<Promise<TypedArray<FileSystemFileHandle>>>()
    }
}
impl Window {
    /// The showSaveFilePicker method.
    /// [`Window.showSaveFilePicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)
    pub fn show_save_file_picker0(&self) -> Promise<FileSystemFileHandle> {
        self.inner
            .call("showSaveFilePicker", &[])
            .as_::<Promise<FileSystemFileHandle>>()
    }
    /// The showSaveFilePicker method.
    /// [`Window.showSaveFilePicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showSaveFilePicker)
    pub fn show_save_file_picker1(
        &self,
        options: &SaveFilePickerOptions,
    ) -> Promise<FileSystemFileHandle> {
        self.inner
            .call("showSaveFilePicker", &[options.into()])
            .as_::<Promise<FileSystemFileHandle>>()
    }
}
impl Window {
    /// The showDirectoryPicker method.
    /// [`Window.showDirectoryPicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)
    pub fn show_directory_picker0(&self) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("showDirectoryPicker", &[])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
    /// The showDirectoryPicker method.
    /// [`Window.showDirectoryPicker`](https://developer.mozilla.org/en-US/docs/Web/API/Window/showDirectoryPicker)
    pub fn show_directory_picker1(
        &self,
        options: &DirectoryPickerOptions,
    ) -> Promise<FileSystemDirectoryHandle> {
        self.inner
            .call("showDirectoryPicker", &[options.into()])
            .as_::<Promise<FileSystemDirectoryHandle>>()
    }
}
impl Window {
    /// The captureEvents method.
    /// [`Window.captureEvents`](https://developer.mozilla.org/en-US/docs/Web/API/Window/captureEvents)
    pub fn capture_events(&self) -> Undefined {
        self.inner.call("captureEvents", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// The releaseEvents method.
    /// [`Window.releaseEvents`](https://developer.mozilla.org/en-US/docs/Web/API/Window/releaseEvents)
    pub fn release_events(&self) -> Undefined {
        self.inner.call("releaseEvents", &[]).as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `external` attribute.
    /// [`Window.external`](https://developer.mozilla.org/en-US/docs/Web/API/Window/external)
    pub fn external(&self) -> External {
        self.inner.get("external").as_::<External>()
    }
}
impl Window {
    /// The queryLocalFonts method.
    /// [`Window.queryLocalFonts`](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)
    pub fn query_local_fonts0(&self) -> Promise<TypedArray<FontData>> {
        self.inner
            .call("queryLocalFonts", &[])
            .as_::<Promise<TypedArray<FontData>>>()
    }
    /// The queryLocalFonts method.
    /// [`Window.queryLocalFonts`](https://developer.mozilla.org/en-US/docs/Web/API/Window/queryLocalFonts)
    pub fn query_local_fonts1(&self, options: &QueryOptions) -> Promise<TypedArray<FontData>> {
        self.inner
            .call("queryLocalFonts", &[options.into()])
            .as_::<Promise<TypedArray<FontData>>>()
    }
}
impl Window {
    /// Getter of the `onappinstalled` attribute.
    /// [`Window.onappinstalled`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)
    pub fn onappinstalled(&self) -> Any {
        self.inner.get("onappinstalled").as_::<Any>()
    }

    /// Setter of the `onappinstalled` attribute.
    /// [`Window.onappinstalled`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onappinstalled)
    pub fn set_onappinstalled(&mut self, value: &Any) {
        self.inner.set("onappinstalled", value);
    }
}
impl Window {
    /// Getter of the `onbeforeinstallprompt` attribute.
    /// [`Window.onbeforeinstallprompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinstallprompt)
    pub fn onbeforeinstallprompt(&self) -> Any {
        self.inner.get("onbeforeinstallprompt").as_::<Any>()
    }

    /// Setter of the `onbeforeinstallprompt` attribute.
    /// [`Window.onbeforeinstallprompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforeinstallprompt)
    pub fn set_onbeforeinstallprompt(&mut self, value: &Any) {
        self.inner.set("onbeforeinstallprompt", value);
    }
}
impl Window {
    /// Getter of the `ondeviceorientation` attribute.
    /// [`Window.ondeviceorientation`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondeviceorientation)
    pub fn ondeviceorientation(&self) -> Any {
        self.inner.get("ondeviceorientation").as_::<Any>()
    }

    /// Setter of the `ondeviceorientation` attribute.
    /// [`Window.ondeviceorientation`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondeviceorientation)
    pub fn set_ondeviceorientation(&mut self, value: &Any) {
        self.inner.set("ondeviceorientation", value);
    }
}
impl Window {
    /// Getter of the `ondeviceorientationabsolute` attribute.
    /// [`Window.ondeviceorientationabsolute`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondeviceorientationabsolute)
    pub fn ondeviceorientationabsolute(&self) -> Any {
        self.inner.get("ondeviceorientationabsolute").as_::<Any>()
    }

    /// Setter of the `ondeviceorientationabsolute` attribute.
    /// [`Window.ondeviceorientationabsolute`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondeviceorientationabsolute)
    pub fn set_ondeviceorientationabsolute(&mut self, value: &Any) {
        self.inner.set("ondeviceorientationabsolute", value);
    }
}
impl Window {
    /// Getter of the `ondevicemotion` attribute.
    /// [`Window.ondevicemotion`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondevicemotion)
    pub fn ondevicemotion(&self) -> Any {
        self.inner.get("ondevicemotion").as_::<Any>()
    }

    /// Setter of the `ondevicemotion` attribute.
    /// [`Window.ondevicemotion`](https://developer.mozilla.org/en-US/docs/Web/API/Window/ondevicemotion)
    pub fn set_ondevicemotion(&mut self, value: &Any) {
        self.inner.set("ondevicemotion", value);
    }
}
impl Window {
    /// Getter of the `portalHost` attribute.
    /// [`Window.portalHost`](https://developer.mozilla.org/en-US/docs/Web/API/Window/portalHost)
    pub fn portal_host(&self) -> PortalHost {
        self.inner.get("portalHost").as_::<PortalHost>()
    }
}
impl Window {
    /// The requestIdleCallback method.
    /// [`Window.requestIdleCallback`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
    pub fn request_idle_callback0(&self, callback: &Function) -> u32 {
        self.inner
            .call("requestIdleCallback", &[callback.into()])
            .as_::<u32>()
    }
    /// The requestIdleCallback method.
    /// [`Window.requestIdleCallback`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback)
    pub fn request_idle_callback1(&self, callback: &Function, options: &IdleRequestOptions) -> u32 {
        self.inner
            .call("requestIdleCallback", &[callback.into(), options.into()])
            .as_::<u32>()
    }
}
impl Window {
    /// The cancelIdleCallback method.
    /// [`Window.cancelIdleCallback`](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelIdleCallback)
    pub fn cancel_idle_callback(&self, handle: u32) -> Undefined {
        self.inner
            .call("cancelIdleCallback", &[handle.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// The getSelection method.
    /// [`Window.getSelection`](https://developer.mozilla.org/en-US/docs/Web/API/Window/getSelection)
    pub fn get_selection(&self) -> Selection {
        self.inner.call("getSelection", &[]).as_::<Selection>()
    }
}
impl Window {
    /// Getter of the `sharedStorage` attribute.
    /// [`Window.sharedStorage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/sharedStorage)
    pub fn shared_storage(&self) -> SharedStorage {
        self.inner.get("sharedStorage").as_::<SharedStorage>()
    }
}
impl Window {
    /// Getter of the `speechSynthesis` attribute.
    /// [`Window.speechSynthesis`](https://developer.mozilla.org/en-US/docs/Web/API/Window/speechSynthesis)
    pub fn speech_synthesis(&self) -> SpeechSynthesis {
        self.inner.get("speechSynthesis").as_::<SpeechSynthesis>()
    }
}
impl Window {
    /// Getter of the `launchQueue` attribute.
    /// [`Window.launchQueue`](https://developer.mozilla.org/en-US/docs/Web/API/Window/launchQueue)
    pub fn launch_queue(&self) -> LaunchQueue {
        self.inner.get("launchQueue").as_::<LaunchQueue>()
    }
}
impl Window {
    /// The getScreenDetails method.
    /// [`Window.getScreenDetails`](https://developer.mozilla.org/en-US/docs/Web/API/Window/getScreenDetails)
    pub fn get_screen_details(&self) -> Promise<ScreenDetails> {
        self.inner
            .call("getScreenDetails", &[])
            .as_::<Promise<ScreenDetails>>()
    }
}
impl Window {
    /// Getter of the `onbeforexrselect` attribute.
    /// [`Window.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforexrselect)
    pub fn onbeforexrselect(&self) -> Any {
        self.inner.get("onbeforexrselect").as_::<Any>()
    }

    /// Setter of the `onbeforexrselect` attribute.
    /// [`Window.onbeforexrselect`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onbeforexrselect)
    pub fn set_onbeforexrselect(&mut self, value: &Any) {
        self.inner.set("onbeforexrselect", value);
    }
}
impl Window {
    /// Getter of the `onportalactivate` attribute.
    /// [`Window.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onportalactivate)
    pub fn onportalactivate(&self) -> Any {
        self.inner.get("onportalactivate").as_::<Any>()
    }

    /// Setter of the `onportalactivate` attribute.
    /// [`Window.onportalactivate`](https://developer.mozilla.org/en-US/docs/Web/API/Window/onportalactivate)
    pub fn set_onportalactivate(&mut self, value: &Any) {
        self.inner.set("onportalactivate", value);
    }
}
impl Window {
    /// Getter of the `crypto` attribute.
    /// [`Window.crypto`](https://developer.mozilla.org/en-US/docs/Web/API/Window/crypto)
    pub fn crypto(&self) -> Crypto {
        self.inner.get("crypto").as_::<Crypto>()
    }
}
impl Window {
    /// The requestAnimationFrame method.
    /// [`Window.requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame)
    pub fn request_animation_frame(&self, callback: &Function) -> u32 {
        self.inner
            .call("requestAnimationFrame", &[callback.into()])
            .as_::<u32>()
    }
}
impl Window {
    /// The cancelAnimationFrame method.
    /// [`Window.cancelAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Window/cancelAnimationFrame)
    pub fn cancel_animation_frame(&self, handle: u32) -> Undefined {
        self.inner
            .call("cancelAnimationFrame", &[handle.into()])
            .as_::<Undefined>()
    }
}
impl Window {
    /// Getter of the `sessionStorage` attribute.
    /// [`Window.sessionStorage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/sessionStorage)
    pub fn session_storage(&self) -> Storage {
        self.inner.get("sessionStorage").as_::<Storage>()
    }
}
impl Window {
    /// Getter of the `localStorage` attribute.
    /// [`Window.localStorage`](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage)
    pub fn local_storage(&self) -> Storage {
        self.inner.get("localStorage").as_::<Storage>()
    }
}
