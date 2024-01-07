use clap::Parser;
use comfy::*;

pub static CASCADE_CANVAS: OnceCell<AtomicRefCell<CascadeCanvas>> =
    OnceCell::new();

pub struct CascadeCanvas {
    width: i32,
    height: i32,
}

struct Grass;
struct ComfyBoid {
    behavior: BoidBehavior,
    neighbor_distance: f32,
    num_neighbors: i32,
    neighbors: Vec<Entity>,
}
pub const BOID_COUNT: i32 = 66;
pub const Z_BOIDS: i32 = 5;
pub const MIN_DISTANCE: f32 = 8.;
simple_game!("Mishka Shader", GameState, setup, update);

pub struct GameState {
    pub my_shader_id: Option<ShaderId>,
    pub intensity: f32,
    pub pattern_intensity: f32,
    pub rules: Option<Rules>,
    pub seed: [u32; 2],
}
#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 's', long)]
    _seed: Option<Vec<u32>>,
}

pub struct Rules {
    alignment: f32,
    cohesion: f32,
    max_velocity: f32,
}
enum BoidBehavior {
    Flocking,
    Chasing,
    Fleeing,
}
pub fn gen_seed() -> [u32; 2] {
    let rand_val: [u32; 2] = [rand(), rand()];
    rand_val
}

impl GameState {
    pub fn new(_c: &mut EngineState) -> Self {
        Self {
            my_shader_id: None,
            intensity: 1.0,
            pattern_intensity: 1.0,
            seed: [111111, 22222],
            rules: Some(Rules {
                alignment: 0.0,
                cohesion: 0.0,
                max_velocity: 0.0,
            }),
        }
    }
}

