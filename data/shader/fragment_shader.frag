#version 450
layout(location = 0) in vec2 o_tex_coord;

///outgoing final color
layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) uniform sampler2D tex;

void main() {
  f_color = texture(tex, o_tex_coord);
}