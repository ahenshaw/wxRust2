use super::*;

// wxHeaderColumn
pub trait HeaderColumnMethods: WxRustMethods {
    fn get_title(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHeaderColumn_GetTitle(self.as_ptr())).into() }
    }
    fn get_bitmap(&self) -> Bitmap {
        unsafe { Bitmap::from_ptr(ffi::wxHeaderColumn_GetBitmap(self.as_ptr())) }
    }
    fn get_bitmap_bundle(&self) -> BitmapBundle {
        unsafe { BitmapBundle::from_ptr(ffi::wxHeaderColumn_GetBitmapBundle(self.as_ptr())) }
    }
    fn get_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetWidth(self.as_ptr()) }
    }
    fn get_min_width(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetMinWidth(self.as_ptr()) }
    }
    fn get_alignment(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetAlignment(self.as_ptr()) }
    }
    fn get_flags(&self) -> c_int {
        unsafe { ffi::wxHeaderColumn_GetFlags(self.as_ptr()) }
    }
    fn has_flag(&self, flag: c_int) -> bool {
        unsafe { ffi::wxHeaderColumn_HasFlag(self.as_ptr(), flag) }
    }
    fn is_resizeable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsResizeable(self.as_ptr()) }
    }
    fn is_sortable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortable(self.as_ptr()) }
    }
    fn is_reorderable(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsReorderable(self.as_ptr()) }
    }
    fn is_hidden(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsHidden(self.as_ptr()) }
    }
    fn is_shown(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsShown(self.as_ptr()) }
    }
    fn is_sort_key(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortKey(self.as_ptr()) }
    }
    fn is_sort_order_ascending(&self) -> bool {
        unsafe { ffi::wxHeaderColumn_IsSortOrderAscending(self.as_ptr()) }
    }
}

// wxHeaderColumnSimple
pub trait HeaderColumnSimpleMethods: SettableHeaderColumnMethods {}

// wxHeaderCtrl
pub trait HeaderCtrlMethods: ControlMethods {
    fn set_column_count(&self, count: c_uint) {
        unsafe { ffi::wxHeaderCtrl_SetColumnCount(self.as_ptr(), count) }
    }
    fn get_column_count(&self) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnCount(self.as_ptr()) }
    }
    fn is_empty(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_IsEmpty(self.as_ptr()) }
    }
    fn update_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrl_UpdateColumn(self.as_ptr(), idx) }
    }
    fn set_columns_order<A: ArrayIntMethods>(&self, order: &A) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_SetColumnsOrder(self.as_ptr(), order)
        }
    }
    fn get_columns_order(&self) -> ArrayInt {
        unsafe { ArrayInt::from_ptr(ffi::wxHeaderCtrl_GetColumnsOrder(self.as_ptr())) }
    }
    fn get_column_at(&self, pos: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnAt(self.as_ptr(), pos) }
    }
    fn get_column_pos(&self, idx: c_uint) -> c_uint {
        unsafe { ffi::wxHeaderCtrl_GetColumnPos(self.as_ptr(), idx) }
    }
    fn reset_columns_order(&self) {
        unsafe { ffi::wxHeaderCtrl_ResetColumnsOrder(self.as_ptr()) }
    }
    fn show_columns_menu<P: PointMethods>(&self, pt: &P, title: &str) -> bool {
        unsafe {
            let pt = pt.as_ptr();
            let title = WxString::from(title);
            let title = title.as_ptr();
            ffi::wxHeaderCtrl_ShowColumnsMenu(self.as_ptr(), pt, title)
        }
    }
    fn add_columns_items<M: MenuMethods>(&self, menu: &M, id_columns_base: c_int) {
        unsafe {
            let menu = menu.as_ptr();
            ffi::wxHeaderCtrl_AddColumnsItems(self.as_ptr(), menu, id_columns_base)
        }
    }
    fn show_customize_dialog(&self) -> bool {
        unsafe { ffi::wxHeaderCtrl_ShowCustomizeDialog(self.as_ptr()) }
    }
    fn get_column_title_width_headercolumn<H: HeaderColumnMethods>(&self, col: &H) -> c_int {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrl_GetColumnTitleWidth(self.as_ptr(), col)
        }
    }
    fn get_column_title_width_uint(&self, idx: c_uint) -> c_int {
        unsafe { ffi::wxHeaderCtrl_GetColumnTitleWidth1(self.as_ptr(), idx) }
    }
    fn move_column_in_order_array<A: ArrayIntMethods>(order: &A, idx: c_uint, pos: c_uint) {
        unsafe {
            let order = order.as_ptr();
            ffi::wxHeaderCtrl_MoveColumnInOrderArray(order, idx, pos)
        }
    }
}

