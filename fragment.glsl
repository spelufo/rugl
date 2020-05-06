#version 300 es

precision highp float;

in vec3 position;
in vec2 tex_coords;
in vec3 normal;

out vec4 FragColor;

uniform sampler2D texture0;

void main() {
    // vec3 sun_dir = vec3(-1., -5., -0.6);
    // vec3 sky_dir = -sun_dir;
    // vec3 sun_light_color = vec3(1., 1., .7);
    // vec3 sky_light_color = vec3(0., 0.05, 0.3);
    // vec3 sun_diffuse = sun_light_color * max(0., dot(normal, -sun_dir));
    // float sky_darkness = min(.5, max(0., dot(normal, sky_dir)));
    // vec3 sky_diffuse = sky_light_color * (1. - sky_darkness * sky_darkness* sky_darkness);
    // vec3 diffuse = vec3(0.,0.,0.);
    // diffuse += sun_diffuse;
    // diffuse += sky_diffuse;
    // FragColor = vec4(diffuse, 1.);

    vec3 ambient = vec3(0.1, 0.1, 0.3);
    vec3 light_pos = vec3(4.0, 6.0, -3.0);
    vec3 backlight_pos = vec3(-4.0, 5.0, 10.0);
    float d = distance(position, light_pos);
    float d2 = distance(position, backlight_pos);
    float diffuse = 25.0 * max(0.0, dot(normal, normalize(light_pos - position))) / (d * d);
    float diffuse_back = 25.0 * max(0.0, dot(normal, normalize(backlight_pos - position))) / (d2 * d2);
    // FragColor = vec4(ambient + diffuse_back * vec3(0.0, 0.1, 1.0) + diffuse * vec3(1.0, 1.0, 0.0), 1.0);
    FragColor = vec4(texture(texture0, tex_coords).rgb, 1.0);
    // FragColor = vec4(tex_coords.s, tex_coords.t, 0.3, 1.0);
}