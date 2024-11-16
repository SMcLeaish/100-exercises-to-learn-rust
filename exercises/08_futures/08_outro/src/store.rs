use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::data::{Status, Ticket, TicketDraft};

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Arc<RwLock<Ticket>>>,
    counter: u64,
}
impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }
}
impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}
