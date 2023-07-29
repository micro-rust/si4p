//! Theme for the console.



use marcel::{
    Color, Container, Picklist,
};

use std::sync::Arc;



pub(super) struct Theme {
    /// Console container theme.
    pub(super) background: Arc<Container>,

    /// Topbar container theme.
    pub(super) topbar: Arc<Container>,

    /// Picklist theme.
    pub(super) picklist: Arc<Picklist>,

    /// Text color.
    pub(super) text: Arc<Color>,

    /// Log level colors.
    pub(super) level: [Arc<Color>; 5],
}
