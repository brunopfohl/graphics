#include <iostream>
#include "../math/Vector.h"
#include "../math/Matrix.h"

int main() {
    // Create a 2D point at (1,0)
    Vector2 point(1.0f, 0.0f);
    std::cout << "Original point: (" << point.x << ", " << point.y << ")\n";

    // Create a rotation matrix (45 degrees)
    float angle = 3.14159f / 4.0f; // 45 degrees in radians
    Matrix3 rotation = Matrix3::rotation(angle);

    // Rotate the point
    Vector2 rotated = rotation.transform(point);
    std::cout << "Rotated point: (" << rotated.x << ", " << rotated.y << ")\n";

    // Create a translation matrix
    Matrix3 translation = Matrix3::translation(2.0f, 1.0f);

    // Translate the rotated point
    Vector2 final = translation.transform(rotated);
    std::cout << "Final point: (" << final.x << ", " << final.y << ")\n";

    // Demonstrate vector operations
    Vector2 v1(1.0f, 2.0f);
    Vector2 v2(2.0f, 3.0f);
    Vector2 sum = v1 + v2;
    float dot = v1.dot(v2);

    std::cout << "Vector sum: (" << sum.x << ", " << sum.y << ")\n";
    std::cout << "Dot product: " << dot << "\n";

    return 0;
}
