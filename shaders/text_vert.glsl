#version 300 es

layout (location = 0) in vec2 a_position;
layout (location = 1) in vec2 a_tex_coords;

uniform vec2 screen_size;

out vec2 position;
out vec2 tex_coords;

void main() {
    float screen_width = screen_size.x / 2.0;
    float screen_height = screen_size.y / 2.0;
    position = a_position;
    tex_coords = a_tex_coords;
    gl_Position = vec4(a_position.x / screen_width, a_position.y / screen_height, -1.0, 1.0);
}
