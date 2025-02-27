# Conway's Game of Life Over Time

An implementation of Conway's Game of Life using the Bevy game engine. Instead of the usual 2D grid simulation, the cellular automata are placed in a 3D grid that slowly descends at a fixed speed. Previous generations of the simulation are retained, allowing you to see the progress of how the states change over time.

This project was inspired by [a cool Instagram post](https://www.instagram.com/reel/C2hoRnFsmQW/) of the same visualization.

## Setup

### Prerequisites

- Rust (MSRV of bevy 0.15)

### Dependencies

- `bevy` for the game engine
- `rand` for random number generation
- `wasm-bindgen` for WebAssembly support

## Usage

### Camera Controls

- **Orbit**: Click and drag with the left mouse button to orbit around the simulation.
- **Zoom**: Use the mouse scroll wheel to zoom in and out.

## License

This project is licensed under the [MIT License](LICENSE).
