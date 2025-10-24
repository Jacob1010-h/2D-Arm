#ifndef SIMULATION_H
#define SIMULATION_H

#include "Link2D.h"
#include <vector>
#include <iostream>

/**
 * Main simulation class for the 2D articulated arm
 */
class Simulation {
private:
    std::vector<Link2D> links;
    double time;
    double dt;  // Time step
    
public:
    Simulation(double timeStep = 0.01) : time(0.0), dt(timeStep) {}
    
    /**
     * Add a link to the arm
     */
    void addLink(double length, double initialAngle = 0.0) {
        links.push_back(Link2D(length, initialAngle));
    }
    
    /**
     * Update all links in the chain
     */
    void step() {
        // Update positions based on chain connectivity
        for (size_t i = 0; i < links.size(); ++i) {
            if (i == 0) {
                // First link starts at origin
                links[i].position = Vector2D(0.0, 0.0);
            } else {
                // Each subsequent link starts where the previous one ends
                links[i].position = links[i-1].getEndPosition();
            }
            
            // Update the link state
            links[i].update(dt);
        }
        
        time += dt;
    }
    
    /**
     * Get the current time in the simulation
     */
    double getTime() const {
        return time;
    }
    
    /**
     * Get number of links
     */
    size_t getLinkCount() const {
        return links.size();
    }
    
    /**
     * Get a specific link
     */
    const Link2D& getLink(size_t index) const {
        return links[index];
    }
    
    /**
     * Print current state of the arm
     */
    void printState() const {
        std::cout << "Time: " << time << "s" << std::endl;
        for (size_t i = 0; i < links.size(); ++i) {
            Vector2D endPos = links[i].getEndPosition();
            std::cout << "  Link " << i << ": "
                      << "angle=" << links[i].angle << " rad, "
                      << "pos=(" << links[i].position.x << ", " << links[i].position.y << "), "
                      << "end=(" << endPos.x << ", " << endPos.y << ")"
                      << std::endl;
        }
    }
};

#endif // SIMULATION_H
