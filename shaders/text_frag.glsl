#version 300 es

precision highp float;

in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec4 FragColor;

uniform sampler2D texture0;


void main() {
    FragColor = vec4(texture(texture0, tex_coords).aaa, 1.0);
}