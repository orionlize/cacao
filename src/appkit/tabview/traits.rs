//! Traits that can be used for Toolbar construction. Relatively straightforward, as far as these
//! go. Currently a bit incomplete in that we don't support the customizing workflow, but feel free
//! to pull request it.

use crate::appkit::tabview::item::TabViewItem;

/// A trait that you can implement to have your struct/etc act as an `NSToolbarDelegate`.
pub trait TabViewDelegate {
    /// Used to cache subclass creations on the Objective-C side.
    /// You can just set this to be the name of your view type. This
    /// value *must* be unique per-type.
    const NAME: &'static str;

    /// You should rarely (read: probably never) need to implement this yourself.
    /// It simply acts as a getter for the associated `NAME` const on this trait.
    fn subclass_name(&self) -> &'static str {
        Self::NAME
    }

    /// This method can be used to configure your toolbar, if you need to do things involving the
    /// handle. Unlike some other view types, it's not strictly necessary, and is provided in the
    /// interest of a uniform and expectable API.
    // fn did_load(&mut self, _toolbar: Toolbar) {}

    fn did_select_tab_view_item(&self, item: &TabViewItem);
}
