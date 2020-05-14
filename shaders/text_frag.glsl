#version 300 es

precision highp float;

in vec2 position;
in vec2 tex_coords;

out vec4 FragColor;

uniform sampler2D texture0;


void main() {
    vec3 magenta = vec3(1.0, 0.0, 1.0);
    vec3 black = vec3(0.0, 0.0, 0.0);
    FragColor = vec4(mix(magenta, black, texture(texture0, tex_coords).a), 1.0);
}