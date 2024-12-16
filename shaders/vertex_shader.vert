#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 scale_matrix;
uniform mat4 pitch_matrix;
uniform mat4 roll_matrix;
uniform mat4 yaw_matrix;

void main() {
    mat4 matrix = pitch_matrix * roll_matrix * yaw_matrix * scale_matrix;
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    gl_Position = matrix * vec4(position, 1.0);
}