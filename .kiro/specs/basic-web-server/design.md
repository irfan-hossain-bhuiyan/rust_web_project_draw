# Design Document

## Overview

The collaborative pixel canvas application is a Leptos CSR web application that allows multiple users to draw pixels on a shared canvas in real-time. The design prioritizes scalability, network efficiency, and memory optimization while maintaining a modular architecture that can evolve rapidly.

## Architecture

### High-Level Architecture

```
┌─────────────────┐    WebSocket    ┌─────────────────┐
│   Leptos CSR    │◄──────────────►│  WebSocket      │
│   Frontend      │   (Binary)      │  Server         │
│                 │                 │  (External)     │
└─────────────────┘                 └─────────────────┘
```

### Component Architecture

```
App
├── CanvasContainer
│   ├── Canvas (scalable grid)
│   └── ConnectionStatus
└── WebSocketManager (service)
```

## Components and Interfaces

### Core Data Structures

**Canvas State Representation:**
- Use bitset/bit vector for memory efficiency (1 bit per pixel for black/white)
- Configurable canvas dimensions (start with 100x100, easily scalable)
- Reactive state management with Leptos signals

**Network Protocol Design:**
- Binary WebSocket messages instead of JSON for maximum efficiency
- Batch pixel updates for drag operations
- Delta compression for large canvas updates
- Message types: single pixel, pixel batch, canvas diff, full canvas state

### Component Responsibilities

**App Component:**
- Root application container
- Global state initialization

**CanvasContainer:**
- Manages canvas state (bitset representation)
- Handles WebSocket connection lifecycle
- Coordinates between canvas rendering and network updates

**Canvas Component:**
- Renders pixel grid efficiently (consider virtual scrolling for large canvases)
- Handles user interactions (click, drag)
- Batches rapid updates for network efficiency

**WebSocketManager:**
- Binary message encoding/decoding
- Connection management with auto-reconnect
- Message queuing and batching for drag operations

## Data Models

### Canvas State Management

**Memory Efficient Storage:**
- Bitset representation: 100x100 pixels = 10,000 bits = 1.25KB
- Easy to scale: 1000x1000 = 125KB, 2000x2000 = 500KB
- Fast bit operations for pixel toggling and batch updates

**Reactive Updates:**
- Leptos signals for UI reactivity
- Efficient partial updates (only changed regions)
- Local optimistic updates with server reconciliation

### Network Protocol

**Binary Message Format:**
- Header: Message type (1 byte) + payload length (2 bytes)
- Single pixel: x (2 bytes) + y (2 bytes) + color (1 bit)
- Line draw: start_x (2 bytes) + start_y (2 bytes) + end_x (2 bytes) + end_y (2 bytes) + color (1 bit)
- Canvas diff: compressed bitset differences
- Full state: compressed bitset of entire canvas

**Drag Operation Efficiency:**
- Line drawing for drag operations (2 endpoints vs hundreds of pixels)
- Server-side line rasterization using efficient algorithms (Bresenham's line algorithm)
- Single message per drag gesture instead of pixel-by-pixel updates

## Error Handling

### Network Resilience

**Connection Management:**
- Automatic reconnection with exponential backoff
- Message queuing during disconnection
- Conflict resolution for concurrent updates

**Data Integrity:**
- Message validation and checksums
- Canvas state synchronization on reconnect
- Graceful degradation for network issues

### Component Error Handling

**Canvas Rendering:**
- Efficient error boundaries
- Fallback rendering for large canvases
- Performance monitoring and optimization

## Testing Strategy

### Performance Testing

**Canvas Scalability:**
- Test rendering performance with different canvas sizes
- Memory usage monitoring for bitset operations
- Network efficiency testing with batch updates

**Network Protocol:**
- Binary message encoding/decoding correctness
- Batch update efficiency testing
- Connection resilience testing

### Development Testing

**Modular Testing:**
- Unit tests for bitset operations
- Component testing with mock WebSocket
- Integration testing with local WebSocket server

## Scalability Considerations

### Canvas Size Flexibility

**Configurable Dimensions:**
- Runtime canvas size configuration
- Efficient rendering for large canvases (virtual scrolling)
- Memory management for different canvas sizes

### Network Optimization

**Efficient Updates:**
- Batch pixel updates during drag operations
- Compression for large canvas synchronization
- Adaptive update frequency based on network conditions

### Future Extensibility

**Modular Design:**
- Easy addition of new pixel colors/tools
- Pluggable network protocols
- Configurable canvas features (layers, history, etc.)

## Implementation Strategy

**Modular Architecture Philosophy:**
- Each component should be completely independent and replaceable
- Use trait-based abstractions for easy swapping of implementations
- Separate concerns: rendering, state management, network, input handling
- Plugin-style architecture for easy feature addition/removal

**Small, Incremental Steps:**
- Start with basic single-pixel click functionality
- Add line drawing for drag operations (client-side preview, server-side rasterization)
- Implement binary protocol step by step
- Each step should be fully working before moving to the next

**Future-Proof Design:**
- Abstract interfaces for canvas operations (CanvasTrait, NetworkTrait, InputTrait)
- Configurable message types and handlers
- Pluggable rendering backends
- Easy addition of new drawing tools (brush sizes, colors, shapes)

**Development Approach:**
- Build the smallest possible working version first
- Add one feature at a time with full testing
- Keep each module under 100 lines when possible
- Use composition over inheritance for flexibility