#pragma once
#include <cmath>

class Vector2 {
public:
    float x, y;

    Vector2(float x = 0.0f, float y = 0.0f) : x(x), y(y) {}

    // Basic vector operations
    Vector2 operator+(const Vector2& v) const { return Vector2(x + v.x, y + v.y); }
    Vector2 operator-(const Vector2& v) const { return Vector2(x - v.x, y - v.y); }
    Vector2 operator*(float scalar) const { return Vector2(x * scalar, y * scalar); }
    
    // Dot product
    float dot(const Vector2& v) const { return x * v.x + y * v.y; }
    
    // Magnitude
    float length() const { return std::sqrt(x * x + y * y); }
    
    // Normalization
    Vector2 normalize() const {
        float len = length();
        if (len != 0) return Vector2(x / len, y / len);
        return *this;
    }
};

class Vector3 {
public:
    float x, y, z;

    Vector3(float x = 0.0f, float y = 0.0f, float z = 0.0f) : x(x), y(y), z(z) {}

    // Basic vector operations
    Vector3 operator+(const Vector3& v) const { return Vector3(x + v.x, y + v.y, z + v.z); }
    Vector3 operator-(const Vector3& v) const { return Vector3(x - v.x, y - v.y, z - v.z); }
    Vector3 operator*(float scalar) const { return Vector3(x * scalar, y * scalar, z * scalar); }
    
    // Dot product
    float dot(const Vector3& v) const { return x * v.x + y * v.y + z * v.z; }
    
    // Cross product
    Vector3 cross(const Vector3& v) const {
        return Vector3(
            y * v.z - z * v.y,
            z * v.x - x * v.z,
            x * v.y - y * v.x
        );
    }
    
    // Magnitude
    float length() const { return std::sqrt(x * x + y * y + z * z); }
    
    // Normalization
    Vector3 normalize() const {
        float len = length();
        if (len != 0) return Vector3(x / len, y / len, z / len);
        return *this;
    }
};
