#include "Simulation.h"
#include <iostream>

int main() {
    std::cout << "2D Arm Simulation" << std::endl;
    std::cout << "=================" << std::endl << std::endl;
    
    // Create simulation with 0.01s time step
    Simulation sim(0.01);
    
    // Create a 2-link arm
    // Link 1: length 1.0, initial angle 0.0 radians
    sim.addLink(1.0, 0.0);
    // Link 2: length 0.8, initial angle 0.0 radians
    sim.addLink(0.8, 0.0);
    
    std::cout << "Created " << sim.getLinkCount() << "-link arm" << std::endl << std::endl;
    
    // Run simulation for a short duration
    std::cout << "Initial state:" << std::endl;
    sim.printState();
    std::cout << std::endl;
    
    // Simulate for 1 second (100 steps)
    for (int i = 0; i < 100; ++i) {
        sim.step();
    }
    
    std::cout << "State after 1 second:" << std::endl;
    sim.printState();
    std::cout << std::endl;
    
    std::cout << "Simulation complete." << std::endl;
    
    return 0;
}
