pub mod heightmap;
pub use heightmap::*;
pub mod components;
use comfy::*;


pub use components::*;
//TODO: use gamestate
pub struct GameState {
    pub map: Heightmap,
    pub player: Player,
    pub mobs: Vec<Mob>,
    pub start_time: f32,
    pub map_gen_history: Vec<Heightmap>,
}
simple_game!("Proc gen demo", setup, update);

pub fn load_assets(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        "floor",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/floor.png"
        )),
    );
    c.load_texture_from_bytes(
        "wall",
        include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/wall.png")),
    );
    c.load_texture_from_bytes(
        // Every texture gets a string name later used to reference it.
        "comfy",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../assets/comfy.png"
        )),
    );
}
#[derive(PartialEq)]
enum MapSize {
    Small,
    Big,
}
#[derive(PartialEq)]
enum GenerativeMethod {
    Midpoint,
    WFC,
}
#[derive(PartialEq)]
enum Tileset {
    Colors,
    Dungeon,
}
static MAP_SIZE: AtomicRefCell<MapSize> = AtomicRefCell::new(MapSize::Small);
static GEN_METHOD: AtomicRefCell<GenerativeMethod> =
    AtomicRefCell::new(GenerativeMethod::Midpoint);
static TILESET: AtomicRefCell<Tileset> = AtomicRefCell::new(Tileset::Colors);
fn setup(_c: &mut EngineContext) {
    let mut map = Heightmap::new(3, 0.44, "comfy", 1);
    // let now = std::time::Instant::now();
    map.build_bsp_map();
    // let elapsed = now.elapsed();
    // info!("took {:?} to do midpoint displacement", elapsed.as_millis());
    map.draw_rooms_test();
    //TODO: use spawn_player later
    load_assets(_c);
    //need to pass down player
    commands().spawn((
        Sprite::new("comfy".to_string(), vec2(1.0, 1.0), 100, WHITE),
        Transform::position(vec2(map.start.x as f32, map.start.y as f32)),
    ));
}
fn update(_c: &mut EngineContext) {
    use MapSize::*;
    use Tileset::*;
    clear_background(GRAY);
    let dt = _c.delta;
    let _viewport = main_camera().world_viewport();
    main_camera_mut().center = Vec2::from([2.0, 2.0]);
    main_camera_mut().zoom = 25.0;
    let _displaced = false;
    const TIME: f32 = 3.0;
    let mut displaced = false;
    let _map = Heightmap::new(3, 0.44, "comfy", 1);
    egui::Window::new("Map size")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(10.0, 10.0))
        .show(egui(), |ui| {
            use MapSize::*;
            let mut map_size = MAP_SIZE.borrow_mut();
            if ui.radio_value(&mut *map_size, Big, "Big").clicked() {
                let mut map = Heightmap::new(7, 0.44, "comfy", 1);
                map.build_bsp_map();
                displaced = true;
                map.draw_rooms_test();
            }
            if ui.radio_value(&mut *map_size, Small, "Small").clicked() {
                let mut map = Heightmap::new(3, 0.44, "comfy", 1);
                map.build_bsp_map();
                displaced = true;
                map.draw_rooms_test();
            }
        });
    let _map_size = match *MAP_SIZE.borrow() {
        Small => 3,
        Big => 7,
    };
    // let mut map = Heightmap::new(map_size, 0.44, "comfy", 1);
    // map.build_bsp_map();
    // map.draw_rooms_test();
    // let mut map = Heightmap::new(map_size, 0.44, "comfy");
    // map.displace();
    // displaced = true;
    // if displaced {
    //     map.draw_rooms_test();
    // }

    // map.draw_heightmap()
    // map.draw_rooms_test();


    // egui::Window::new("Generative method")
    //     .anchor(egui::Align2::RIGHT_CENTER, egui::vec2(10.0, 10.0))
    //     .show(egui(), |ui| {
    //         let mut gen_method = GEN_METHOD.borrow_mut();
    //         ui.radio_value(&mut *gen_method, Midpoint, "Midpoint displacement");
    //         ui.radio_value(&mut *gen_method, WFC, "Wave Function Collapse");
    //     });

    egui::Window::new("Tileset")
        .anchor(egui::Align2::RIGHT_CENTER, egui::vec2(10.0, 10.0))
        .show(egui(), |ui| {
            let mut tileset = TILESET.borrow_mut();
            ui.radio_value(&mut *tileset, Colors, "Colors");
            if ui.radio_value(&mut *tileset, Dungeon, "Dungeon").clicked() {
                let mut dungeon_map = Heightmap::new(3, 0.44, "dungeon", 1);
                dungeon_map.displace();
                dungeon_map.draw_rooms(_c);
            };
        });
    //TODO: need to pass down player to comfy
    for (_, (_, _, transform)) in
        world().query::<(&Player, &Sprite, &mut Transform)>().iter()
    {
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
            // animated_sprite.flip_x = move_dir.x < 0.0;
            transform.position += move_dir.normalize_or_zero() * speed * dt;
        }
        main_camera_mut().center = transform.position;

        //todo: query for mobs and entities and check for collisions
    }
}
