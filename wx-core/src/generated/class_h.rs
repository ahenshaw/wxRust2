use super::*;

// wxHScrolledWindow
wxwidgets! {
    /// In the name of this class, "H" stands for "horizontal" because it can be used for scrolling columns of variable widths.
    #[doc(alias = "wxHScrolledWindow")]
    #[doc(alias = "HScrolledWindow")]
    class HScrolledWindow
        = HScrolledWindowIsOwned<true>(wxHScrolledWindow) impl
        HScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HScrolledWindowIsOwned<OWNED> {
    // BLOCKED: fn wxHScrolledWindow()
    // BLOCKED: fn wxHScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for HScrolledWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HScrolledWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HScrolledWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHScrolledWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for HScrolledWindowIsOwned<OWNED> {
    /// Same as the non-default constructor, but returns a status code: true if ok, false if the window couldn't be created.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHTMLDataObject
wxwidgets! {
    /// wxHTMLDataObject is used for working with HTML-formatted text.
    #[doc(alias = "wxHTMLDataObject")]
    #[doc(alias = "HTMLDataObject")]
    class HTMLDataObject
        = HTMLDataObjectIsOwned<true>(wxHTMLDataObject) impl
        HTMLDataObjectMethods,
        DataObjectSimpleMethods,
        DataObjectMethods
}
impl<const OWNED: bool> HTMLDataObjectIsOwned<OWNED> {
    /// Constructor.
    pub fn new(html: &str) -> HTMLDataObjectIsOwned<OWNED> {
        unsafe {
            let html = WxString::from(html);
            let html = html.as_ptr();
            HTMLDataObjectIsOwned(ffi::wxHTMLDataObject_new(html))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HTMLDataObjectIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HTMLDataObjectIsOwned<OWNED>> for DataObjectSimpleIsOwned<OWNED> {
    fn from(o: HTMLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HTMLDataObjectIsOwned<OWNED>> for DataObjectIsOwned<OWNED> {
    fn from(o: HTMLDataObjectIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for HTMLDataObjectIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHTMLDataObject_delete(self.0) }
        }
    }
}

// wxHVScrolledWindow
wxwidgets! {
    /// This window inherits all functionality of both vertical and horizontal, variable scrolled windows.
    #[doc(alias = "wxHVScrolledWindow")]
    #[doc(alias = "HVScrolledWindow")]
    class HVScrolledWindow
        = HVScrolledWindowIsOwned<true>(wxHVScrolledWindow) impl
        HVScrolledWindowMethods,
        PanelMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HVScrolledWindowIsOwned<OWNED> {
    // BLOCKED: fn wxHVScrolledWindow()
    // BLOCKED: fn wxHVScrolledWindow1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for HVScrolledWindowIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for PanelIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HVScrolledWindowIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HVScrolledWindowIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HVScrolledWindowIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHVScrolledWindow_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for HVScrolledWindowIsOwned<OWNED> {
    /// Same as the non-default constructor, but returns a status code: true if ok, false if the window couldn't be created.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHVScrolledWindow_Create(self.as_ptr(), parent, id, pos, size, style, name)
        }
    }
}

// wxHeaderColumn
wxwidgets! {
    /// Represents a column header in controls displaying tabular data such as wxDataViewCtrl or wxGrid.
    #[doc(alias = "wxHeaderColumn")]
    #[doc(alias = "HeaderColumn")]
    class HeaderColumn
        = HeaderColumnIsOwned<true>(wxHeaderColumn) impl
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnIsOwned<OWNED> {
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> Drop for HeaderColumnIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumn_delete(self.0) }
        }
    }
}

// wxHeaderColumnSimple
wxwidgets! {
    /// Simple container for the information about the column.
    #[doc(alias = "wxHeaderColumnSimple")]
    #[doc(alias = "HeaderColumnSimple")]
    class HeaderColumnSimple
        = HeaderColumnSimpleIsOwned<true>(wxHeaderColumnSimple) impl
        HeaderColumnSimpleMethods,
        SettableHeaderColumnMethods,
        HeaderColumnMethods
}
impl<const OWNED: bool> HeaderColumnSimpleIsOwned<OWNED> {
    /// Constructor for a column header.
    pub fn new_with_str(
        title: &str,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let title = WxString::from(title);
            let title = title.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new(title, width, align, flags))
        }
    }
    pub fn new_with_bitmapbundle<B: BitmapBundleMethods>(
        bitmap: &B,
        width: c_int,
        align: c_int,
        flags: c_int,
    ) -> HeaderColumnSimpleIsOwned<OWNED> {
        unsafe {
            let bitmap = bitmap.as_ptr();
            HeaderColumnSimpleIsOwned(ffi::wxHeaderColumnSimple_new1(bitmap, width, align, flags))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderColumnSimpleIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HeaderColumnSimpleIsOwned<OWNED>>
    for SettableHeaderColumnIsOwned<OWNED>
{
    fn from(o: HeaderColumnSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderColumnSimpleIsOwned<OWNED>> for HeaderColumnIsOwned<OWNED> {
    fn from(o: HeaderColumnSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> Drop for HeaderColumnSimpleIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxHeaderColumnSimple_delete(self.0) }
        }
    }
}

// wxHeaderCtrl
wxwidgets! {
    /// wxHeaderCtrl is the control containing the column headings which is usually used for display of tabular data.
    #[doc(alias = "wxHeaderCtrl")]
    #[doc(alias = "HeaderCtrl")]
    class HeaderCtrl
        = HeaderCtrlIsOwned<true>(wxHeaderCtrl) impl
        HeaderCtrlMethods,
        ControlMethods,
        // WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlIsOwned<OWNED> {
    // BLOCKED: fn wxHeaderCtrl()
    // BLOCKED: fn wxHeaderCtrl1()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for HeaderCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrl_CLASSINFO()) }
    }
}
impl<const OWNED: bool> WindowMethods for HeaderCtrlIsOwned<OWNED> {
    /// Create the header control window.
    fn create<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> bool {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHeaderCtrl_Create(self.as_ptr(), parent, winid, pos, size, style, name)
        }
    }
}

// wxHeaderCtrlEvent
wxwidgets! {
    /// Event class representing the events generated by wxHeaderCtrl.
    #[doc(alias = "wxHeaderCtrlEvent")]
    #[doc(alias = "HeaderCtrlEvent")]
    class HeaderCtrlEvent
        = HeaderCtrlEventIsOwned<true>(wxHeaderCtrlEvent) impl
        HeaderCtrlEventMethods,
        NotifyEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlEventIsOwned<OWNED> {
    // NOT_SUPPORTED: fn wxHeaderCtrlEvent()
    pub fn new<H: HeaderCtrlEventMethods>(event: &H) -> HeaderCtrlEventIsOwned<OWNED> {
        unsafe {
            let event = event.as_ptr();
            HeaderCtrlEventIsOwned(ffi::wxHeaderCtrlEvent_new1(event))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HeaderCtrlEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for NotifyEventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrlEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HeaderCtrlEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHeaderCtrlSimple
wxwidgets! {
    /// wxHeaderCtrlSimple is a concrete header control which can be used directly, without inheriting from it as you need to do when using wxHeaderCtrl itself.
    #[doc(alias = "wxHeaderCtrlSimple")]
    #[doc(alias = "HeaderCtrlSimple")]
    class HeaderCtrlSimple
        = HeaderCtrlSimpleIsOwned<true>(wxHeaderCtrlSimple) impl
        HeaderCtrlSimpleMethods,
        HeaderCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HeaderCtrlSimpleIsOwned<OWNED> {
    /// Default constructor not creating the underlying window.
    pub fn new_2step() -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe { HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new()) }
    }
    /// Constructor creating the window.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        winid: c_int,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HeaderCtrlSimpleIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HeaderCtrlSimpleIsOwned(ffi::wxHeaderCtrlSimple_new1(
                parent, winid, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for HeaderCtrlSimpleIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for HeaderCtrlIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HeaderCtrlSimpleIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HeaderCtrlSimpleIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HeaderCtrlSimpleIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHeaderCtrlSimple_CLASSINFO()) }
    }
}

// wxHelpEvent
wxwidgets! {
    /// A help event is sent when the user has requested context-sensitive help.
    #[doc(alias = "wxHelpEvent")]
    #[doc(alias = "HelpEvent")]
    class HelpEvent
        = HelpEventIsOwned<true>(wxHelpEvent) impl
        HelpEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HelpEventIsOwned<OWNED> {
    //  ENUM: Origin
    pub const Origin_Unknown: c_int = 0;
    pub const Origin_Keyboard: c_int = 0 + 1;
    pub const Origin_HelpButton: c_int = 0 + 2;

    // NOT_SUPPORTED: fn wxHelpEvent()
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HelpEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HelpEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HelpEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HelpEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHelpEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HelpEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}

// wxHyperlinkCtrl
wxwidgets! {
    /// This class shows a static text element which links to an URL.
    #[doc(alias = "wxHyperlinkCtrl")]
    #[doc(alias = "HyperlinkCtrl")]
    class HyperlinkCtrl
        = HyperlinkCtrlIsOwned<true>(wxHyperlinkCtrl) impl
        HyperlinkCtrlMethods,
        ControlMethods,
        WindowMethods,
        EvtHandlerMethods,
        ObjectMethods
}
impl<const OWNED: bool> HyperlinkCtrlIsOwned<OWNED> {
    pub fn new_2step() -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe { HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new()) }
    }
    /// Constructor.
    pub fn new<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
        pos: &P,
        size: &S,
        style: c_long,
        name: &str,
    ) -> HyperlinkCtrlIsOwned<OWNED> {
        unsafe {
            let parent = match parent {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            HyperlinkCtrlIsOwned(ffi::wxHyperlinkCtrl_new1(
                parent, id, label, url, pos, size, style, name,
            ))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl<const OWNED: bool> Clone for HyperlinkCtrlIsOwned<OWNED> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for ControlIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for WindowIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for EvtHandlerIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkCtrlIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HyperlinkCtrlIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HyperlinkCtrlIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHyperlinkCtrl_CLASSINFO()) }
    }
}

