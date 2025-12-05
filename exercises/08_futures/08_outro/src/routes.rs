use std::sync::Arc;

use crate::{data::TicketDraft, store::TicketStore};

use super::handlers;
use warp::{Filter, Rejection, Reply};

// impl Filter<Extract = impl Reply, Error = Rejection> + Clone

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    get_post()
}

// A route to handle GET requests for a specific post
fn get_post() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::get())
        .and_then(handlers::get_post)
}

fn add_ticket(
    store: Arc<TicketStore>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("ticket/add")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|ticket_draft: TicketDraft| handlers::add_ticket(store, ticket_draft))
}
