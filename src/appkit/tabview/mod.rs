use std::cell::{RefCell, UnsafeCell};

use objc::rc::Owned;
use objc::{class, msg_send};
use objc::{
    foundation::NSString,
    msg_send_id,
    rc::{Id, Shared},
    runtime::Object,
};

#[cfg(feature = "autolayout")]
use crate::appkit::tabview;
use crate::appkit::tabview::{class::register_tabview_class, traits::TabViewDelegate};
use crate::color::Color;
use crate::foundation::{id, nil};
use crate::layout::Layout;
#[cfg(feature = "autolayout")]
use crate::layout::{LayoutAnchorDimension, LayoutAnchorX, LayoutAnchorY};
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
    /// A pointer to the Objective-C runtime top layout constraint.
    #[cfg(feature = "autolayout")]
    pub top: LayoutAnchorY,

    /// A pointer to the Objective-C runtime leading layout constraint.
    #[cfg(feature = "autolayout")]
    pub leading: LayoutAnchorX,

    /// A pointer to the Objective-C runtime left layout constraint.
    #[cfg(feature = "autolayout")]
    pub left: LayoutAnchorX,

    /// A pointer to the Objective-C runtime trailing layout constraint.
    #[cfg(feature = "autolayout")]
    pub trailing: LayoutAnchorX,

    /// A pointer to the Objective-C runtime right layout constraint.
    #[cfg(feature = "autolayout")]
    pub right: LayoutAnchorX,

    /// A pointer to the Objective-C runtime bottom layout constraint.
    #[cfg(feature = "autolayout")]
    pub bottom: LayoutAnchorY,

    /// A pointer to the Objective-C runtime width layout constraint.
    #[cfg(feature = "autolayout")]
    pub width: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime height layout constraint.
    #[cfg(feature = "autolayout")]
    pub height: LayoutAnchorDimension,

    /// A pointer to the Objective-C runtime center X layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_x: LayoutAnchorX,

    /// A pointer to the Objective-C runtime center Y layout constraint.
    #[cfg(feature = "autolayout")]
    pub center_y: LayoutAnchorY,
}

impl TabView {
    pub fn new() -> Self {
        unsafe {
            let tab_view = msg_send![class!(NSTabView), new];

            let _: () = msg_send![tab_view, setTranslatesAutoresizingMaskIntoConstraints: false];

            Self {
                objc: ObjcProperty::retain(tab_view),
                objc_delegate: RefCell::new(None),
                #[cfg(feature = "autolayout")]
                top: LayoutAnchorY::top(tab_view),

                #[cfg(feature = "autolayout")]
                left: LayoutAnchorX::left(tab_view),

                #[cfg(feature = "autolayout")]
                leading: LayoutAnchorX::leading(tab_view),

                #[cfg(feature = "autolayout")]
                right: LayoutAnchorX::right(tab_view),

                #[cfg(feature = "autolayout")]
                trailing: LayoutAnchorX::trailing(tab_view),

                #[cfg(feature = "autolayout")]
                bottom: LayoutAnchorY::bottom(tab_view),

                #[cfg(feature = "autolayout")]
                width: LayoutAnchorDimension::width(tab_view),

                #[cfg(feature = "autolayout")]
                height: LayoutAnchorDimension::height(tab_view),

                #[cfg(feature = "autolayout")]
                center_x: LayoutAnchorX::center(tab_view),

                #[cfg(feature = "autolayout")]
                center_y: LayoutAnchorY::center(tab_view),
            }
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
