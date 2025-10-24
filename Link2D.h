#ifndef LINK2D_H
#define LINK2D_H

#include "Vector2D.h"
#include <cmath>

/**
 * Represents a single link in a 2D articulated arm
 */
class Link2D {
public:
    double length;        // Length of the link
    double angle;         // Current angle in radians
    double angularVel;    // Angular velocity
    Vector2D position;    // Position of the base of this link
    
    Link2D(double length, double initialAngle = 0.0) 
        : length(length), angle(initialAngle), angularVel(0.0), position(0.0, 0.0) {}
    
    /**
     * Get the end position of this link
     */
    Vector2D getEndPosition() const {
        return Vector2D(
            position.x + length * std::cos(angle),
            position.y + length * std::sin(angle)
        );
    }
    
    /**
     * Update link state based on time step
     */
    void update(double dt) {
        angle += angularVel * dt;
    }
};

#endif // LINK2D_H
