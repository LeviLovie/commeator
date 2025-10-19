// mod left;
// mod right;

use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PanelLayout {
    Desktop,
    Mobile,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum MobileState {
    #[default]
    Left,
    Right,
}

#[derive(Clone)]
pub struct LayoutContext {
    pub layout: Signal<PanelLayout>,
    pub mobile_state: Signal<MobileState>,
}
