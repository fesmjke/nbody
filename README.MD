## Particle Systems and N-body simulation
### About the project

This project implements an N-body simulation to model the motion of particles under the influence of gravitational forces.The simulation computes the interaction between multiple bodies, applying Newton's laws of motion and universal gravitation to predict their positions, velocities, and trajectories over time.

## Main Features:
- Simulates gravitational interactions between any number of bodies (particles).
- Uses numerical integration to update positions and velocities.
- Visualizes the movement of bodies in space.

## What is used in this project
- Rust
- [macroquad](https://macroquad.rs/)
### Prerequisites:
- **Rust**: Make sure you have Rust installed. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **Cargo**: Cargo comes bundled with Rust, and it will be used to build and run the project.

### Usage:
1. **Clone the Repository**:
   ```bash
   git clone https://github.com/fesmjke/nbody
   cd nbody
2. **Build project**:
   ```bash
   cargo build --release
   ```
3. **Start application**
   ```bash
   cargo run
   ```

## Todo
- [ ] UI:
  - [ ] Change variables in real time
  - [ ] Select single entity
  - [ ] ...
- [ ] Reset/Restart simulation
- [ ] Pause
- [ ] Barnes-Hut 
