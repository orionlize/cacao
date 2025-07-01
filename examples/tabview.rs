use cacao::appkit::tabview::item::TabViewItem;
use cacao::appkit::tabview::traits::TabViewDelegate;
use cacao::appkit::tabview::TabView;
use cacao::appkit::window::{Window, WindowLevel};
use cacao::appkit::{App, AppDelegate};
use cacao::geometry::Rect;
use cacao::layout::Layout;
use cacao::view::View;

struct BasicApp {
    window: Window,
    content: View,
    tabview: TabView,
}

impl BasicApp {
    fn new() -> Self {
        let res = Self {
            window: Window::default(),
            content: View::new(),
            tabview: TabView::new(),
        };

        res.tabview.set_delegate(&res);

        res.tabview.set_frame(Rect::new(30., 30., 200., 400.));

        let mut label = TabViewItem::new("test");

        label.set_label("Test Tab");
        res.tabview.add_tab_view_item(label);

        let mut label2 = TabViewItem::new("test2");
        label2.set_label("Test Tab 2");
        res.tabview.add_tab_view_item(label2);

        res
    }
}

impl TabViewDelegate for BasicApp {
    const NAME: &'static str = "BasicAppTabViewDelegate";

    fn did_select_tab_view_item(&self, item: &TabViewItem) {
        println!("Selected tab: {:?}", item);
    }
}

impl AppDelegate for BasicApp {
    fn did_finish_launching(&self) {
        self.window.set_title("Crabu");
        self.window.set_minimum_size(472, 744);
        self.window.set_frame(Rect::new(0., 0., 472., 744.));

        self.content.add_subview(&self.tabview);
        self.window.set_content_view(&self.content);
        self.window.set_level(WindowLevel::Normal);

        self.window.show();
    }
}

fn main() {
    App::new("com.hello.world", BasicApp::new()).run();
}
