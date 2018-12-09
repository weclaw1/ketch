#version 450
layout(location = 0) in vec4 o_color;

///outgoing final color
layout(location = 0) out vec4 f_color;

void main() {
  f_color = o_color;
}