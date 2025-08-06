# Chaikin Curve Smoothing Visualizer

This is a simple interactive visualizer for Chaikin's curve smoothing algorithm, built with Rust and [macroquad](https://github.com/not-fl3/macroquad).

## Features

- Click to add points to the canvas.
- Press <kbd>Enter</kbd> to start smoothing (requires at least 3 points).
- Press <kbd>C</kbd> to clear and restart.
- Press <kbd>Escape</kbd> to quit.
- Displays the number of smoothing steps (up to 7).

## Usage

1. **Install Rust**:  
   [Get Rust here](https://www.rust-lang.org/tools/install)

2. **Add macroquad dependency**:  
   Add the following to your `Cargo.toml` under `[dependencies]`:
   ```toml
   macroquad = "0.4"
   ```

3. **Run the app**:
   ```sh
   cargo run
   ```

4. **Controls**:
   - **Left Mouse Button**: Add points.
   - **Enter**: Start smoothing (≥3 points) or draw a line (2 points).
   - **C**: Clear all points.
   - **Escape**: Exit the app.

## How it works

The app implements Chaikin's algorithm to smooth a polyline. Each time smoothing is triggered, the algorithm refines the curve by generating new points, creating a visually smoother path.

## File structure

- [`src/main.rs`](src/main.rs): Main application logic.
- [`Cargo.toml`](Cargo.toml): Project configuration.

## License

MIT