attribute vec3 inPos;

uniform mat3 u_matrix;

void main() {
    // Multiply the position by the matrix.
    gl_Position = u_matrix * inPos;
}