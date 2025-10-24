# 2D-Arm

A basic simulation framework for a 2D articulated arm.

## Overview

This project provides a simple simulation groundwork for modeling a 2D articulated arm with multiple links. The simulation framework allows you to create chains of links and step through time to observe their behavior.

## Structure

- **Vector2D.h**: 2D vector math utilities for position and direction calculations
- **Link2D.h**: Represents a single link in the articulated arm
- **Simulation.h**: Main simulation framework that manages multiple links and time stepping
- **main.cpp**: Example usage showing how to create and run a simulation

## Building

This project uses CMake for building:

```bash
mkdir build
cd build
cmake ..
make
```

## Running

After building, run the simulation:

```bash
./bin/arm_simulation
```

## Usage Example

```cpp
#include "Simulation.h"

int main() {
    // Create simulation with 0.01s time step
    Simulation sim(0.01);
    
    // Add links to create a 2-link arm
    sim.addLink(1.0, 0.0);  // length 1.0, angle 0.0 rad
    sim.addLink(0.8, 0.0);  // length 0.8, angle 0.0 rad
    
    // Step through simulation
    for (int i = 0; i < 100; ++i) {
        sim.step();
    }
    
    // Print current state
    sim.printState();
    
    return 0;
}
```

## Features

- Simple 2D vector mathematics
- Link-based arm structure with position and angle tracking
- Time-stepped simulation
- Chain connectivity (each link connects to the previous one)
- State inspection and printing

## Note

This is a basic simulation framework. No control logic is implemented - it provides only the groundwork for simulating a 2D articulated arm.