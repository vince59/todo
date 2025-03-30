use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use minijinja::context;
use std::sync::Arc;
use crate::AppState;

pub async fn controller_home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("home").unwrap();

    let rendered = template
        .render(context! {
            title => "Todo liste",
        })
        .unwrap();

    Ok(Html(rendered))
}