use comfy::*;

simple_game!("ECS Topdown Game", setup, update);

static WORLD_WIDTH: i32 = 50;
static WORLD_HEIGHT: i32 = 50;

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
pub enum Movement {
    Static,
    Random,
    RandomWaypoint { path: Option<Vec<usize>> },
}

#[derive(Clone, Debug)]
pub struct MoveMode {
    pub mode: Movement,
}

struct Player;
struct Grass;
struct Mob {
    pub position: Vec2,
    pub velocity: Vec2,
    pub move_mode: Movement,
    pub move_target: Vec2,
    pub move_timer: f32,
}

pub struct GameState {
    pub player: Player,
    pub mobs: Vec<Mob>,
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub rects: Vec<Rect>,
    pub tiles: Vec<TileType>,
}



enum TileType {
    Wall,
    Floor,
    Grass,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    pub walls: Vec<bool>,
}

fn piss_off_mob(_state: &mut GameState, _new_target: &mut Vec2) {
    unimplemented!()
}

fn random_move_mob(_state: &mut GameState, _new_target: &mut Vec2) {
    unimplemented!()
}

fn move_mobs(state: &mut GameState, time_delta: f32) {
    for mob in state.mobs.iter_mut() {
        let _new_pos = mob.position + mob.velocity * time_delta;
        // piss_off_mob(state, &mut new_pos);
    }
}

pub fn apply_room(state: &mut GameState, ) {

}
pub fn gen_rooms(state: &mut GameState, _c: &mut EngineContext) {
    state.rects.clear();
    state.rects.push(Rect::new(
        2.,
        2.,
        WORLD_WIDTH as f32 - 5.,
        WORLD_HEIGHT as f32 - 5.,
    ));
    let first_room = state.rects[0];
    add_subrects(state, first_room);
    let n_rooms = 0;
    while n_rooms < 15 {
        let _rect = get_random_rect(state);
        let canidate = get_random_sub_rect(_rect);
        if is_possible(state, canidate) {
            
            state.rooms.push(canidate);
            add_subrects(state, canidate);
        }
            
        
    }
}
pub fn is_possible(_state: &mut GameState, canidate: Rect) -> bool {
    let mut expanded = canidate;
    expanded.x -= 2.;
    expanded.w += 2.;
    expanded.y -= 2.;
    expanded.h += 2.;
    let mut possible = true;
    for y in expanded.y as i32..expanded.h as i32 {
        for x in expanded.x as i32..expanded.w as i32 {
            if x > WORLD_WIDTH - 2 {possible = false;}
            if y > WORLD_HEIGHT - 2 {possible = false;}
            if x < 1 {possible = false;}
            if y < 1 {possible = false;}
            // if possible {
            // let idx = ;
            // if state.map.walls[idx] {
                // possible = false;
        // } 
        }
    }
        possible
}


//TODO: evaluate if width and height are correct or need to switch around
pub fn get_random_sub_rect(_canidate: Rect) -> Rect {
    let mut result = _canidate;
    let rect_w = i32::abs(_canidate.x  as i32 - _canidate.w as i32);
    let rect_h = i32::abs(_canidate.y  as i32 - _canidate.h as i32);
    let w = i32::max(3, random_i32(1, i32::min(rect_w, 10)) -1) + 1;
    let h = i32::max(3, random_i32(1, i32::min(rect_h, 10)) -1) + 1;
    result.x += random_range(1., 6.) -1. ;
    result.y += random_range(1., 6.) -1. ;
    result.w = result.x + w as f32;
    result.h = result.y + h as f32;
    result

}
pub fn get_random_rect(state: &mut GameState) -> Rect {
    if state.rects.len() == 1 {
        return state.rects[0];
    }
    let idx = random_i32(0, state.rects.len() as i32 - 1) as usize;
    state.rects[idx]
}


pub fn add_subrects(state: &mut GameState, canidate: Rect) {
    let w = i32::abs(canidate.x as i32 - canidate.w as i32);
    let h = i32::abs(canidate.y as i32 - canidate.h as i32);
    let half_w = i32::max(w / 2, 1);
    let half_h = i32::max(h / 2, 1);
    state.rects.push(Rect::new(
        canidate.x,
        canidate.y,
        half_w as f32,
        half_h as f32,
    ));
    state.rects.push(Rect::new(
        canidate.x,
        canidate.y + half_h as f32,
        half_w as f32,
        half_h as f32,
    ));
    state.rects.push(Rect::new(
        canidate.x + half_w as f32,
        canidate.y,
        half_w as f32,
        half_h as f32,
    ));
    state.rects.push(Rect::new(
        canidate.x + half_w as f32,
        canidate.y + half_h as f32,
        half_w as f32,
        half_h as f32,
    ));
}