fn setup(_state: &mut GameState, _c: &mut EngineContext) {
    use BoidBehavior::*;
    _c.load_texture_from_bytes(
        "grass",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/grass.png"
        )),
    );
    _c.load_texture_from_bytes(
        "boid",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/chicken.png"
        )),
    );

    // for x in 0..50 {
    //     for y in 0..50 {
    //         let variant = random_i32(0, 2);
    //         // Tile the world with random variant of grass sprite
    //         commands().spawn((
    //             Sprite::new("grass".to_string(), vec2(1.0, 1.0), 0, WHITE)
    //                 .with_rect(32 * variant, 0, 32, 32),
    //             Transform::position(vec2(x as f32, y as f32)),
    //             Grass,
    //         ));
    //     }
    // }


    game_config_mut().bloom_enabled = true;
    // let mut builder = Builder::new();
    // builder.filter()
    let _args = Args::parse();
    let seed = gen_seed();
    _state.seed = seed;
    info!("shader demo with seed {:?}", _state.seed);
    for _x in 0..BOID_COUNT {
        for _y in 0..BOID_COUNT {
            commands().spawn((
                Transform::position(
                    vec2(random() * 300. / 2.0, random() * 300. / 2.0) +
                        splat(0.5),
                ),
                Sprite::new("boid", splat(1.0), Z_BOIDS, WHITE)
                    .with_rect(0, 0, 18, 18),
                // We tag these so that we can query them later.
                ComfyBoid {
                    behavior: Flocking,
                    neighbor_distance: 2.,
                    neighbors: Vec::new(),
                    num_neighbors: 0,
                                    
                },
                AnimatedSpriteBuilder::new()
                    .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                        name: "boid".into(),
                        offset: ivec2(0, 0),
                        step: ivec2(16, 0),
                        size: isplat(16),
                        frames: 1,
                    })
                    .add_animation("walk", 0.05, true, AnimationSource::Atlas {
                        name: "boid".into(),
                        offset: ivec2(16, 0),
                        step: ivec2(16, 0),
                        size: isplat(16),
                        frames: 6,
                    })
                    .build(),
            ));
        }
    }
}
pub fn add_vec2(a: Vec2, b: Vec2) -> Vec2 {
    Vec2 { x: a.x + b.x, y: a.y + b.y }
}
fn update(state: &mut GameState, c: &mut EngineContext) {
    if state.my_shader_id.is_none() {
        state.my_shader_id = Some(
            // Comfy now supports shader hot reloading. We'll create a simple shader and provide
            // both `static_source` which would be used in release builds allowing the game to be
            // shipped with shaders embedded in the binary, as well as a path for hot reloading,
            // which will be watched by Comfy and hot-reloaded on change.
            //
            // Note that currently hot reloading an invalid shader will log the error in the
            // terminal, but will automatically fall back to the previous shader that compiled.
            create_reloadable_sprite_shader(
                &mut c.renderer.shaders.borrow_mut(),
                "mishka-shader",
                ReloadableShaderSource {
                    static_source: include_str!("funfun-shader.wgsl")
                        .to_string(),
                    path: "comfy/examples/funfun-shader.wgsl".to_string(),
                },
                // Uniforms can have default values. When we switch to this shader we'll have to
                // set all the uniforms that don't have a default value before drawing anything
                // using the shader, otherwise we'll get a crash.
                //
                // In this case we don't provide a default for "time" as we'll set it every frame,
                // but we will provide a value for "intensity" just to showcase how this would
                // work.
                //
                // Experiment with this to learn what happens in different scenarios!
                //
                // If you change "intensity" default to `None` you'll get a crash saying which
                // uniform was missing a value.
                hashmap! {
                    "time".to_string() => UniformDef::F32(None),
                    "intensity".to_string() => UniformDef::F32(Some(1.0)),
                    "max_velocity".to_string() => UniformDef::F32(Some(1.0)),
                    "alignment".to_string() => UniformDef::F32(Some(1.0)),
                    "cohesion".to_string() => UniformDef::F32(Some(1.0)),
                    // "pattern intensity".to_string() => UniformDef::F32(Some(1.0)),
                    "seed1".to_string() => UniformDef::F32(Some(1.0)),
                    "seed2".to_string() => UniformDef::F32(Some(1.0)),


                },
            )
            .unwrap(),
        )
    }


    let shader_id = state.my_shader_id.unwrap();
    let rules = state.rules.as_mut().unwrap();

    // First draw with a default shader.
    // draw_comfy(vec2(-2.0, 0.0), WHITE, 0, splat(1.0));

    egui::Window::new("Uniforms")
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, -100.0))
        .show(egui(), |ui| {
            ui.label("HDR intensity");
            ui.add(egui::Slider::new(&mut state.intensity, 1.0..=5.0));
            ui.label("Pattern intensity");
            ui.add(egui::Slider::new(&mut state.intensity, 1.0..=5.0));
            ui.label("Alignment force");
            ui.add(egui::Slider::new(&mut rules.alignment, 0.0..=1.0));
            ui.label("Cohesion force");
            ui.add(egui::Slider::new(&mut rules.cohesion, 0.0..=1.0));
            ui.label("Max velocity");
            ui.add(egui::Slider::new(&mut rules.max_velocity, 0.0..=7.));
        });

    // When we switch a shader the uniforms will get their default value
    // use_shader(shader_id);

    let time = get_time() as f32;

    // We can only set one and then draw and the other uniform will be set
    // to the default value we specified when creating the shader.
    set_uniform_f32("time", time);
    set_uniform_f32("intensity", state.intensity);
    // set_uniform("seed", state.seed);

    // draw_comfy(vec2(0.0, 0.0), WHITE, 0, splat(1.0));

    // This will set "intensity" while retaining "time" from the previous set in this frame, as
    // expected. None of this should be surprising, other than the fact that we can draw in between
    // `set_uniform` calls and things will _just work_.
    //
    // Note that doing things like this will result in the draw calls not being batched together
    // and instead be done in two separate render passes. This is unavoidable and should be
    // expected, but we're mentioning it here just for extra clarity.
    set_uniform_f32("intensity", state.intensity);

    // set_uniform_f32("pattern intensity", state.pattern_intensity);
    // draw_comfy(vec2(2.0, 0.0), WHITE, 0, splat(1.0));

    // We can also easily switch back to the default sprite shader.
    // use_default_shader();
    // draw_comfy(vec2(4.0, 0.0), WHITE, 0, splat(1.0));
    use_shader(shader_id);
    let time = get_time() as f32;
    set_uniform_f32("time", time);
    set_uniform_f32("intensity", state.intensity);
    set_uniform_f32("seed1", *state.seed.first().unwrap() as f32);
    set_uniform_f32("seed2", *state.seed.get(1).unwrap() as f32);
    set_uniform_f32("alignment", rules.alignment);
    set_uniform_f32("cohesion", rules.cohesion);
    set_uniform_f32("max_velocity", rules.max_velocity);

    srand(*state.seed.first().unwrap() as u64);

    // for x in 0..BOID_COUNT {
    //     for y in 0..BOID_COUNT {
    //     commands().spawn((Transform::position(
    //     vec2(random_range(1., 20.), random_range(1., 20.)) + splat(0.5),
    //             ),
    //             Sprite::new("comfy", splat(1.0), Z_BOIDS, WHITE)

    //                 .with_rect(0, 0, 16, 16),
    //             // We tag these so that we can query them later.
    //             ComfyBoid,))
    //                 }
    // }
    let mut avg_close_pos = Vec2::ZERO;
    let mut avg_move_dir = Vec2::ZERO;
    let mut boid_pos = Vec2::ZERO;
    let mut coshesion_x = 0;
    let mut cohesion_y = 0;
    let mut neighbors = 0;

    for (_, (boid, sprite, transform)) in &mut world()
        .query::<(&mut ComfyBoid, &mut AnimatedSprite, &mut Transform)>()
        .iter()
    {
        boid_pos = transform.position;
        let mut moved = false;
        // let neighbors: HashMap<Entity, &Transform> = world().query::<&Transform>().iter().filter(|(_, t)| 
        //     t.position.distance(transform.position) < rules.cohesion && t.position != transform.position)
        //     .map(|(e, t)| (e, t)).collect();
        // let distance_from_neighbors = transform.position.distance(neighbors);
        // let neighbors = get_neighbors(transform.position, rules.cohesion);
        avg_move_dir.x = random_range(-1., 1.);
        avg_move_dir.y = random_range(-1., 1.);
        let vel_x = random_range(1., rules.max_velocity) * 0.05;
        let vel_y = random_range(1., rules.max_velocity) * 0.05;
        let normalized = avg_move_dir.normalize_or_right();
        transform.position.x += vel_x * normalized.x ;
        transform.position.y += vel_y * normalized.y ;
        if transform.position.distance(boid_pos) < rules.cohesion  && transform.position != boid_pos {
            info!("inside cohesion distance");
            coshesion_x += boid_pos.x as i32;
            cohesion_y += boid_pos.y as i32;
            // avg_close_pos = add_vec2(avg_close_pos, boid_pos);
            // let normalized = avg_close_pos.normalize_or_zero();
            boid.num_neighbors += 1;
            neighbors += 1;
        }     
        // } else  {
        if (neighbors > 0) {
                info!("outside cohesion distance");
                let cohesion_radius = random_range(1., rules.cohesion);
                let distance = transform.position.distance(boid_pos);
                info!("distance {:?}", distance);
                coshesion_x = coshesion_x / neighbors;
                cohesion_y = cohesion_y / neighbors;
                avg_close_pos=  Vec2 {x: transform.position.x - boid_pos.x * rules.cohesion, y: transform.position.y - boid_pos.y * rules.cohesion};

                let normalized = avg_close_pos.normalize_or_right();
                info!("normalized avg_close_pos {:?}", normalized);
                let vel_x = random_range(1., rules.max_velocity) * 0.05;
                let vel_y = random_range(1., rules.max_velocity) * 0.05;
                transform.position.x += vel_x *  normalized.x - 1. 
                    ;
                transform.position.y += vel_y * normalized.y - 1. ;
        }
        if transform.position.x  >= 300.{
            transform.position.x = 20.;

        }
        if transform.position.y  >= 300.{
            transform.position.y = 20.;
        }
        if transform.position.x  <= -300.{
            transform.position.x = 20.;

        }
        if transform.position.y  <= -300.{
            transform.position.y = 20.;
        }

        // let neighbors = world().query::<&Position>().iter().filter(|(_, t)| 
        //     t.distance(boid_pos) < rules.cohesion && t != &boid_pos)
        //     .map(|(e, t)| (e, t)).collect();
        // if distance_from_neighbors < rules.cohesion {
        //     avg_close_pos = add_vec2(avg_close_pos, boid_pos);
        //     transform.position += avg_close_pos / neighbors as f32;
        // }
         

         


        moved = true;
        if moved {
            sprite.flip_x = vel_x < 0.;
            sprite.play("walk");
        } else {
            sprite.play("idle");
        }
    }
    // for (_, (transform, boid)) in world().query::<(&mut Transform, &mut ComfyBoid)>().iter() {
    //     if transform.position.distance(boid_pos) < rules.cohesion {
    //         info!("inside cohesion distance");
    //         avg_close_pos = add_vec2(avg_close_pos, boid_pos);
    //         let normalized = avg_close_pos.normalize_or_zero();
    //         boid.num_neighbors += 1;                                         
    //     } else if rules.cohesion > 0.2 {
    //             info!("outside cohesion distance");
    //             let distance = transform.position.distance(boid_pos);
    //             info!("distance {:?}", distance);
    //             avg_close_pos = add_vec2(avg_close_pos, boid_pos);
    //             let normalized = avg_close_pos.normalize_or_right();
    //             info!("normalized avg_close_pos {:?}", normalized);
    //             transform.position.x += normalized.x - 1.;
    //             transform.position.y += normalized.y - 1.;
    //             // transform.position += avg_close_pos;
    //         }
    // }
            // avg_close_pos = add_vec2(avg_close_pos, transform.position);
            // transform.position += avg_close_pos ;
                    
    
    // }
    // for (boid_idx, (_, transform)) in
    //     world().query::<(&ComfyBoid, &mut Transform)>().iter()
    // {
    //     let mut avg_pos = Vec2::ZERO;
    //     let mut avg_move_dir = Vec2::ZERO;
    //     let mut avg_close_pos = Vec2::ZERO;
    //     let mut neighbors = 0;


    //     let other_pos = transform.position;
    //     let distance = transform.position.distance(other_pos);
    //     if distance < rules.cohesion {
    //         avg_close_pos = add_vec2(avg_close_pos, other_pos);
    //         neighbors += 1;
    //     }
    //     avg_pos = add_vec2(avg_pos, other_pos);

    //     // avg_move_dir += ;
    // }


    if is_mouse_button_pressed(MouseButton::Left) {
        draw_circle(mouse_world(), 1., WHITE, 0);
    }
    fn get_neighbors(pos: Vec2, radius: f32, ) -> Vec<Entity> {
        let mut neighbors = Vec::new();
        for (idx, (_, transform)) in
            world().query::<(&ComfyBoid, &mut Transform)>().iter()
        {
            let other_pos = transform.position;
            let distance = pos.distance(other_pos);
            if distance < radius {
                neighbors.push(idx);
            }
        }
        neighbors
    }
}

pub fn wrap_window() {
    todo!()
}
impl Rules {
    pub fn new(alignment: f32, cohesion: f32, max_velocity: f32) -> Self {
        Self { alignment, cohesion, max_velocity }
    }
}
//experment with 2.5d
// fn draw_room(pos: Vec2) {
//     let pos = pos.extend(0.0);
// draw_mesh_ex(Mesh {
//     vertices: vec![
//         SpriteVertex::new()
//     ]
// })
// }
