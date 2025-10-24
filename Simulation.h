#ifndef SIMULATION_H
#define SIMULATION_H

#include "Link2D.h"
#include <vector>
#include <iostream>
#include <stdexcept>

class Simulation {
private:
    std::vector<Link2D> links;
    double time;
    double dt;  // Time step
    
public:
    Simulation(double timeStep = 0.01) : time(0.0), dt(timeStep) {}
    
    void addLink(double length, double initialAngle = 0.0) {
        links.push_back(Link2D(length, initialAngle));
    }
    
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
    
    double getTime() const {
        return time;
    }
    
    size_t getLinkCount() const {
        return links.size();
    }
    
    const Link2D& getLink(size_t index) const {
        if (index >= links.size()) {
            throw std::out_of_range("Link index out of bounds");
        }
        return links[index];
    }
    
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
