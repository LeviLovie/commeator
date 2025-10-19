mod left;
mod right;

pub use left::*;
pub use right::*;

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PanelLayout {
    Desktop,
    Mobile,
}

#[derive(Clone)]
pub struct LayoutContext {
    pub layout: Signal<PanelLayout>,
}
