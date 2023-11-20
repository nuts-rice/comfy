use comfy::*;
//Color map

simple_game!("Proc gen demo", setup, update);


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
#[derive(Clone, Copy, Debug)]
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
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * (self.bottom_right.x - self.top_left.x) + x) as usize
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
pub fn apply_room_to_map(room: &Room, map: &mut Heightmap) {
    for x in room.top_left.x..room.bottom_right.x {
        for y in room.top_left.y..room.bottom_right.y {
            map.positions.borrow_mut()[x as usize][y as usize].tiletype =
                TileType::Floor;
        }
    }
}
pub struct Heightmap {
    size: usize,
    positions: AtomicRefCell<Vec<Vec<Position>>>,
    exponent: i32,
    spread_rate: f32,
    colormap_name: String,
    rooms: Vec<Room>,
    rects: Vec<Rect>,
}
impl Heightmap {
    pub fn new(exp: i32, spread: f32, color_name: &str) -> Heightmap {
        let _size = 2_i32.pow(exp.try_into().unwrap()) as usize;
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
            spread_rate: spread,
            exponent: exp,
            colormap_name: color_name.to_string(),
            rooms: Vec::new(),
            rects: Vec::new(),                    
        }
    }
    //Midpoint displacemnt used here
    pub fn displace(&mut self) {
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
                    let t = square_avg + displace;
                    // let canidate_room
                    let color: Color =
                        Self::terrain_lerp(t, &self.colormap_name);
                    self.positions.borrow_mut()[x as usize][y as usize] =
                        Position { x, y, color, tiletype: TileType::Floor };
                }
            }
            resolution /= 2;
        }
    }
    //Wave function collapse:
    //see https://github.com/mxgmn/WaveFunctionCollapse
    pub fn wfc(&mut self) {
        unimplemented!()
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
}

pub fn load_assets(c: &mut EngineContext) {
    c.load_texture_from_bytes(
        "floor",
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/floor.png"
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
    let _viewport = main_camera().world_viewport() / 2.0;
    egui::Window::new("Map size")
        .anchor(egui::Align2::RIGHT_TOP, egui::vec2(10.0, 10.0))
        .show(egui(), |ui| {
            let mut map_size = MAP_SIZE.borrow_mut();
            if ui.radio_value(&mut *map_size, Big, "Big").clicked() {
                let mut big_map = Heightmap::new(7, 0.44, "comfy");
                big_map.displace();
                big_map.draw_heightmap();
            }
            if ui.radio_value(&mut *map_size, Small, "Small").clicked(){
                let mut small_map = Heightmap::new(3, 0.44, "comfy");
                small_map.displace();
                small_map.draw_heightmap();
            };
        });
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
            ui.radio_value(&mut *tileset, Dungeon, "Dungeon");
        });

    // MapSize::Small => {
    //     let mut small_map = Heightmap::new(3, 0.44);
    //     small_map.displace();
    //     small_map.draw_heightmap();
    // }

    // MapSize::Big => {
    //             let mut big_map = Heightmap::new(7, 0.44);
    //     big_map.displace();
    //     big_map.draw_heightmap();

    // }
}


// map.displace();
// map.draw_heightmap();


// let size = match *MAP_SIZE.borrow() {
//     MapSize =>
// }
