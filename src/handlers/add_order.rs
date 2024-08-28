use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{contracts::saver::Saver, model::model::Order, state::state::AppState};

pub async fn add_order(State(state):State<AppState>,Json(order):Json<Order>)->impl IntoResponse{
        let b=serde_json::to_string_pretty(&order).unwrap();
        state.saver.save(b).await;

        return StatusCode::OK
    //1) find into redis (on id)
        //if err=morder exist =>return from handler with msg (order exist) HTTP Code -409 (conflict)
        //if err= redis not availabale=> save model in file(in future need check save in redis and postgres) and return Http code 500(internal error) 
    //2) Insert into postgres
}