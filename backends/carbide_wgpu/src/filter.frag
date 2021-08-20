#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in vec2 v_tex_coords;
layout(location=2) flat in uint v_mode;

layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set = 0, binding = 2) uniform texture2D t_text_texture;
layout(set = 0, binding = 3) uniform texture2D atlas_texture;

const vec4[] gaussKernel3x3 =
{
//vec4(7.0, 7.0, 0.0, 1.0),

vec4(-3.0, -3.0, 0.0, 1.0 / 7.0),
vec4(-2.0, -2.0, 0.0, 1.0 / 7.0),
vec4(-1.0, -1.0, 0.0, 1.0 / 7.0),
vec4(0.0, 0.0, 0.0, 1.0 / 7.0),
vec4(1.0, 1.0, 0.0, 1.0 / 7.0),
vec4(2.0, 2.0, 0.0, 1.0 / 7.0),
vec4(3.0, 3.0, 0.0, 1.0 / 7.0),

/*vec4(-3.0, -3.0, 0.0, 1.0 / 4096.0),
vec4(-3.0, -2.0, 0.0, 6.0 / 4096.0),
vec4(-3.0, -1.0, 0.0, 15.0 / 4096.0),
vec4(-3.0, 0.0, 0.0, 20.0 / 4096.0),
vec4(-3.0, 1.0, 0.0, 15.0 / 4096.0),
vec4(-3.0, 2.0, 0.0, 6.0 / 4096.0),
vec4(-3.0, 3.0, 0.0, 1.0 / 4096.0),

vec4(-2.0, -3.0, 0.0, 6.0 / 4096.0),
vec4(-2.0, -2.0, 0.0, 36.0 / 4096.0),
vec4(-2.0, -1.0, 0.0, 90.0 / 4096.0),
vec4(-2.0, 0.0, 0.0, 120.0 / 4096.0),
vec4(-2.0, 1.0, 0.0, 90.0 / 4096.0),
vec4(-2.0, 2.0, 0.0, 36.0 / 4096.0),
vec4(-2.0, 3.0, 0.0, 6.0 / 4096.0),

vec4(-1.0, -3.0, 0.0, 15.0 / 4096.0),
vec4(-1.0, -2.0, 0.0, 90.0 / 4096.0),
vec4(-1.0, -1.0, 0.0, 225.0 / 4096.0),
vec4(-1.0, 0.0, 0.0, 300.0 / 4096.0),
vec4(-1.0, 1.0, 0.0, 225.0 / 4096.0),
vec4(-1.0, 2.0, 0.0, 90.0 / 4096.0),
vec4(-1.0, 3.0, 0.0, 15.0 / 4096.0),

vec4(0.0, -3.0, 0.0, 20.0 / 4096.0),
vec4(0.0, -2.0, 0.0, 120.0 / 4096.0),
vec4(0.0, -1.0, 0.0, 300.0 / 4096.0),
vec4(0.0, 0.0, 0.0, 400.0 / 4096.0),
vec4(0.0, 1.0, 0.0, 300.0 / 4096.0),
vec4(0.0, 2.0, 0.0, 120.0 / 4096.0),
vec4(0.0, 3.0, 0.0, 20.0 / 4096.0),

vec4(1.0, -3.0, 0.0, 15.0 / 4096.0),
vec4(1.0, -2.0, 0.0, 90.0 / 4096.0),
vec4(1.0, -1.0, 0.0, 225.0 / 4096.0),
vec4(1.0, 0.0, 0.0, 300.0 / 4096.0),
vec4(1.0, 1.0, 0.0, 225.0 / 4096.0),
vec4(1.0, 2.0, 0.0, 90.0 / 4096.0),
vec4(1.0, 3.0, 0.0, 15.0 / 4096.0),

vec4(2.0, -3.0, 0.0, 6.0 / 4096.0),
vec4(2.0, -2.0, 0.0, 36.0 / 4096.0),
vec4(2.0, -1.0, 0.0, 90.0 / 4096.0),
vec4(2.0, 0.0, 0.0, 120.0 / 4096.0),
vec4(2.0, 1.0, 0.0, 90.0 / 4096.0),
vec4(2.0, 2.0, 0.0, 36.0 / 4096.0),
vec4(2.0, 3.0, 0.0, 6.0 / 4096.0),

vec4(3.0, -3.0, 0.0, 1.0 / 4096.0),
vec4(3.0, -2.0, 0.0, 6.0 / 4096.0),
vec4(3.0, -1.0, 0.0, 15.0 / 4096.0),
vec4(3.0, 0.0, 0.0, 20.0 / 4096.0),
vec4(3.0, 1.0, 0.0, 15.0 / 4096.0),
vec4(3.0, 2.0, 0.0, 6.0 / 4096.0),
vec4(3.0, 3.0, 0.0, 1.0 / 4096.0),*/

/*vec4(-1.0, -1.0, 0.0,  1.0 / 16.0),
vec4(-1.0,  0.0, 0.0,  2.0 / 16.0),
vec4(-1.0, +1.0, 0.0,  1.0 / 16.0),
vec4( 0.0, -1.0, 0.0,  2.0 / 16.0),
vec4( 0.0,  0.0, 0.0,  4.0 / 16.0),
vec4( 0.0, +1.0, 0.0,  2.0 / 16.0),
vec4(+1.0, -1.0, 0.0,  1.0 / 16.0),
vec4(+1.0,  0.0, 0.0,  2.0 / 16.0),
vec4(+1.0, +1.0, 0.0,  1.0 / 16.0),*/
};

void main() {
    const vec2 texelSize = vec2(1.0) / vec2(640.0, 426.0);
    vec4 color = vec4(0.0);
    for (int i = 0; i < gaussKernel3x3.length(); ++i) {
        color += gaussKernel3x3[i].w * textureLod(sampler2D(t_diffuse, s_diffuse), v_tex_coords + texelSize * gaussKernel3x3[i].xy, 0);
    }
    f_color = color;
}