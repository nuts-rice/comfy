struct Mouse {
pos: vec2<f32> ,
}

fn hash(n: vec2<u32>)  -> u32 {
let p1 = 1103515245u;
var p = p1 * ((n >> 1u) ^ (n.yx));
var h32 = p1 * ((p.x) ^ (p.y >> 3u)); 
return h32^(h32 >> 16u);
}


//var m2: mat2x2f = mat2x2f(vec2f(0.8, 0.6), vec2f(-0.6, 0.8));
fn rand22(n: vec2f) -> f32 { return fract(sin(dot(n, vec2f(12.9898, 4.1414))) * 43758.5453); }
fn noise(n: vec2f) -> f32 {
let d = vec2f(0., 1.);
    let b = floor(n);
    let f = smoothstep(vec2f(0.), vec2f(1.), fract(n));
    return mix(mix(rand22(b), rand22(b + d.yx), f.x), mix(rand22(b + d.xy), rand22(b + d.yy), f.x), f.y);
}

fn mod289(x: vec2f) -> vec2f {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn mod289_3(x: vec3f) -> vec3f {
    return x - floor(x * (1. / 289.)) * 289.;
}

fn permute3(x: vec3f) -> vec3f {
    return mod289_3(((x * 34.) + 1.) * x);
}

//  MIT License. © Ian McEwan, Stefan Gustavson, Munrocket
fn simplexNoise2(v: vec2f) -> f32 {
    let C = vec4(
        0.211324865405187, // (3.0-sqrt(3.0))/6.0
        0.366025403784439, // 0.5*(sqrt(3.0)-1.0)
        -0.577350269189626, // -1.0 + 2.0 * C.x
        0.024390243902439 // 1.0 / 41.0
    );

    // First corner
    var i = floor(v + dot(v, C.yy));
    let x0 = v - i + dot(i, C.xx);

    // Other corners
    var i1 = select(vec2(0., 1.), vec2(1., 0.), x0.x > x0.y);

    // x0 = x0 - 0.0 + 0.0 * C.xx ;
    // x1 = x0 - i1 + 1.0 * C.xx ;
    // x2 = x0 - 1.0 + 2.0 * C.xx ;
    var x12 = x0.xyxy + C.xxzz;
    x12.x = x12.x - i1.x;
    x12.y = x12.y - i1.y;

    // Permutations
    i = mod289(i); // Avoid truncation effects in permutation

    var p = permute3(permute3(i.y + vec3(0., i1.y, 1.)) + i.x + vec3(0., i1.x, 1.));
    var m = max(0.5 - vec3(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3(0.));
    m *= m;
    m *= m;

    // Gradients: 41 points uniformly over a line, mapped onto a diamond.
    // The ring size 17*17 = 289 is close to a multiple of 41 (41*7 = 287)
    let x = 2. * fract(p * C.www) - 1.;
    let h = abs(x) - 0.5;
    let ox = floor(x + 0.5);
    let a0 = x - ox;

    // Normalize gradients implicitly by scaling m
    // Approximation of: m *= inversesqrt( a0*a0 + h*h );
    m *= 1.79284291400159 - 0.85373472095314 * (a0 * a0 + h * h);

    // Compute final noise value at P
    let g = vec3(a0.x * x0.x + h.x * x0.y, a0.yz * x12.xz + h.yz * x12.yw);
    return 130. * dot(m, g);
}


fn fbm(p: vec2<f32>, seed: f32) -> f32 {
    var f: f32 = 0.;
    f = f + 0.5000 * simplexNoise2(p * seed); 
    //p = m2 * p * 2.02;
    f = f + 0.2500 * simplexNoise2(p * seed); 
    //p = m2 * p * 2.03;
    f = f + 0.1250 * simplexNoise2(p * seed); 
    //p = m2 * p * 2.01;
    f = f + 0.0625 * simplexNoise2(p * seed);
    return f / 0.9375;
}

fn draw_mouse(mouse: Mouse) -> vec4<f32> {
let mouse_pos = vec2<f32>(mouse.pos);
}
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let tex = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // Todo: Smooth mouse drawing conditional
    //    if (in.tex_coords.y != 0.5 || in.tex_coords.x != 0.5) {
    //  break;
    //}

    var final_color: vec4<f32> = tex * in.color;
    let seed: vec2<f32> = vec2f(f32(seed1), f32(seed2));
    let noise = noise(in.tex_coords * 10.0 + seed);

    // ***************************************************************
    // We can use our uniforms here directly by name. Their WGSL
    // declarations are automatically generated, mapped and checked
    // at runtime by Comfy.
    // ***************************************************************
    final_color.r = final_color.r * abs(cos(time * 3.0));
    final_color.g = final_color.g * abs(cos(time * 3.0));  
    final_color.b = final_color.b * abs(cos(time * 3.0));
    final_color = final_color * intensity;

    return final_color;
}

