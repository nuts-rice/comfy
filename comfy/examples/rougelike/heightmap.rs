use comfy::*;
//Color map
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::{max, min},
};

use crate::components;
pub const MAPWIDTH: i32 = 80;
pub const MAPHEIGHT: i32 = 43;
pub const MAPCOUNT: usize = (MAPWIDTH * MAPHEIGHT) as usize;
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
#[derive(Copy, Clone, Debug)]
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
            x: (self.top_left.x + self.top_left.x) / 2,
            y: (self.top_left.y + self.bottom_right.y) / 2,
            color: self.top_left.color,
            tiletype: TileType::Floor,
        }
    }
}

pub fn spawn_entities(_map: &Heightmap, _world: &mut World) {
    unimplemented!()
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
    pub start: Position,
    depth: i32,
}
impl Heightmap {
    pub fn new(
        exp: i32,
        spread: f32,
        color_name: &str,
        new_depth: i32,
    ) -> Heightmap {
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
            width: MAPWIDTH,
            height: MAPHEIGHT,
            start: Position {
                x: 0,
                y: 0,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
            spread_rate: spread,
            depth: new_depth,
            exponent: exp,
            colormap_name: color_name.to_string(),
            rooms: Vec::new(),
            rects: Vec::new(),
            tiles: vec![TileType::Wall; MAPCOUNT],
        }
    }
    fn get_start_position(&self) -> Position {
        self.start
    }
   pub fn build_map(&mut self) {
        self.displace();
    }
    pub fn build_bsp_map(&mut self) {
        self.bsp();
    }
    fn spawn_entities(&mut self, world: &mut World) {
        for room in self.rooms.iter().skip(1) {
            components::spawn_room(room, world, self.depth);
        }
    }
    fn bsp(&mut self) {
        self.rects.clear();
        self.rects.push(Rect::new(
            Position { x: 2, y: 2, color: YELLOW, tiletype: TileType::Floor },
            Position {
                x: self.width - 5,
                y: self.height - 5,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
        ));
        let first_room = self.rects[0];
        self.add_subrects(first_room);
        let mut n_rooms = 0;
        while n_rooms < 240 {
            let rect = self.get_random_rect();
            let canidate = self.get_random_sub_rect(rect);
            info!("canidate room has corners of : {:?} and {:?}", canidate.top_left, canidate.bottom_right);
            if self.is_bsp_possible(canidate) {
                apply_room_to_map(&canidate, self);
                self.rooms.push(canidate);
                self.add_subrects(rect);
            }
            n_rooms += 1;
                    
        } 
        self.rooms.sort_by(|a, b| a.top_left.x.cmp(&b.top_left.x));
        for i in 0..self.rooms.len()  - 1 {
            let room = self.rooms[i];
            let next_room = self.rooms[i + 1];
            let start_x = room.top_left.x + (random_i32(1, i32::abs(room.top_left.x - room.bottom_right.x)) - 1);
            let start_y = room.top_left.y + (random_i32(1, i32::abs(room.top_left.y - room.bottom_right.y)) - 1);
            let end_x = next_room.top_left.x + (random_i32(1, i32::abs(next_room.top_left.x - next_room.bottom_right.x)) - 1);
            let end_y = next_room.top_left.y + (random_i32(1, i32::abs(next_room.top_left.y - next_room.bottom_right.y)) - 1);
            self.draw_corridor(start_x, start_y, end_x, end_y);
                    

        }
        // let start = self.rooms[0].center();
        // self.start = Position {
        //     x: start.x,
        //     y: start.y,
        //     color: YELLOW,
        //     tiletype: TileType::Floor,
        // };
            
    }

    fn draw_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        let mut x = x1;
        let mut y = y1;
        while x != x2 || y != y2 {
            if x < x2 {
                x += 1;
            } else if x > x2 {
                x -= 1;
            } else if y < y2 {
                y += 1;
            } else if y > y2 {
                y -= 1;
            }
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
                    
        }
    }

    fn get_random_rect(&self) -> Rect {
        if self.rects.len() == 1 {
            return self.rects[0];
        }
        let idx = random_usize(1, self.rects.len()) - 1   as usize;
        self.rects[idx]
    }

