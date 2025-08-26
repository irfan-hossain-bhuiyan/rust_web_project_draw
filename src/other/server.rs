use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::{info, warn, error};

// Import our shared protocol
use rust_web::{DrawEvent, CanvasState};

/// Shared application state
#[derive(Clone)]
struct AppState {
    /// Canvas state shared across all clients
    canvas: Arc<Mutex<CanvasState>>,
    /// Broadcast channel for real-time updates
    tx: broadcast::Sender<Vec<u8>>,
}

impl AppState {
    fn new() -> Self {
        let (tx, _rx) = broadcast::channel(1000);
        Self {
            canvas: Arc::new(Mutex::new(CanvasState::new())),
            tx,
        }
    }

    /// Apply a draw event to the canvas and broadcast it
    fn apply_draw_event(&self, event: DrawEvent) {
        // Apply to local canvas state
        {
            let mut canvas = self.canvas.lock().unwrap();
            match event.msg_type {
                0 => {
                    // Draw line
                    self.draw_line_on_canvas(
                        &mut canvas,
                        event.x0 as usize,
                        event.y0 as usize,
                        event.x1 as usize,
                        event.y1 as usize,
                        event.is_black == 1,
                    );
                }
                2 => {
                    // Clear canvas
                    canvas.clear();
                }
                _ => {}
            }
        }

        // Broadcast to all connected clients
        let bytes = event.to_bytes().to_vec();
        if let Err(e) = self.tx.send(bytes) {
            warn!("Failed to broadcast draw event: {}", e);
        }
    }

    /// Draw line using Bresenham's algorithm (same as frontend)
    fn draw_line_on_canvas(
        &self,
        canvas: &mut CanvasState,
        x0: usize,
        y0: usize,
        x1: usize,
        y1: usize,
        is_black: bool,
    ) {
        let (mut x0, mut y0, x1, y1) = (x0 as isize, y0 as isize, x1 as isize, y1 as isize);

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && y0 >= 0 && x0 < 100 && y0 < 100 {
                canvas.set_pixel(x0 as usize, y0 as usize, is_black);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Get current canvas state for new clients
    fn get_canvas_state(&self) -> CanvasState {
        let canvas = self.canvas.lock().unwrap();
        CanvasState::from_bytes(canvas.to_bytes())
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app_state = AppState::new();

    // Build our application with routes
    let app = Router::new()
        .route("/ws", get(websocket_handler))
        .route("/", get(serve_index))
        .nest_service("/dist", get_service(ServeDir::new("dist")))
        .nest_service("/styles.css", get_service(ServeFile::new("styles.css")))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("🚀 Collaborative drawing server listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}

/// Serve the main HTML page
async fn serve_index() -> impl IntoResponse {
    Html(std::fs::read_to_string("index.html").unwrap_or_else(|_| {
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Collaborative Drawing Canvas</title>
    <link rel="stylesheet" href="/styles.css">
</head>
<body>
    <script type="module">
        import init from '/dist/rust_web.js';
        init();
    </script>
</body>
</html>"#.to_string()
    }))
}

/// WebSocket handler for real-time drawing updates
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

/// Handle individual WebSocket connection
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    info!("🔗 New client connected");

    // Send current canvas state to new client
    let canvas_state = state.get_canvas_state();
    let full_sync_bytes = [&[1u8], canvas_state.to_bytes().as_slice()].concat();
    if let Err(e) = sender.send(Message::Binary(full_sync_bytes)).await {
        error!("Failed to send initial canvas state: {}", e);
        return;
    }

    // Spawn task to handle broadcasting to this client
    let broadcast_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Binary(msg)).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages from this client
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                if data.len() == 9 {
                    // Parse draw event
                    let mut event_bytes = [0u8; 9];
                    event_bytes.copy_from_slice(&data);
                    let event = DrawEvent::from_bytes(&event_bytes);
                    
                    info!("📝 Received draw event: {:?}", event);
                    
                    // Apply and broadcast the event
                    state.apply_draw_event(event);
                } else {
                    warn!("Received invalid binary message length: {}", data.len());
                }
            }
            Ok(Message::Close(_)) => {
                info!("🔌 Client disconnected");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {
                // Ignore other message types
            }
        }
    }

    // Clean up
    broadcast_task.abort();
    info!("🧹 Client connection cleaned up");
}
