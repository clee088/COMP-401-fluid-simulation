# Exploring Fluid Simulation

This project explores creating a 2D environment to simulate fluid using
smoothed-particle hydrodynamics (SPH). It is greatly-inspired by [Sebastian Lague](https://github.com/SebLague)'s amazing 
[fluid simulation video](https://www.youtube.com/watch?v=rSKMYc1CQHE).

The code is written in the [Rust](https://www.rust-lang.org/) programming 
language and uses the [Bevy](https://bevyengine.org/) game engine. Bevy uses an
Entity Component System (ECS) architecture, which you can learn more about
[here](https://bevyengine.org/learn/quick-start/getting-started/ecs/).

For more information about the proposal, design, and results of this project,
presentations can be found in the `presentations` directory.


## Installation Instructions

1. Ensure [Rust](https://www.rust-lang.org/) is installed.
2. Clone this repository.
3. `cd` into `fluid_simulation`.
4. Run `cargo run --release`.

That should be it! It may take a while for everything to get compiled.

## User Instructions

There is no user interaction in the project's current state. However, you can
control the simulation time, as well as adjust inital parameters in the
codebase before running the simulation.

### Time
- Press `space` to pause/unpause
- Press `arrow.up` to speed up time
- Press `arrow.down` to slow down time

### Initial Parameters

`main.rs`
- `BOUNDS`: the size of the boundary containing the particles.

`particles.rs`
- `NUM_PARTICLES`: the number of particles in the simulation.
- `PARTICLE_SIZE`: the size of a particle.
- `PARTICLE_SPACING`: the spacing between each particle when spawned in a grid.
    - use the `spawn_particles_randomly` method instead of `spawn_particles`
    to spawn the particles with random coordinates instead of a grid.

`movement.rs`
- `DEFAULT_GRAVITY`: the acceleration due to gravity.
- `DAMPENING_FACTOR`: the amount of velocity lost when a particle collides with
a boundary border.
- `SMOOTHING_RADIUS`: the smoothing radius of the smoothing function.
- `MASS`: the mass of a particle.
- `REST_DENSITY`: the target rest density in the simulation.
- `GAS_CONSTANT`: the gas constant in the simulation.

## Future Directions
- Integrate solution for boundary issues
- Optimize by breaking space into grids to improve search times
    - or write a compute shader
- Incorporate interactivity into simulation
    - geometric objects, mouse repels or attracts particles
- Implement UI to customize and toggle initial parameters