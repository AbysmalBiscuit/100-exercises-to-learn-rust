use std::boxed::Box;
use std::sync::Arc;

use crate::{
    data::{Post, TicketDraft},
    store::TicketStore,
};

/// Function that handles GET requests at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    let post = Post {
        id,
        title: "Hello".to_string(),
        body: "this is a post".to_string(),
    };
    Ok(warp::reply::json(&post))
}

pub fn add_ticket(
    mut store: Arc<&TicketStore>,
    request: TicketDraft,
) -> Result<impl warp::Reply, warp::Rejection> {
    let id = store.add_ticket(request);
    Ok(id)
    // Ok(Box::new(id))
}
