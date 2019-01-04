#version 450
layout(location = 0) in vec2 o_tex_coord;

///outgoing final color
layout(location = 0) out vec4 f_color;

layout(binding = 1) uniform LightData {
  vec3 light_color;
} light_data;

layout(binding = 2) uniform sampler2D tex;

layout(push_constant) uniform PushConstants {
  bool light_source;
} push_constants;

void main() {
  if(push_constants.light_source) {
    f_color = vec4(light_data.light_color, 1.0);
  } else {
    f_color = vec4(light_data.light_color * vec3(texture(tex, o_tex_coord)), 1.0);
  }
}