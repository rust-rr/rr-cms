use crate::{
    model::{ModelController, Ticket, TicketForCreate},
    Result,
};
use axum::{
    extract::{Path, State},
    routing::{delete, post},
    Json, Router,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - {:<24}", "HANDLER", "create_ticket");

    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - {:<24}", "HANDLER", "list_tickets");

    let tickets = mc.list_tickets().await?;
    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - {:<24}", "HANDLER", "delete_ticket");

    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
