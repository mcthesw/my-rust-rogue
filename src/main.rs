mod map;
mod rect;
pub use rect::*;
pub use map::*;

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
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        // 遍历所有玩家组件
        let new_x = (pos.x + delta_x).clamp(0, 79);
        let new_y = (pos.y + delta_y).clamp(0, 49);
        let dst_index = xy2index(new_x, new_y);
        if map[dst_index] != TileType::Wall {
            pos.x = new_x;
            pos.y = new_y;
        }
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
    let a = 0;
    let b = 0;
    if a >= b+1 {}
    let a = 0;
    let b = 0;
    if a >= b+1 {}
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("神必Rouge").build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    let (rooms,map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);

    let (player_x,player_y) = rooms.first().unwrap().center();
    gs.ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'), // Ascii table for IBM PC charset (CP437)
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    // for i in 0..11 {
    //     gs.ecs
    //         .create_entity()
    //         .with(Position { x: i * 7, y: 20 })
    //         .with(Renderable {
    //             glyph: rltk::to_cp437('A'),
    //             fg: RGB::named(rltk::RED),
    //             bg: RGB::named(rltk::BLACK),
    //         })
    //         .with(LeftMover {})
    //         .build();
    // }

    rltk::main_loop(context, gs)
}
