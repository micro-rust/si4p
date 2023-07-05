//! All messages of an USB Selector.



use std::sync::Arc;

use super::actions::show::ShowAction;



#[derive(Clone, Debug)]
pub enum Message {
    Show( Arc<dyn ShowAction> ),
}
