#ifndef VECTOR2D_H
#define VECTOR2D_H

#include <cmath>

/**
 * Simple 2D vector class for simulation calculations
 */
class Vector2D {
public:
    double x;
    double y;
    
    Vector2D() : x(0.0), y(0.0) {}
    Vector2D(double x, double y) : x(x), y(y) {}
    
    // Vector addition
    Vector2D operator+(const Vector2D& other) const {
        return Vector2D(x + other.x, y + other.y);
    }
    
    // Vector subtraction
    Vector2D operator-(const Vector2D& other) const {
        return Vector2D(x - other.x, y - other.y);
    }
    
    // Scalar multiplication
    Vector2D operator*(double scalar) const {
        return Vector2D(x * scalar, y * scalar);
    }
    
    // Magnitude
    double magnitude() const {
        return std::sqrt(x * x + y * y);
    }
    
    // Normalize
    Vector2D normalized() const {
        double mag = magnitude();
        if (mag > 0.0) {
            return Vector2D(x / mag, y / mag);
        }
        return Vector2D(0.0, 0.0);
    }
};

#endif // VECTOR2D_H
