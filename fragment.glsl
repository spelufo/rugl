#version 300 es

precision highp float;

in vec3 color;
// uniform sampler2D tex;
// uniform int mode;
// uniform float opacity;

// in vec2 uvs;

out vec4 FragColor;

void main() {
    FragColor = vec4(color, 1.0);
}