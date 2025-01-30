#pragma once
#include "Vector.h"

class Matrix3 {
public:
    float m[3][3];

    Matrix3() {
        // Initialize as identity matrix
        for (int i = 0; i < 3; i++)
            for (int j = 0; j < 3; j++)
                m[i][j] = (i == j) ? 1.0f : 0.0f;
    }

    // Static methods for creating transformation matrices
    static Matrix3 translation(float tx, float ty) {
        Matrix3 mat;
        mat.m[0][2] = tx;
        mat.m[1][2] = ty;
        return mat;
    }

    static Matrix3 rotation(float angle) {
        Matrix3 mat;
        float c = std::cos(angle);
        float s = std::sin(angle);
        mat.m[0][0] = c;  mat.m[0][1] = -s;
        mat.m[1][0] = s;  mat.m[1][1] = c;
        return mat;
    }

    static Matrix3 scaling(float sx, float sy) {
        Matrix3 mat;
        mat.m[0][0] = sx;
        mat.m[1][1] = sy;
        return mat;
    }

    // Matrix multiplication
    Matrix3 operator*(const Matrix3& other) const {
        Matrix3 result;
        for (int i = 0; i < 3; i++) {
            for (int j = 0; j < 3; j++) {
                result.m[i][j] = 0;
                for (int k = 0; k < 3; k++) {
                    result.m[i][j] += m[i][k] * other.m[k][j];
                }
            }
        }
        return result;
    }

    // Transform a vector
    Vector2 transform(const Vector2& v) const {
        return Vector2(
            m[0][0] * v.x + m[0][1] * v.y + m[0][2],
            m[1][0] * v.x + m[1][1] * v.y + m[1][2]
        );
    }
};
