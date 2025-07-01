//! Handles the Objective-C functionality for the Toolbar module.

use std::sync::Once;

use objc::declare::ClassDecl;
use objc::rc::Id;
use objc::runtime::{Bool, Class, Object, Sel};
use objc::{class, msg_send, sel};

use crate::appkit::tabview::item::TabViewItem;
use crate::appkit::tabview::traits::TabViewDelegate;
use crate::appkit::tabview::TAB_VIEW_PTR;
use crate::foundation::{id, load_or_register_class, NSArray, NSString};
use crate::utils::load;

/// Loads the controller, grabs whatever item is for this identifier, and returns what the
/// Objective-C runtime needs.
extern "C" fn did_select_tab_view_item<T: TabViewDelegate>(this: &Object, _: Sel, _: id, tabview_item: id) {
    let tabview = load::<T>(this, TAB_VIEW_PTR);

    tabview.did_select_tab_view_item(&TabViewItem::from(tabview_item));
}

/// Registers a `NSToolbar` subclass, and configures it to hold some ivars for various things we need
/// to store. We use it as our delegate as well, just to cut down on moving pieces.
pub(crate) fn register_tabview_class<T: TabViewDelegate>(instance: &T) -> &'static Class {
    load_or_register_class("NSObject", instance.subclass_name(), |decl| unsafe {
        // For callbacks
        decl.add_ivar::<usize>(TAB_VIEW_PTR);

        decl.add_method(
            sel!(tabView:didSelectTabViewItem:),
            did_select_tab_view_item::<T> as extern "C" fn(_, _, _, _),
        );
    })
}
