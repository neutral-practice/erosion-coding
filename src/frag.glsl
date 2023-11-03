#version 460

layout(location = 0) in vec3 v_normal;
layout(location = 0) out vec4 f_color;

const vec3 LIGHT = vec3(0.0, 1.0, 0.0);

void main() {
    float brightness = dot(normalize(v_normal), normalize(LIGHT));
    vec3 dark_color = vec3(0.5, 0.5, 0.33);
    vec3 regular_color = vec3(1.0, 1.0, 0.6);

    f_color = vec4(mix(dark_color, regular_color, brightness), 1.0);
}
