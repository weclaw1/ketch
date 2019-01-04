#version 450
layout(location = 0) in vec2 o_tex_coord;
layout(location = 1) in vec3 o_normal;
layout(location = 2) in vec3 frag_position;

///outgoing final color
layout(location = 0) out vec4 f_color;

layout(binding = 1) uniform LightData {
  vec3 light_position;
  vec3 light_color;
} light_data;

layout(binding = 2) uniform sampler2D tex;

layout(push_constant) uniform PushConstants {
  bool light_source;
} push_constants;

const float AMBIENT_STRENGTH = 0.1;

void main() {
  if(push_constants.light_source) {
    f_color = vec4(light_data.light_color, 1.0);
  } else {
    // ambient
    vec3 ambient = AMBIENT_STRENGTH * light_data.light_color;

    // diffuse
    vec3 norm = normalize(o_normal);
    vec3 light_dir = normalize(light_data.light_position - frag_position);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = diff * light_data.light_color;

    f_color = vec4((ambient + diffuse) * vec3(texture(tex, o_tex_coord)), 1.0);
  }
}