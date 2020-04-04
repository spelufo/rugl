#version 300 es

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_color;

out vec3 color;

void main() {
    color = a_color;
    gl_Position = vec4(a_position, 1.0);
}