// wxHyperlinkEvent
wxwidgets! {
    /// This event class is used for the events generated by wxHyperlinkCtrl.
    #[doc(alias = "wxHyperlinkEvent")]
    #[doc(alias = "HyperlinkEvent")]
    class HyperlinkEvent
        = HyperlinkEventIsOwned<true>(wxHyperlinkEvent) impl
        HyperlinkEventMethods,
        CommandEventMethods,
        EventMethods,
        ObjectMethods
}
impl<const OWNED: bool> HyperlinkEventIsOwned<OWNED> {
    /// The constructor is not normally used by the user code.
    pub fn new<O: ObjectMethods>(
        generator: Option<&O>,
        id: c_int,
        url: &str,
    ) -> HyperlinkEventIsOwned<OWNED> {
        unsafe {
            let generator = match generator {
                Some(r) => r.as_ptr(),
                None => ptr::null_mut(),
            };
            let url = WxString::from(url);
            let url = url.as_ptr();
            HyperlinkEventIsOwned(ffi::wxHyperlinkEvent_new(generator, id, url))
        }
    }
    pub fn none() -> Option<&'static Self> {
        None
    }
}
impl Clone for HyperlinkEventIsOwned<false> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for CommandEventIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for EventIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> From<HyperlinkEventIsOwned<OWNED>> for ObjectIsOwned<OWNED> {
    fn from(o: HyperlinkEventIsOwned<OWNED>) -> Self {
        unsafe { Self::from_ptr(o.as_ptr()) }
    }
}
impl<const OWNED: bool> DynamicCast for HyperlinkEventIsOwned<OWNED> {
    fn class_info() -> ClassInfoIsOwned<false> {
        unsafe { ClassInfoIsOwned::from_ptr(ffi::wxHyperlinkEvent_CLASSINFO()) }
    }
}
impl<const OWNED: bool> Drop for HyperlinkEventIsOwned<OWNED> {
    fn drop(&mut self) {
        if OWNED {
            unsafe { ffi::wxObject_delete(self.0) }
        }
    }
}