// wxHeaderCtrlSimple
pub trait HeaderCtrlSimpleMethods: HeaderCtrlMethods {
    fn insert_column<H: HeaderColumnSimpleMethods>(&self, col: &H, idx: c_uint) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_InsertColumn(self.as_ptr(), col, idx)
        }
    }
    fn append_column<H: HeaderColumnSimpleMethods>(&self, col: &H) {
        unsafe {
            let col = col.as_ptr();
            ffi::wxHeaderCtrlSimple_AppendColumn(self.as_ptr(), col)
        }
    }
    fn delete_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_DeleteColumn(self.as_ptr(), idx) }
    }
    fn show_column(&self, idx: c_uint, show: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowColumn(self.as_ptr(), idx, show) }
    }
    fn hide_column(&self, idx: c_uint) {
        unsafe { ffi::wxHeaderCtrlSimple_HideColumn(self.as_ptr(), idx) }
    }
    fn show_sort_indicator(&self, idx: c_uint, sort_order: bool) {
        unsafe { ffi::wxHeaderCtrlSimple_ShowSortIndicator(self.as_ptr(), idx, sort_order) }
    }
    fn remove_sort_indicator(&self) {
        unsafe { ffi::wxHeaderCtrlSimple_RemoveSortIndicator(self.as_ptr()) }
    }
}

// wxHyperlinkCtrl
pub trait HyperlinkCtrlMethods: ControlMethods {
    fn create_str<W: WindowMethods, P: PointMethods, S: SizeMethods>(
        &self,
        parent: Option<&W>,
        id: c_int,
        label: &str,
        url: &str,
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
            let label = WxString::from(label);
            let label = label.as_ptr();
            let url = WxString::from(url);
            let url = url.as_ptr();
            let pos = pos.as_ptr();
            let size = size.as_ptr();
            let name = WxString::from(name);
            let name = name.as_ptr();
            ffi::wxHyperlinkCtrl_Create(
                self.as_ptr(),
                parent,
                id,
                label,
                url,
                pos,
                size,
                style,
                name,
            )
        }
    }
    fn get_hover_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetHoverColour(self.as_ptr())) }
    }
    fn get_normal_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetNormalColour(self.as_ptr())) }
    }
    fn get_url(&self) -> String {
        unsafe { WxString::from_ptr(ffi::wxHyperlinkCtrl_GetURL(self.as_ptr())).into() }
    }
    fn get_visited(&self) -> bool {
        unsafe { ffi::wxHyperlinkCtrl_GetVisited(self.as_ptr()) }
    }
    fn get_visited_colour(&self) -> Colour {
        unsafe { Colour::from_ptr(ffi::wxHyperlinkCtrl_GetVisitedColour(self.as_ptr())) }
    }
    fn set_hover_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetHoverColour(self.as_ptr(), colour)
        }
    }
    fn set_normal_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetNormalColour(self.as_ptr(), colour)
        }
    }
    fn set_url(&self, url: &str) {
        unsafe {
            let url = WxString::from(url);
            let url = url.as_ptr();
            ffi::wxHyperlinkCtrl_SetURL(self.as_ptr(), url)
        }
    }
    fn set_visited(&self, visited: bool) {
        unsafe { ffi::wxHyperlinkCtrl_SetVisited(self.as_ptr(), visited) }
    }
    fn set_visited_colour<C: ColourMethods>(&self, colour: &C) {
        unsafe {
            let colour = colour.as_ptr();
            ffi::wxHyperlinkCtrl_SetVisitedColour(self.as_ptr(), colour)
        }
    }
}
