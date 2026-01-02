// 示例 DSL 程序
// 这是一个简单的 DSL 代码示例，展示如何使用 classes! 宏

use classes_macros::classes;
use classes::prelude::*;

classes! {
    // 抽象类 BuildContext
    abstract class BuildContext {
        pub fn widget(&self) -> CRc<Widget>;
    }

    // 抽象类 Element，实现 BuildContext 接口
    abstract class Element implements BuildContext {
        struct {
            parent: Option<CWeak<Element>>,
            widget: Option<CWeak<Widget>>,
            dirty: bool,
        }
        
        pub fn new(widget: Option<CRc<Widget>>) -> Self {
            Self {
                parent: None,
                widget,
                dirty: true,
            }
        }
        
        pub fn rebuild(&self, force: bool) {
            if self.get_dirty() || force {
                self.perform_rebuild();
            }
        }
        
        pub fn perform_rebuild(&self) {
            self.set_dirty(false);
        }
        
        pub override fn <Self as BuildContext>::widget(&self) -> CRc<Widget> {
            self.get_widget().unwrap()
        }
    }

    // 抽象类 Widget
    abstract class Widget {
        pub fn create_element(&self) -> CRc<Element>;
    }
}

