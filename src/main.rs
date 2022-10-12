use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct LeftMover {}

struct LeftWorker {}
impl<'a> System<'a> for LeftWorker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        // 遍历所有玩家组件
        pos.x = (pos.x + delta_x).clamp(0, 79);
        pos.y = (pos.y + delta_y).clamp(0, 49);
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            rltk::VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            rltk::VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
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

fn new_map() -> Vec<TileType> {
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
    for _ in 0..400 {
        // 400是墙壁的数量
        let x = rng.roll_dice(1, 79); // 一个79面骰子
        let y = rng.roll_dice(1, 49);

        let index = xy2index(x, y);
        map[index] = TileType::Wall;
    }
    map[xy2index(40, 25)] = TileType::Floor; // 出生点必须是空的

    map
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    for index in 0..map.len() {
        let tile = map[index];
        let (x, y) = index2xy(index);
        
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('#'),
                );
            }
        }
    }
}
struct State {
    ecs: World,
}

impl State {
    fn run_system(&mut self) {
        let mut lw = LeftWorker {};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_system();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("神必Rouge").build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.insert(new_map());

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    for i in 0..11 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('A'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    rltk::main_loop(context, gs)
}
