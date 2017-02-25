use Pixel;
use world::World;
use piston_window::*;
use car::CarRules;
use camera::Camera;
use cgmath::{Vector3, Vector2};
use color::*;

pub struct Game {
    config: GameConfig,
    world: World,
    window: PistonWindow,
    bot_rules: CarRules,
    camera: Camera,
    state: State,
}

struct State {
    pub turn: Turn,
    pub sprint: bool,
}
pub enum Turn { Left, Right, None, }

pub struct GameConfig {
    pub title: &'static str,
    pub screen_size: Pixel,
    pub ups: u64,
    pub max_fps: u64,
    pub tunel_size: [f64; 3],
    pub player_size: [f64; 3],
    pub player_speed: f64,
    pub player_turn_speed: f64,
    pub bot_size: [(f64, f64); 3],
    pub bot_speed: (f64, f64),
    pub bot_turn_speed: (f64, f64),
    pub divider_size: [f64; 2],
    pub camera_height: f64,
    pub camera_distance: f64,
    pub decor_distance: f64,
    pub sprint_factor: f64,
}

impl Game {
    pub fn new(config: GameConfig) -> Game {
        let mut window: PistonWindow = WindowSettings::new(
            config.title, [config.screen_size.w, config.screen_size.h])
            .exit_on_esc(true).build().unwrap();
        window.set_ups(config.ups);
        window.set_max_fps(config.max_fps);
        let bot_rules = CarRules {
            size: config.bot_size,
            position: [(0., config.tunel_size[0]), (0., 0.), (0., config.tunel_size[2])],
            speed: config.bot_speed,
            turn_speed: config.bot_turn_speed,
            color: Vec::new(),
        };
        let world = World::new(&config);
        let camera = Camera::new(config.screen_size.clone(),
                                 Vector3::new(world.player.position.x,
                                              config.camera_height,
                                              world.player.position.z-config.camera_distance));
        let state = State {
            turn: Turn::None,
            sprint: false,
        };
        Game {
            config: config,
            world: world,
            window: window,
            bot_rules: bot_rules,
            camera: camera,
            state: state,
        }
    }

    pub fn run(&mut self) {
        while let Some(e) = self.window.next() {
            match e {
                Input::Press(Button::Keyboard(key)) => self.key_press(key),
                Input::Release(Button::Keyboard(key)) => self.key_release(key),
                Input::Render(_) => self.draw(&e),
                Input::Update(args) => self.update(args.dt),
                _ => {}
            }
        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::A => self.state.turn = Turn::Left,
            Key::D => self.state.turn = Turn::Right,
            Key::W => if !self.state.sprint {
                self.state.sprint = true;
                self.world.player.speed *= self.config.sprint_factor;
            },
            _ => (),
        }
    }
    fn key_release(&mut self, key: Key) {
        match key {
            Key::A => if let Turn::Left = self.state.turn {
                self.state.turn = Turn::None;
            },
            Key::D => if let Turn::Right = self.state.turn {
                self.state.turn = Turn::None;
            },
            Key::W => if self.state.sprint {
                self.state.sprint = false;
                self.world.player.speed /= self.config.sprint_factor;
            },
            _ => (),
        }
    }
    fn draw(&mut self, e: &Input) {
        let lines = self.world.render(&self.camera);
        self.window.draw_2d(e, |c, g| {
            clear(BLACK, g);
            for (l, color) in lines {
                line(color, 1., convert(l), c.transform, g);
            }
        });
    }
    fn update(&mut self, dt: f64) {
        self.world.player.turn(&self.state.turn, dt);
        self.world.update(dt);
        self.world.validate();
        self.camera.eye.x = self.world.player.position.x;
    }
}

fn convert(x: [Vector2<f64>; 2]) -> [f64; 4] {
    [x[0].x, x[0].y, x[1].x, x[1].y]
}
