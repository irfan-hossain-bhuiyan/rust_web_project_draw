# Collaborative Drawing Canvas

A real-time collaborative drawing application built with Rust, Leptos (frontend), and Axum (backend).

## Features

- **Real-time Collaboration**: Multiple users can draw simultaneously
- **Efficient Binary Protocol**: Custom 9-byte binary messages for optimal performance
- **Modern UI**: Hover toolbar with pen/eraser tools
- **Responsive Canvas**: Zoom, pan, and drawing capabilities
- **WebSocket Communication**: Low-latency real-time updates

## Architecture

- **Frontend**: Leptos (Rust WASM) with canvas rendering
- **Backend**: Axum server with WebSocket support
- **Protocol**: Custom binary protocol (9 bytes per drawing event, 1250 bytes for full canvas sync)
- **Canvas**: 100x100 pixel grid with efficient bit-packed storage

## Getting Started

### Prerequisites

- Rust (latest stable)
- Trunk (for WASM building): `cargo install trunk`

### Running the Application

1. **Build the frontend**:
   ```bash
   trunk build --release
   ```

2. **Start the backend server**:
   ```bash
   cargo run --bin server
   ```

3. **Open your browser** and navigate to `http://127.0.0.1:3000`

4. **Test collaboration** by opening multiple browser tabs/windows

### Development Mode

For development with hot reload:

1. **Terminal 1** - Start the backend:
   ```bash
   cargo run --bin server
   ```

2. **Terminal 2** - Start frontend with hot reload:
   ```bash
   trunk serve
   ```

3. **Open** `http://127.0.0.1:8080` (frontend dev server will proxy to backend)

## Usage

- **Drawing**: Click and drag with left mouse button
- **Erasing**: Select eraser tool from hover toolbar, then click and drag
- **Panning**: Click and drag with middle mouse button
- **Zooming**: Use mouse wheel
- **Toolbar**: Move mouse near top of screen to reveal tools

## Technical Details

### Binary Protocol

- **DrawEvent**: 9 bytes per message
  - `msg_type` (1 byte): 0=DrawLine, 1=FullSync, 2=Clear
  - `x0, y0, x1, y1` (4 bytes): Line coordinates (0-99)
  - `is_black` (1 byte): 1=draw, 0=erase
  - `padding` (3 bytes): Alignment

- **CanvasState**: 1250 bytes for full canvas (100x100 bits)

### Performance

- **Latency**: Sub-millisecond drawing updates
- **Bandwidth**: ~9 bytes per drawing stroke
- **Memory**: Efficient bit-packed canvas storage
- **Scalability**: Broadcast to unlimited concurrent users

## Project Structure

```
src/
├── bin/
│   └── server.rs          # Axum backend server
├── components/
│   ├── canvas.rs          # Drawing canvas component
│   └── toolbar.rs         # Hover toolbar component
├── services/
│   └── websocket.rs       # WebSocket client service
├── shared/
│   └── mod.rs             # Binary protocol types
├── types/
│   └── pixel_canvas.rs    # Canvas data structures
└── main.rs                # Frontend entry point
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test with multiple browser instances
5. Submit a pull request

## License

MIT License
