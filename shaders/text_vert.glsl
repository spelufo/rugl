#version 300 es

layout (location = 0) in vec2 a_position;
layout (location = 1) in vec2 a_tex_coords;

uniform vec2 screen_size;

out vec2 position;
out vec2 tex_coords;

void main() {
    position = a_position;
    tex_coords = a_tex_coords;
    gl_Position = vec4(
        -1.0 + 2.0 * a_position.x / screen_size.x,
         1.0 - 2.0 * a_position.y / screen_size.y,
        -1.0,
        1.0);
}
