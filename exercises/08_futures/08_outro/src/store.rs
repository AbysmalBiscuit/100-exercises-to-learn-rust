use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};

use crate::data::{Status, Ticket, TicketDraft, TicketPatch};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

impl warp::Reply for TicketId {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::html(self.0.to_string()).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TicketStoreError {
    #[error("TicketId was not found in the store {0:?}")]
    MissingTicket(TicketId),
    #[error("Failed to get WriteLock for ticket {0:?}")]
    FailedWriteLock(TicketId),
}

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

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        let ticket = Arc::new(RwLock::new(ticket));
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<Arc<RwLock<Ticket>>> {
        self.tickets.get(&id).cloned()
    }

    pub fn update(&mut self, patch: TicketPatch) -> anyhow::Result<Ticket> {
        let Some(ticket) = self.tickets.get_mut(&patch.id).cloned() else {
            Err(TicketStoreError::MissingTicket(patch.id))?
        };
        let mut writer = ticket
            .write()
            .map_err(|_| TicketStoreError::FailedWriteLock(patch.id))?;
        if let Some(title) = patch.title {
            writer.title = title;
        }
        if let Some(description) = patch.description {
            writer.description = description;
        }
        if let Some(status) = patch.status {
            writer.status = status;
        }
        Ok(writer.clone())
    }
}

impl Default for TicketStore {
    fn default() -> Self {
        Self::new()
    }
}
