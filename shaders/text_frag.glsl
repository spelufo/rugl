#version 300 es

precision highp float;

in vec2 position;
in vec2 tex_coords;

out vec4 FragColor;

uniform sampler2D texture0;


void main() {
    vec3 magenta = vec3(1.0, 0.0, 1.0);
    vec3 black = vec3(0.0, 0.0, 0.0);
    float a = texture(texture0, tex_coords).a;
    if(a < 0.01)
        discard;
    FragColor = vec4(black, a);
    // FragColor = vec4(mix(black, magenta, a), 1.0);
}