#version 300 es

layout (location = 0) in vec3 a_position;
layout (location = 1) in vec2 a_tex_coords;
layout (location = 2) in vec3 a_normal;

uniform mat4 T_model;
uniform mat4 T_view;
uniform mat4 T_projection;

out vec3 position;
out vec2 tex_coords;
out vec3 normal;

void main() {
    position = a_position;
    tex_coords = a_tex_coords;
    normal = a_normal;
    gl_Position = T_projection * T_view * T_model * vec4(a_position, 1.0);
}
