use axum::{
    Json, Router,
    extract::State,
    http::{Method, header},
    response::IntoResponse,
    routing::{get, post},
};
use seiti_core::{BoardState, Logger, StoneMove, compute_stone_moves};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};

const DEFAULT_PORT: u16 = 3000;

#[derive(Clone)]
struct AppState {
    logger: Arc<dyn Logger + Send + Sync>,
}

struct StdoutLogger;
impl Logger for StdoutLogger {
    fn log(&self, s: &str) {
        println!("{s}");
    }
}

#[derive(Deserialize)]
struct GenerateReq {
    seed: u32,
}

#[derive(Serialize)]
struct ErrorResp {
    error: String,
}

async fn health() -> &'static str {
    "ok"
}

async fn generate_board(Json(req): Json<GenerateReq>) -> Json<BoardState> {
    Json(seiti_core::generate_board_state(req.seed))
}

#[derive(Deserialize)]
struct LevelReq {
    board: BoardState,
}

#[derive(Serialize)]
struct LevelResp {
    board: BoardState,
    moves: Vec<StoneMove>,
}

async fn level_board(
    State(state): State<AppState>,
    Json(req): Json<LevelReq>,
) -> impl IntoResponse {
    let before = req.board.clone();
    match seiti_core::level_board(req.board, Some(state.logger.as_ref())) {
        Ok(after) => match compute_stone_moves(&before, &after) {
            Ok(moves) => (
                axum::http::StatusCode::OK,
                Json(LevelResp {
                    board: after,
                    moves,
                }),
            )
                .into_response(),
            Err(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResp {
                    error: format!("failed to compute moves: {}", e),
                }),
            )
                .into_response(),
        },
        Err(e) => (
            axum::http::StatusCode::BAD_REQUEST,
            Json(ErrorResp { error: e }),
        )
            .into_response(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app_state = AppState {
        logger: Arc::new(StdoutLogger),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/api/board/generate", post(generate_board))
        .route("/api/board/level", post(level_board))
        .layer(cors)
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], DEFAULT_PORT));
    println!("backend listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| {
            eprintln!("Failed to bind to {addr}: {e}");
            eprintln!("Port {DEFAULT_PORT} may already be in use. Please stop the existing process or use a different port.");
            e
        })?;
    axum::serve(listener, app).await.map_err(|e| {
        eprintln!("Server error: {e}");
        e
    })?;
    Ok(())
}
