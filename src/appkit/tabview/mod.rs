use std::cell::{RefCell, UnsafeCell};

use objc::rc::Owned;
use objc::{class, msg_send};
use objc::{
    foundation::NSString,
    msg_send_id,
    rc::{Id, Shared},
    runtime::Object,
};

use crate::appkit::tabview::{class::register_tabview_class, traits::TabViewDelegate};
use crate::color::Color;
use crate::foundation::{id, nil};
use crate::layout::Layout;
use crate::objc_access::ObjcAccess;
use crate::utils::properties::ObjcProperty;
use crate::view::BACKGROUND_COLOR;

pub mod class;
pub mod item;
pub mod traits;

pub(crate) static TAB_VIEW_PTR: &str = "cacaoTabViewPtr";

#[derive(Debug)]
pub struct TabView {
    /// The Objective-C runtime toolbar.
    pub objc: ObjcProperty,
    objc_delegate: RefCell<Option<Id<Object, Owned>>>,
}

impl TabView {
    pub fn new() -> Self {
        let objc = unsafe {
            let tabview = msg_send![class!(NSTabView), new];

            ObjcProperty::retain(tabview)
        };

        Self {
            objc,

            objc_delegate: RefCell::new(None),
        }
    }
}

impl TabView {
    pub fn set_delegate<T>(&self, delegate: &T)
    where
        T: TabViewDelegate + 'static,
    {
        let class = register_tabview_class(delegate);

        unsafe {
            let mut objc_delegate: Id<Object, Owned> = msg_send_id![class, new];
            objc_delegate.set_ivar(TAB_VIEW_PTR, delegate as *const T as usize);

            self.objc.with_mut(|objc| {
                let _: () = msg_send![objc, setDelegate: &*objc_delegate];
            });

            *self.objc_delegate.borrow_mut() = Some(objc_delegate);
        }
    }
}

impl TabView {
    pub fn add_tab_view_item(&self, item: item::TabViewItem) {
        unsafe {
            self.objc.with_mut(|objc| {
                let _: () = msg_send![objc, addTabViewItem: &*item.objc];
            });
        }
    }

    pub fn select_tab_view_item_with_identifier<S: Into<String>>(&self, identifier: S) {
        let identifier = NSString::from_str(&identifier.into());
        unsafe {
            self.objc.with_mut(|objc| {
                let _: () = msg_send![objc, selectTabViewItemWithIdentifier: &*identifier];
            });
        }
    }
}

impl ObjcAccess for TabView {
    fn with_backing_obj_mut<F: Fn(id)>(&self, handler: F) {
        self.objc.with_mut(handler);
    }

    fn get_from_backing_obj<F: Fn(&Object) -> R, R>(&self, handler: F) -> R {
        self.objc.get(handler)
    }
}

impl Layout for TabView {}
