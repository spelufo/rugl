#version 300 es

precision highp float;

in vec3 position;
in vec3 tex_coords;
in vec3 normal;

out vec4 FragColor;

void main() {
    vec3 light_pos = vec3(4.0, 3.0, -5.0);
    vec3 backlight_pos = vec3(-4.0, 5.0, 10.0);
    float d = distance(position, light_pos);
    float d2 = distance(position, backlight_pos);
    vec3 ambient = 0.5 * vec3(0.5, 0.5, 0.5);
    float diffuse = 25.0 * max(0.0, dot(normal, normalize(light_pos - position))) / (d * d);
    float diffuse_back = 25.0 * max(0.0, dot(normal, normalize(backlight_pos - position))) / (d2 * d2);
    FragColor = vec4(ambient + diffuse_back * vec3(0.0, 0.1, 1.0) + diffuse * vec3(1.0, 1.0, 0.0), 1.0);
}