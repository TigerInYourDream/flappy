use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURTION: f32 = 75.0;

pub enum GameMode {
    Menu,
    Playing,
    End,
}

pub struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURTION {
            self.frame_time = 0.0;

            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) {
            self.mode = GameMode::End;
        }


    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to play flarrt dargon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(10, "(Q) Quite Game");

        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You earn {} points", &self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(10, "(Q) Quite Game");

        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0); 
        self.score = 0;
    }

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

pub struct Player {
    x: i32,
    y: i32,
    volocity: f32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            volocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            // codepage 437 character for `@`
            to_cp437('@')
        )
    }

    fn gravity_and_move(&mut self) {
        if self.volocity < 2.0 {
            self.volocity += 0.2;
        }

        self.y += self.volocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0
        }
    }

    fn flap(&mut self) {
        // under 0 is up to top because start from left top is 0 0
        self.volocity = -2.0;
    }


    pub fn update(&mut self) {
        self.y += self.volocity as i32;
        self.volocity += 0.2;
    }
    
}

pub struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;
        let half_x = self.size / 2;
        for y in 0..self.gap_y - half_x {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|')
            );
        }

        for y in self.gap_y + half_x..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|')
            );
        }
    }
    
    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < self.gap_y - half_size;
        let player_below_gap = player.y > self.gap_y + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }

   
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - Simple")
        .build()?;
    main_loop(context, State::new())
}
