use comfy::*;
//Color map
simple_game!("Proc gen demo", setup, update);
use std::cmp::{max, min};

pub fn color_data(colormap_name: &str) -> [Color; 7] {
    
    match colormap_name {
        "default" => {
            [
                RED,
                ORANGE_RED,
                ORANGE,
                YELLOW,
                COMFY_GREEN,
                COMFY_BLUE,
                COMFY_DARK_BLUE,
            ]
        }
        "comfy" => [RED, ORANGE_RED, ORANGE, YELLOW, GREEN, BLUE, DARKBLUE],
        _ => [DARKRED, ORANGE_RED, ORANGE, YELLOW, DARKBROWN, LIME_GREEN, BLUE],
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
enum TileType {
    Floor,
    Wall,
}

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    //Gonna be a repersentation of elevation
    pub color: Color,
    pub tiletype: TileType,
}
#[derive(Copy, Clone,Debug)]
pub struct Rect {
    top_left: Position,
    bottom_right: Position,
}
pub struct Room {
    top_left: Position,
    bottom_right: Position,
}
impl Rect {
    pub fn new(top_left: Position, bottom_right: Position) -> Rect {
        Rect { top_left, bottom_right }
    }
    pub fn intersects(&self, other: &Rect) -> bool {
        self.top_left.x <= other.bottom_right.x &&
            self.bottom_right.x >= other.top_left.x &&
            self.top_left.y <= other.bottom_right.y &&
            self.bottom_right.y >= other.top_left.y
    }
    pub fn center(&self) -> Position {
        Position {
            x: (self.bottom_right.x - self.top_left.x) / 2,
            y: (self.bottom_right.y - self.top_left.y) / 2,
            color: self.top_left.color,
            tiletype: TileType::Floor,
        }
    }
}
pub fn apply_room_to_map(room: &Rect, map: &mut Heightmap) {
    for x in room.top_left.x + 1..=room.bottom_right.x {
        for y in room.top_left.y + 1..=room.bottom_right.y {
            let idx = map.xy_idx(x, y);
            map.tiles[idx] = TileType::Floor;
        }
    }
}
pub struct Heightmap {
    size: usize,
    width: i32,
    height: i32,
    positions: AtomicRefCell<Vec<Vec<Position>>>,
    exponent: i32,
    spread_rate: f32,
    colormap_name: String,
    rooms: Vec<Rect>,
    rects: Vec<Rect>,
    tiles: Vec<TileType>,
}
impl Heightmap {
    pub fn new(exp: i32, spread: f32, color_name: &str) -> Heightmap {
        let _size = 2_i32.pow(exp.try_into().unwrap()) as usize;
        let width = _size as i32 / 2;
        let height = _size as i32 / 2;
        let default_color = Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
        let default_position = Position {
            x: 0,
            y: 0,
            color: default_color,
            tiletype: TileType::Floor,
        };
        let positions =
            AtomicRefCell::new(vec![vec![default_position; _size]; _size]);

        Heightmap {
            positions,
            size: _size,
            width,
            height,
            spread_rate: spread,
            exponent: exp,
            colormap_name: color_name.to_string(),
            rooms: Vec::new(),
            rects: Vec::new(),                    
            tiles: vec![TileType::Wall; _size * _size]
                    
        }
    }
    //Midpoint displacemnt used here
    pub fn displace(&mut self) {
        const MIN_SIZE: i32 = 3;
        const MAX_SIZE: i32 = 9;
        let mut resolution = (2_f32.powf(self.size as f32) - 1.) as i32;
        while resolution >= 1 {
            let half_res = resolution / 2;
            for x in (half_res..self.size as i32)
                .step_by(resolution.try_into().unwrap())
            {
                for y in (half_res..self.size as i32)
                    .step_by(resolution.try_into().unwrap())
                {
                    //TODO: set tiletype
                    let top_left = self
                        .get_color(x - half_res, y - half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let top_right = self
                        .get_color(x + half_res, y - half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let bottom_left = self
                        .get_color(x - half_res, y + half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let bottom_right = self
                        .get_color(x + half_res, y + half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;

                    let rng_val = random_range(0., 1.0);
                    let square_avg = Self::calculate_square_average(
                        top_left,
                        top_right,
                        bottom_left,
                        bottom_right,
                    );
                    let displace = rng_val * self.spread_rate;
                    let t = (square_avg  + displace)  ;
                    println!("t: {}", t as f32);
                    // let w = gen_range(t, MAX_SIZE as f32);
                    // let h = gen_range(t, MAX_SIZE as f32);
                        let canidate_room: Rect = Rect::new(
                        Position {
                            x: x - t as i32,
                            y: y - t as i32,
                            color: Self::terrain_lerp(t , &self.colormap_name),
                            tiletype: TileType::Floor,
                        },
                        Position {
                            x: x + (t )  as i32,
                            y: y + (t ) as i32,
                            color: Self::terrain_lerp(t , &self.colormap_name),
                            tiletype: TileType::Floor,
                        },
                    );
                    let mut ok = true;
                    for other_rooms in self.rooms.iter() {
                        if canidate_room.intersects(other_rooms) {
                            ok = false;
                        }
                    }
                    if ok  {
                    // if self.is_possible(canidate_room, t) {
                        apply_room_to_map(&canidate_room, self);
                        if !self.rooms.is_empty() {
                            let new_position = canidate_room.center();
                            let prev_position =
                                self.rooms[self.rooms.len() - 1].center();
                            if random_range(0.0, 1.0) > 0.5 {
                                self.apply_horizontal_tunnel(
                                    prev_position.x,
                                    new_position.x,
                                    prev_position.y,
                                );
                                self.apply_vertical_tunnel(
                                    prev_position.y,
                                    new_position.y,
                                    new_position.x,
                                );
                            } else {
                                self.apply_vertical_tunnel(
                                    prev_position.y,
                                    new_position.y,
                                    prev_position.x,
                                );
                                self.apply_horizontal_tunnel(
                                    prev_position.x,
                                    new_position.x,
                                    new_position.y,
                                );
                            }
                                                    
                        }
                    }
                    println!("canidate room: {:?}", canidate_room);
                    self.rooms.push(canidate_room);
                    let color: Color =
                        Self::terrain_lerp(t , &self.colormap_name);
                    self.positions.borrow_mut()[x as usize][y as usize] =
                        Position { x, y, color, tiletype: TileType::Floor };
                }
            }
            resolution /= 2;
        }
    }
    pub fn is_possible(&self ,canidate: Rect, t: f32 )   -> bool {
        let mut expanded = canidate;
        expanded.top_left.x -= (t ) as i32;
        expanded.bottom_right.x += (t ) as i32;
        expanded.top_left.y -= (t ) as i32;
        expanded.bottom_right.y += (t ) as i32;
        let mut can_build = true;
        for y in expanded.top_left.y..=expanded.bottom_right.y {
            for x in expanded.top_left.x..=expanded.bottom_right.x {
                if x > self.width - 2 {can_build = false; }
                if y > self.height - 2 {can_build = false; }
                if x < 1 {can_build = false; }
                if y < 1 {can_build = false; }
                if can_build {
                    let idx = self.xy_idx(x, y) ;
                    if self.tiles[idx] != TileType::Wall {
                        can_build = false;
                    }
                }

            }
        }
        can_build

    } 
    //Wave function collapse:
    //see https://github.com/mxgmn/WaveFunctionCollapse
    pub fn wfc(&mut self) {
        unimplemented!()
    }
    pub fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    pub fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.tiles.len() {
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    fn get_tiletype(&self, x: i32, y: i32) -> Option<TileType> {
        if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
            Some(self.positions.borrow_mut()[x as usize][y as usize].tiletype)
        } else {
            None
        }
    }
    fn get_color(&self, x: i32, y: i32) -> Option<Color> {
        if x >= 0 && x < self.size as i32 && y >= 0 && y < self.size as i32 {
            Some(self.positions.borrow_mut()[x as usize][y as usize].color)
        } else {
            None
        }
    }

    fn calculate_square_average(
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    ) -> f32 {
        (top_left + top_right + bottom_left + bottom_right) / 4.0
    }

    fn terrain_lerp(t: f32, colormap_name: &str) -> Color {
        let colors = color_data(colormap_name);

        
        match t {
            t if (0.95..=1.25).contains(&t) => colors[0],
            t if (0.55..=0.94).contains(&t) => colors[1],
            t if (0.35..=0.54).contains(&t) => colors[2],
            t if (0.25..=0.34).contains(&t) => colors[3],
            t if (0.15..=0.24).contains(&t) => colors[4],
            t if (0.01..=0.14).contains(&t) => colors[5],
            _ => colors[6],
        }
    }

    pub fn draw_heights(&self) {
        unimplemented!()
    }
    pub fn draw_rooms(&self, c: &mut EngineContext) {
            c.load_texture_from_bytes(
        "floor",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/floor.png"
        )),
    );
    c.load_texture_from_bytes(
        "wall",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/wall.png"
        )),
    );

        for room in self.rooms.iter() {
            for x in room.top_left.x..=room.bottom_right.x {
                for y in room.top_left.y..=room.bottom_right.y {
                    commands().spawn((Sprite::new("floor".to_string(), vec2(1.0, 1.0), 0, WHITE).with_rect(x, y, 12, 12), 
                                      Transform::position(vec2(x as f32, y as f32,)), ));
                }
            }
                    
        }
    }

    pub fn draw_heightmap(&self) {
        for row in self.positions.borrow().iter() {
            for position in row {
                draw_rect(
                    vec2(position.x as f32, position.y as f32),
                    splat(2.0),
                    position.color,
                    0,
                );
            }
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize ) + x as usize
    }
}

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
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/wall.png"
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
    let mut map = Heightmap::new(3, 0.44, "comfy");
    // let now = std::time::Instant::now();
    map.displace();
    // let elapsed = now.elapsed();
    // info!("took {:?} to do midpoint displacement", elapsed.as_millis());
    map.draw_heightmap();
}
fn update(_c: &mut EngineContext) {
    use GenerativeMethod::*;
    use MapSize::*;
    use Tileset::*;
    clear_background(GRAY.alpha(0.1));
    let _viewport = main_camera().world_viewport() ;
    egui::Window::new("Map size")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(10.0, 10.0))
        .show(egui(), |ui| {
            let mut map_size = MAP_SIZE.borrow_mut();
            ui.radio_value(&mut *map_size, Big, "Big");
            ui.radio_value(&mut *map_size, Small, "Small");
        });
    let map_size = match *MAP_SIZE.borrow(){
        Small => 3,
        Big => 7,            
    };
    let mut map = Heightmap::new(map_size, 0.44, "comfy");
    
    map.displace();
    map.draw_heightmap();
    
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
            let mut  tileset = TILESET.borrow_mut();
            ui.radio_value(&mut *tileset, Colors, "Colors");
            if ui.radio_value(&mut *tileset, Dungeon, "Dungeon").clicked() {
                let mut dungeon_map = Heightmap::new(3, 0.44, "dungeon");
                dungeon_map.displace();
                dungeon_map.draw_rooms(_c);
            };
        });

}

