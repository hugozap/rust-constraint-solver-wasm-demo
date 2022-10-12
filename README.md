# Experiment with Cassowary Constraint Solvers, Rust and WASM

What's going on:

This repo contains a rust crate that uses the cassowary constraint solver library for rust.

It exports one struct called App that contains the state (point locations)

The App has a public method called `update_locations` that runs the solver.

Instead of returning the point location, the internal state variable is updated.

To access the point locations there's another exported function called `get_points`

We use the pointer to access the wasm memory buffer and retrieve the values updated by the solver.