fn setup(c: &mut EngineContext) {
    let _mobs: Vec<Mob> = Vec::new();
    // Load the grass texture
    c.load_texture_from_bytes(
        "grass",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/grass.png"
        )),
    );
    c.load_texture_from_bytes(
        "wall",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/wall.png"
        )),
    );
    c.load_texture_from_bytes(
        "floor",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/floor.png"
        )),
    );

    // Load the player texture
    c.load_texture_from_bytes(
        "player",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/chicken.png"
        )),
    );
    c.load_texture_from_bytes(
        "mob",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/chicken.png"
        )),
    );


    for x in 0..50 {
        for y in 0..50 {
            let variant = random_i32(0, 2);
            // Tile the world with random variant of grass sprite
            commands().spawn((
                Sprite::new("grass".to_string(), vec2(1.0, 1.0), 0, WHITE)
                    .with_rect(32 * variant, 0, 32, 32),
                Transform::position(vec2(x as f32, y as f32)),
                Grass,
            ));
        }
    }

    commands().spawn((
        Transform::position(vec2(10.0, 25.0)),
        Mob {
            move_mode: Movement::Random,
            position: vec2(10.0, 25.0),
            velocity: Vec2::ZERO,
            move_target: vec2(10.0, 25.0),
            move_timer: 0.0,
        },
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                name: "mob".into(),
                offset: ivec2(0, 0),
                step: ivec2(16, 0),
                size: isplat(16),
                frames: 1,
            })
            .add_animation("walk", 0.05, true, AnimationSource::Atlas {
                name: "mob".into(),
                offset: ivec2(16, 0),
                step: ivec2(16, 0),
                size: isplat(16),
                frames: 6,
            })
            .build(),
    ));


    // Spawn the player entity and make sure z-index is above the grass
    commands().spawn((
        Transform::position(vec2(25.0, 25.0)),
        Player,
        AnimatedSpriteBuilder::new()
            .z_index(10)
            .add_animation("idle", 0.1, true, AnimationSource::Atlas {
                name: "player".into(),
                offset: ivec2(0, 0),
                step: ivec2(16, 0),
                size: isplat(16),
                frames: 1,
            })
            .add_animation("walk", 0.05, true, AnimationSource::Atlas {
                name: "player".into(),
                offset: ivec2(16, 0),
                step: ivec2(16, 0),
                size: isplat(16),
                frames: 6,
            })
            .build(),
    ));
}


fn update(c: &mut EngineContext) {
    clear_background(TEAL);

    let dt = c.delta;
    // for (mob,(_, transform)) in world().query::<(&mut Mob , &mut Transform)>().iter() {
    //     let velocity = vec2(0., 0.1);
    //     if velocity.length() > 0. {
    //         move_timer += dt;
    //     } else {
    //        mob.move_timer = 0.;
    //     }
    //     if mob.move_timer > 1. {
    //         mob.move_timer = 0.;
    //         mob.move_target = vec2(random_i32(0, 50) as f32, random_i32(0, 50) as f32);
    //     }
    //     let mut move_dir = mob.move_target - mob.position;
    //     if move_dir.length() > 0. {
    //         move_dir = move_dir.normalize_or_zero();
    //     }
    //     mob.position += move_dir * 0.1 * dt;
    //     mob.position.x = mob.position.x.clamp(0., 50.);
    //     mob.position.y = mob.position.y.clamp(0., 50.);
    //     mob.position = mob.position.round();
    // }
    for (_, (_, animated_sprite, transform, mode)) in world()
        .query::<(&Mob, &mut AnimatedSprite, &mut Transform, &mut MoveMode)>()
        .iter()
    {
        //TODO: turn system
        let mut moved = false;
        let speed = 3.0;
        let mut move_dir = Vec2::ZERO;
        match &mut mode.mode {
            Movement::Static => {}
            Movement::Random => {
                let mut _move_roll = random_i32(1, 5);
                match _move_roll {
                    1 => {
                        move_dir.x += 1.;
                        moved = true;
                        info!("move right");
                    }
                    2 => {
                        move_dir.x -= 1.;
                        moved = true;
                        info!("move left");
                    }
                    3 => {
                        move_dir.y += 1.;
                        moved = true;
                        info!("move up");
                    }
                    4 => {
                        move_dir.y -= 1.;
                        moved = true;
                        info!("move down");
                    }
                    _ => (),
                }
            }

            // if move_dir.x > 0.
            //     && move_dir.x < 50.
            //     && move_dir.y > 0.
            //     && move_dir.y <50.
            // {
            //     let destination = map.xy_idx(move_dir.x as i32, move_dir.y as i32);

            //     if !state.map.walls[destination] {
            //         let idx = state
            //             .map
            //             .xy_idx(_transform.position.y as i32, _transform.position.y as i32);
            //         state.map.walls[idx] = false;
            //         _transform.position.x = move_dir.x;
            //         _transform.position.y = move_dir.y;
            //         state.map.walls[destination] = true;
            //     }
            // }
            // }
            Movement::RandomWaypoint { path } => {
                if let Some(path) = path {
                    if path.len() > 1 {
                        transform.position.x = path[0] as f32 % 50.;
                        transform.position.y = path[1] as f32 % 50.;
                        path.remove(0);
                    }
                } else {
                    mode.mode = Movement::RandomWaypoint { path: None };
                }
            }
        }
        if moved {
            animated_sprite.flip_x = move_dir.x < 0.0;
            transform.position += move_dir.normalize_or_zero() * speed * dt;
            animated_sprite.play("walk");
        } else {
            animated_sprite.play("idle");
        }
    }


    for (_, (_, animated_sprite, transform)) in
        world().query::<(&Player, &mut AnimatedSprite, &mut Transform)>().iter()
    {
        // Handle movement and animation
        let mut moved = false;
        let speed = 3.0;
        let mut move_dir = Vec2::ZERO;

        if is_key_down(KeyCode::W) {
            move_dir.y += 1.0;
            moved = true;
        }
        if is_key_down(KeyCode::S) {
            move_dir.y -= 1.0;
            moved = true;
        }
        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
            moved = true;
        }
        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
            moved = true;
        }

        if moved {
            animated_sprite.flip_x = move_dir.x < 0.0;
            transform.position += move_dir.normalize_or_zero() * speed * dt;
            animated_sprite.play("walk");
        } else {
            animated_sprite.play("idle");
        }

        main_camera_mut().center = transform.position;
    }
}
