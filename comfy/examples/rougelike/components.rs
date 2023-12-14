use crate::heightmap::Rect;

use super::*;

pub struct Player {
    pub pos: Vec2,
    pub move_timer: f32,
    pub moved: bool,
    pub is_dead: bool,
    pub inventory: Vec<Item>,
    pub player_type: PlayerType,
    pub name: String,
}

pub struct Mob {}
pub struct Item {}

enum Grammar {
    Noun,
    Verb,
    Adjective,
    Adverb,
}

enum PlayerType {
    Warrior,
    Mage,
    Rogue,
    Cleric,
}
pub const WARRIOR_ADJECTIVES: [&str; 4] =
    ["Brave", "Strong", "Mighty", "Powerful"];
pub const MAGE_ADJECTIVES: [&str; 4] =
    ["Arcane", "Mystical", "Magical", "Enchanted"];
pub const ROGUE_ADJECTIVES: [&str; 4] =
    ["Sneaky", "Stealthy", "Cunning", "Devious"];
pub const CLERIC_ADJECTIVES: [&str; 4] =
    ["Holy", "Divine", "Blessed", "Sacred"];
enum ItemType {
    Book,
    Weapon,
    Armor,
    Consumable,
    Misc,
}

pub fn spawn_room(_room: &Rect, _world: &mut World, _map_depth: i32) {
    unimplemented!()
}

fn room_table(_map_depth: i32) {
    unimplemented!()
}

pub fn spawn_entities(_map: &Heightmap, _world: &mut World) {
    unimplemented!()
}
impl Player {
    pub fn new(pos: Vec2) -> Player {
        Player {
            inventory: Vec::new(),
            player_type: PlayerType::Warrior,
            name: String::from("Krugg the fearsome"),
            pos,
            move_timer: 0.0,
            moved: false,
            is_dead: false,
        }
    }
}
//pass down start pos to this from map...
pub fn spawn_player(
    _world: &mut World,
    _player_x: i32,
    _player_y: i32,
) -> Entity {
    // draw_comfy(map.start, tint, z_index, world_size)
    // let mut builder = EntityBuilder::new();

    // let player = Player::new(vec2(_player_x as f32, _player_y as f32)) ;
    unimplemented!()
}


pub fn gen_text(_world: &mut World, _x: i32, _y: i32, _text: String) -> Entity {
    unimplemented!()
}

// pub fn gen_book(_world: &mut World, _x: i32, _y: i32, player: Player) -> Entity {
//     let mut builder = EntityBuilder::new();
//     let player_type = player.player_type;
//     let mut title = String::new();

//     match player_type {
//         PlayerType::Warrior => {
//             let adjective_list = WARRIOR_ADJECTIVES;
//             let adjective: = adjective_list[random_i32(0, 4)];
//         }
//         PlayerType::Mage => {
//             let adjective_list = MAGE_ADJECTIVES;
//             let adjective = adjective_list[random_i32(0, 4)];
//         }
//         PlayerType::Rogue => {
//             let adjective_list = ROGUE_ADJECTIVES;
//             let adjective = adjective_list[random_i32(0, 4)];
//         }
//         PlayerType::Cleric => {
//             let adjective_list = CLERIC_ADJECTIVES;
//             let adjective = adjective_list[random_i32(0, 4)];
//         }
//         let mut extract: [String; 4] = [
//             format!("{} {} {}.", adjective, extract, title),
//             format!("{} {} {}.", adjective, extract, title),
//             format!("{} {} {}.", adjective, extract, title),
//             format!("{} {} {}.", adjective, extract, title),
//         ];
//     let book : Entit
//     }


// }


#[cfg(test)]
mod test {

    #[cfg(test)]
    fn book_gen_test() {
        unimplemented!()
    }
}