    fn get_random_sub_rect(&self, canidate: Rect) -> Rect {
        let mut result = canidate;
        let rect_width = i32::abs(canidate.top_left.x - canidate.bottom_right.x);
        let rect_height = i32::abs(canidate.top_left.y - canidate.bottom_right.y);
        let w = i32::max(3, random_i32(1, i32::min(rect_width, 10)) - 1) + 1;
        let h = i32::max(3, random_i32(1, i32::min(rect_height, 10)) - 1) + 1;
        result.top_left.x += random_i32(1, 6) - 1;
        result.top_left.y += random_i32(1, 6) - 1;
        result.bottom_right.x = result.top_left.x + w;
        result.bottom_right.y = result.top_left.y + h;
        result
    }
    fn add_subrects(&mut self, rect: Rect) {
        let width = i32::abs(rect.top_left.x - rect.bottom_right.x);
        let height = i32::abs(rect.top_left.y - rect.bottom_right.y);
        let half_width = i32::max(width / 2, 1);
        let half_height = i32::max(height / 2, 1);
        self.rects.push(Rect::new(
            Position {
                x: rect.top_left.x,
                y: rect.top_left.y,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
            Position {
                x: half_width,
                y: half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
        ));
        self.rects.push(Rect::new(
            Position {
                x: rect.top_left.x,
                y: rect.top_left.y + half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
            Position {
                x: half_width,
                y: half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
        ));
        self.rects.push(Rect::new(
            Position {
                x: rect.top_left.x + half_width,
                y: rect.top_left.y,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
            Position {
                x: half_width,
                y: half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
        ));
        self.rects.push(Rect::new(
            Position {
                x: rect.top_left.x + half_width,
                y: rect.top_left.y + half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
            Position {
                x: half_width,
                y: half_height,
                color: YELLOW,
                tiletype: TileType::Floor,
            },
        ));
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
                        .get_color(x - half_res, y + half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let top_right = self
                        .get_color(x + half_res, y + half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let bottom_left = self
                        .get_color(x - half_res, y - half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;
                    let bottom_right = self
                        .get_color(x + half_res, y + half_res)
                        .unwrap_or(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 })
                        .r;

                    let rng_val = random_range(0., 5.0);
                    let square_avg = Self::calculate_square_average(
                        top_left,
                        top_right,
                        bottom_left,
                        bottom_right,
                    );
                    let displace = rng_val * self.spread_rate;
                    let t = square_avg + displace;
                    println!("t: {}", { t });
                    // let w = gen_range(t, MAX_SIZE as f32);
                    // let h = gen_range(t, MAX_SIZE as f32);
                    let canidate_room: Rect = Rect::new(
                        Position {
                            x: x - t as i32,
                            y: y - t as i32,
                            color: Self::terrain_lerp(t, &self.colormap_name),
                            tiletype: TileType::Floor,
                        },
                        Position {
                            x: x + (t) as i32,
                            y: y + (t) as i32,
                            color: Self::terrain_lerp(t, &self.colormap_name),
                            tiletype: TileType::Floor,
                        },
                    );
                    let mut ok = true;
                    for other_rooms in self.rooms.iter() {
                        if canidate_room.intersects(other_rooms) {
                            ok = false;
                        }
                    }
                    if ok {
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
                    println!(
                        "canidate room has corners of : {:?} and {:?}",
                        canidate_room.top_left, canidate_room.bottom_right
                    );

                    self.rooms.push(canidate_room);
                    let color: Color =
                        Self::terrain_lerp(t, &self.colormap_name);
                    self.positions.borrow_mut()[x as usize][y as usize] =
                        Position { x, y, color, tiletype: TileType::Floor };
                }
            }
            resolution /= 2;
        }
        let start = self.rooms[0].center();
        self.start = Position {
            x: start.x,
            y: start.y,
            color: YELLOW,
            tiletype: TileType::Floor,
        };
    }
    fn is_bsp_possible(&self, canidate: Rect)    -> bool {
        let mut expanded = canidate;
        expanded.top_left.x -= (2) as i32;
        expanded.bottom_right.x += (2) as i32;
        expanded.top_left.y -= (2) as i32;
        expanded.bottom_right.y += (2) as i32;
        let mut can_build = true;
        for y in expanded.top_left.y ..= expanded.bottom_right.y {
            for x in expanded.top_left.x ..= expanded.bottom_right.x {
                if x > self.width - 2 {
                    can_build = false;
                }
                if y > self.height - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    let idx = self.xy_idx(x, y);
                    if self.tiles[idx] != TileType::Wall {
                can_build = false;
                    }
                }
            }
        }
        can_build
    }

    pub fn is_possible(&self, canidate: Rect, t: f32) -> bool {
        let mut expanded = canidate;
        expanded.top_left.x -= (t) as i32;
        expanded.bottom_right.x += (t) as i32;
        expanded.top_left.y -= (t) as i32;
        expanded.bottom_right.y += (t) as i32;
        let mut can_build = true;
        for y in expanded.top_left.y..=expanded.bottom_right.y {
            for x in expanded.top_left.x..=expanded.bottom_right.x {
                if x > self.width - 2 {
                    can_build = false;
                }
                if y > self.height - 2 {
                    can_build = false;
                }
                if x < 1 {
                    can_build = false;
                }
                if y < 1 {
                    can_build = false;
                }
                if can_build {
                    let idx = self.xy_idx(x, y);
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
        let mut y = 0;
        let mut x = 0;
        for (idx, tile) in self.tiles.iter().enumerate() {
            match tile {
                TileType::Floor => {
                    commands().spawn((
                        Sprite::new(
                            "floor".to_string(),
                            vec2(1., 1.),
                            0,
                            WHITE,
                        )
                        .with_rect(x, y, 12, 12),
                        Transform::position(vec2(x as f32, y as f32)),
                    ));
                }
                TileType::Wall => {
                    commands().spawn((
                        Sprite::new(
                            "wall".to_string(),
                            vec2(1., 1.),
                            0,
                            WHITE,
                        )
                        .with_rect(x, y, 12, 12),
                        Transform::position(vec2(x as f32, y as f32)),
                        ));
                }                     
            }            
        }
        x += 1;
        if x > MAPWIDTH as i32 - 1 { 
            x = 0;
            y += 1;
    }

        // for room in self.rooms.iter() {
        //     for x in room.top_left.x..=room.bottom_right.x {
        //         for y in room.top_left.y..=room.bottom_right.y {
        //             commands().spawn((
        //                 Sprite::new(
        //                     "floor".to_string(),
        //                     vec2(1.0, 1.0),
        //                     0,
        //                     WHITE,
        //                 )
        //                 .with_rect(x, y, 12, 12),
        //                 Transform::position(vec2(x as f32, y as f32)),
        //             ));
        //         }
        //     }
        // }
    }

    pub fn draw_dungeon(&self) {}

    pub fn draw_rooms_test(&self) {
        let hw = 5.0;
        let hh = 0.5;
        for room in self.rooms.iter() {
            let position_2: Vec2 =
                vec2(room.bottom_right.x as f32, room.top_left.y as f32);
            let position_3: Vec3 = position_2.extend(0.0);
            {
                draw_mesh(Mesh {
                    vertices: [
                        SpriteVertex::new(
                            position_3 + vec3(-hw, hh, 0.0),
                            Vec2::ZERO,
                            room.top_left.color,
                        ),
                        SpriteVertex::new(
                            position_3 + vec3(-hw, -hh, 0.0),
                            Vec2::ZERO,
                            room.top_left.color,
                        ),
                        SpriteVertex::new(
                            position_3 + vec3(hw, hh, 0.0),
                            Vec2::ZERO,
                            room.bottom_right.color,
                        ),
                        SpriteVertex::new(
                            position_3 + vec3(hw, -hh, 0.0),
                            Vec2::ZERO,
                            room.bottom_right.color,
                        ),
                    ]
                    .into(),
                    indices: vec![0, 2, 3, 0, 3, 1].into(),
                    texture: None,
                    z_index: 0,
                });
            }
            let room_size = splat(2.0);
            let off = 0.2;

            draw_rect(
                position_2 - vec2(hw + hh + off, 0.0),
                room_size,
                room.top_left.color,
                0,
            );
            draw_rect(
                position_2 + vec2(hw + hh + off, 0.0),
                room_size,
                room.bottom_right.color,
                0,
            );
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
        (y as usize * self.width as usize) + x as usize
    }
}
