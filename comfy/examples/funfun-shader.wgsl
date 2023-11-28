fn hash(n: vec2<u32>)  -> u32 {
let p1 = 1103515245u;
var p = p1 * ((n >> 1u) ^ (n.yx));
var h32 = p1 * ((p.x) ^ (p.y >> 3u)); 
return h32^(h32 >> 16u);
}

fn fbm(p: vec2<f32>) -> f32 {
var v = 0.0;

return v;

}
fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }
fn noise(n: vec2<f32>) -> f32 {
let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2f(0.), vec2f(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    var final_color: vec4<f32> = tex * in.color;
    let seed: vec2<u32> = vec2u(u32(seed1), u32(seed2));
//    let noisey
    // ***************************************************************
    // We can use our uniforms here directly by name. Their WGSL
    // declarations are automatically generated, mapped and checked
    // at runtime by Comfy.
    // ***************************************************************
    final_color.r = final_color.r * abs(cos(time * 3.0));
    final_color.g = final_color.g * abs(sin(time * 2.0));
    final_color.b = final_color.b * abs(cos(time * 5.0));
    final_color = final_color * intensity;

    return final_color;
}

