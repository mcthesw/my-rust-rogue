use rltk::RandomNumberGenerator;

use crate::Rect;
use std::cmp::{max, min};

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy2index(x: i32, y: i32) -> usize {
    // ?为什么要使用usize
    (y as usize * 80) + x as usize
}
pub fn index2xy(i: usize) -> (i32, i32) {
    let x = (i % 80) as i32;
    let y = (i / 80) as i32;
    (x, y)
}

pub fn new_map_test() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];
    for x in 0..80 {
        // 上下墙壁
        map[xy2index(x, 0)] = TileType::Wall;
        map[xy2index(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        // 左右墙壁
        map[xy2index(0, y)] = TileType::Wall;
        map[xy2index(79, y)] = TileType::Wall;
    }

    let mut rng = rltk::RandomNumberGenerator::new();
    for _i in 0..400 {
        // 400是墙壁的数量
        let x = rng.roll_dice(1, 79); // 一个79面骰子
        let y = rng.roll_dice(1, 49);

        let index = xy2index(x, y);
        map[index] = TileType::Wall;
    }
    map[xy2index(40, 25)] = TileType::Floor; // 出生点必须是空的

    map
}

pub fn new_map_rooms_and_corridors() -> (Vec<Rect>, Vec<TileType>) {
    let mut map = vec![TileType::Wall; 80 * 50];

    let mut rooms = Vec::new();
    const MAX_ROOMS: i32 = 30;
    const MIN_SIZE: i32 = 6;
    const MAX_SIZE: i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    'collapse_detect: for _i in 0..MAX_ROOMS {
        let w = rng.range(MIN_SIZE, MAX_SIZE);
        let h = rng.range(MIN_SIZE, MAX_SIZE);
        let x = rng.roll_dice(1, 80 - w - 1) - 1;
        let y = rng.roll_dice(1, 50 - h - 1) - 1;
        let new_room = Rect::new(x, y, w, h);

        for other in rooms.iter() {
            // 如果房间冲突，就放弃当前房间
            if new_room.intersect(other) {
                continue 'collapse_detect;
            }
        }

        apply_room_to_map(&new_room, &mut map);
        if !rooms.is_empty() {
            let (new_x, new_y) = new_room.center();
            let (prev_x, prev_y) = rooms.last().unwrap().center();
            if rng.range(0, 1 + 1) == 1 {
                apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
            } else {
                apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
            }
        }

        rooms.push(new_room);
    }

    (rooms, map)
}

fn apply_room_to_map(room: &Rect, map: &mut [TileType]) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map[xy2index(x, y)] = TileType::Floor;
        }
    }
}

fn apply_horizontal_tunnel(map: &mut [TileType], x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let index = xy2index(x, y);
        if index > 0 && index < 80 * 50 {
            map[index as usize] = TileType::Floor;
        } else {
            panic!("错误的走廊坐标");
        }
    }
}

fn apply_vertical_tunnel(map: &mut [TileType], y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let index = xy2index(x, y);
        if index > 0 && index < 80 * 50 {
            map[index as usize] = TileType::Floor;
        } else {
            panic!("错误的走廊坐标");
        }
    }
}
