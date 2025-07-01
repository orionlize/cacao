use objc::{
    class, msg_send, msg_send_id,
    rc::{Id, Owned},
    runtime::Object,
};

use crate::{
    appkit::{tabview, toolbar::ToolbarItem},
    foundation::{id, NSString},
    utils::properties::ObjcProperty,
};

#[derive(Debug)]
pub struct TabViewItem {
    pub objc: Id<Object, Owned>,
}

impl From<id> for TabViewItem {
    fn from(value: id) -> Self {
        unsafe {
            Self {
                objc: Id::retain(value).unwrap(),
            }
        }
    }
}

impl TabViewItem {
    pub fn new<S: Into<String>>(indentifier: S) -> Self {
        let indentifier = indentifier.into();

        let objc = unsafe {
            let indentifier = NSString::new(&indentifier);

            let alloc = msg_send_id![class!(NSTabViewItem), alloc];
            msg_send_id![alloc, initWithIdentifier: &*indentifier]
        };

        Self { objc }
    }

    pub fn set_label<S: Into<String>>(&mut self, label: S) {
        let label = NSString::new(&label.into());
        unsafe {
            let _: () = msg_send![&*self.objc, setLabel: &*label];
        }
    }
}
