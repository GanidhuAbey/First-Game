//inorder to get the origin point to the middle i will get screen height divide it by 2 then get screen width divide
//by two and put both those values into a Point2 coordinate.


//importing game engine
extern crate ggez;
extern crate rand;

//some imports havent been used yet
//imports audio
use ggez::audio;
//imports window and window settings
use ggez::conf;
//imports Keymap, Gameloop, and sets how certain values are returned
use ggez::event::{self, EventHandler, Keycode, Mod};
//imports screen graphics; what appears on the screen
use ggez::graphics;
//imports a tool to save coordinate points and offsets of the coordinates
use ggez::graphics::{Point2, Vector2};
//imports math tools
use ggez::nalgebra as na;
//allows the game to cap framerate at specific points to avoid 100% CPU at all times
use ggez::timer;
//imports screen builder, and imports a result type for game output
use ggez::{Context, ContextBuilder, GameResult};
//allows for drawing objects onto the screen
use ggez::graphics::DrawMode;

//helps find assets
use std::env;
use std::path;

use rand::Rng;

//creates the main game objects
enum ActorType {
    player,
    boss,
    shot,
}

//creates values for game objects
struct Actor {
    facing: f32,
    life: f32,
    size: Point2,
    tag: ActorType,
    pos: Point2,
}

//HP values of player and enemy
const PLAYER_LIFE: f32 = 1.0;
const BOSS_LIFE: f32 = 10.0;
//might be implemented later as a timer for the bullet
const SHOT_LIFE: f32 = 1.0;

//creates sizes for the game


//assigning values to player object
fn create_player() -> Actor {
    Actor {
        facing: 0.,
        life: PLAYER_LIFE,
        size: Point2::new(1.0, 1.0),
        tag: ActorType::player,
        pos: Point2::origin()
    }
}

//assigning values to enemy object
fn create_boss() -> Actor {
    Actor {
        facing: 0.,
        life: BOSS_LIFE,
        size: Point2::new(3.1, 3.1),
        tag: ActorType::boss,
        pos: Point2::origin(),
    }
}

//assigning values for bullets used in both enemy and player
fn create_shot() -> Actor {
    Actor {
        facing: 0.,
        life: SHOT_LIFE,
        size: Point2::new(1.0, 1.0),
        tag: ActorType::shot,
        pos: Point2::new(700.0, 700.0),
    }
}

//obselete functions
/*
fn move_player_x(input: &InputState, mut sv: f32) -> f32 {
    match input.x_value {
        1.0 => sv += 1.0,
        -1.0 => sv -= 1.0,
        _ => (),
    }
    return sv;
}
*/
//function to move player across y axis
/*
fn move_player_y(input: &InputState, mut sv: f32) -> f32 {
    match input.y_value {
        1.0 => sv += 1.0,
        -1.0 => sv -= 1.0,
        _ => (),
    }
    return sv;
}
*/



/*
fn player_model(pos: Point2, ctx: &mut Context) {
    graphics::circle(ctx,
                         DrawMode::Fill,
                         pos,
                         30.0,
                         1.0).unwrap();
    graphics::circle(ctx,
                         DrawMode::Fill,
                         Point2::new(350.0, 350.0),
                         50.0,
                         1.0).unwrap();
    graphics::present(ctx);
}
*/

//calculates where the where the player object is facing based on the rotation value given
fn forward_angle(angle: f32) -> Vector2 {
    let vx = angle.cos();
    let vy = angle.sin();

    Vector2::new(vx, vy)
}

//will help output origin point of game
fn screen_origin(screen_width: f32 , screen_height: f32, pos: Point2) -> Point2 {
    let width = screen_width;
    let height = screen_height;

    let origin_x = pos.x + width / 2.0;
    let origin_y = pos.y + height / 2.0;

    Point2::new(origin_x, origin_y)
}

//obselete function
/*
fn collision(pos: Point2) {

    let combined_radius = Point2::new(50.0, 50.0);
    let object_p = Point2::new(350.0, 350.0);

    let distance_x = object_p.x - pos.x;
    let distance_y = object_p.y - pos.y;

    let distance = Point2::new(distance_x, distance_y);

    if distance > Point2::new(0.0, 0.0) {
        if distance <= Point2::new(50.0, 50.0) {
            println!("x: {} , y: {}", pos.x, pos.y);
        }
    }
    else if distance <= Point2::new(0.0, 0.0) {
        if distance < Point2::new(750.0, 750.0) {
            println!("x: {} , y: {}", pos.x, pos.y);
        }
    }
    else if distance.x >= 0.0 && distance.y < 0.0  {
        if distance <= Point2::new(50.0, 750.0) {
            println!("x: {} , y: {}", pos.x, pos.y);
        }
    }
    else if distance.x <= 0.0 && distance.y > 0.0 {
        if distance < Point2::new(750.0, 50.0) {
            println!("x: {} , y: {}", pos.x, pos.y);
        }
    }

}
*/

//binds Keycodes "A" and "S" to the rotation of player
fn turn_rate(input: &InputState, actor: &mut Actor) {
    match input.x_value {
        1 => actor.facing += 0.1,
        -1 => actor.facing -= 0.1,
        _ => (),
    }
}

//draws the player object onto the game
fn draw_limits(ctx: &mut Context, coords: Point2, actor: &Actor, assets: &mut Assets) -> GameResult<()> {
    let position: Point2 = screen_origin(coords.x , coords.y, actor.pos);

    //binds a png image to the variable
    let image = assets.actor_image(actor);
    //creates values so the object can be controlled
    let drawparams = graphics::DrawParam {
        dest: position,
        rotation: actor.facing,
        scale: actor.size,
        offset: Point2::new(0.5 , 0.5),
        ..Default::default()
    };
    //draws the object onto the screen using the image and drawparams
    graphics::draw_ex(ctx, image, drawparams)
}

//uses the forward_angle function to figure out where player is facing, multiplys it by 3 so it moves forward. Only moves forward when "W" is pressed
fn move_player(actor: &mut Actor) {
    let forward_direction = forward_angle(actor.facing);
    let speed = forward_direction * (3.0);
    
    actor.pos += speed;
}

//when the player is out of the screen bounds they are teleported onto the other side.
fn wrap_player(actor: &mut Actor) {
    while actor.pos.x > 360.0 {
        actor.pos.x = -360.0;
    }while actor.pos.y > 550.0 {
        actor.pos.y = -200.0;
    }while actor.pos.x < -360.0 {
        actor.pos.x = 360.0;
    }while actor.pos.y < -200.0 {
        actor.pos.y = 550.0;
    }
}

//how the player shoots
fn player_shoot(input: &mut InputState, actor1: &mut Actor, actor2: &Actor) {
    //if "Spacebar" is pressed
    if input.fire == 1.0 {
        //teleport bullet to player 
        actor1.pos = actor2.pos;
        //align the bullet so its facing where the player is looking
        actor1.facing = actor2.facing;
        //this if statement runs only on the first loop
        input.fire -= 0.1;
    }
    //calculate direction the player is facing
    let mut path = forward_angle(actor1.facing);
    //multiply the coordinates each loop so it moves forward every frame
    path = path * (4.0);

    //add it to the current location of the bullet so it knows from where to move
    actor1.pos += path;

    if actor1.pos.x > 360.0 || actor1.pos.x < -360.0 || actor1.pos.y > 550.0 || actor1.pos.y < -200.0 {
        //once the bullet is outside of the screen set input fire to activate the if statement on press
        input.fire = 1.1;
    }
}
//teleports enemy bullet to enemy location
fn tele_bbullet(s_actor: &mut Actor, b_actor: &Actor, p_actor: &Actor, f_loop: u32) -> u32{
    if f_loop == 1 { 
        s_actor.pos = b_actor.pos;
        s_actor.facing = p_actor.facing;
    }
    //when bullet is out of screen radius it reteleports to enemy location
    if s_actor.pos.x > 360.0 || s_actor.pos.x < -360.0 || s_actor.pos.y > 550.0 || s_actor.pos.y < -350.0 {
        return 1;       
    }
    //else it doesnt teleport
    else{
        return f_loop + 1;
    }
}
//calculates the slope between player and enemy bullet, runs only once between lifecycle of bullet
fn bullet_path(s_actor: &Actor, p_actor: &Actor) -> Vector2 {
    //the values of both objects are zero at the start might be the cause of the slope not
    //working
    let player_position = p_actor.pos + Vector2::new(350., 175.);
    let bullet_position = s_actor.pos + Vector2::new(350., 350.);
    //cant sqareroot becuase the cooridnate might be negative
    let path = bullet_position - player_position;
    //normalizing path vector
    //turns out there is a function to square the numbers in a vector and add them together
    let additive_path = path.norm_squared();
    let sqrt_path = additive_path.sqrt();
    let final_path = path / (sqrt_path);
    return final_path;    
}
//because the values start at zero we have to hand craft the first value to point towards player
fn player_loc(actor: &Actor) -> u32 {
    if actor.pos == Point2::new(0., 0.) {
        return 1
    }
    else {
        return 2
    }
}
//shoots enemy bullet
fn boss_shoot(input: &mut InputState, s_actor: &mut Actor, pattern: u32, path: Vector2, loc: u32) -> u32 {
    //creates a cooridnate point that will be used if player_loc returns 1
    let mut direction = Vector2::new(0., -1.,);
    //planning to add more bullet patterns to enemy, hasnt been implemented yet
    if pattern == 1 {
        //variable holding return value of player_loc function
        //if it equals 1 then player point is zero and it shoots at the predetermind location
        if loc == 1 {
            direction = direction * (4.0);
            s_actor.pos += direction;
        }
        //else player is at a different location in which case we shoot at that location
        else {
            direction = path * (-4.0);
            s_actor.pos += direction;
        }
        
    }
    //pattern variable is being randomized from 1-4 if it isnt 1 it goes to the else statement to be reset
    else {
        let new_pattern = boss_should_shoot();
        return new_pattern;
    }
    
    //checks whether bullet is past screen bounds, if true
    
    if s_actor.pos.x > 360.0 || s_actor.pos.x < -360.0 || s_actor.pos.y > 550.0 || s_actor.pos.y < -350.0 {
        //obselete ignore
        input.boss_shoot = 0.0;
        //calculates a new bullet pattern, not implemented yet
        let new_pattern = boss_should_shoot();
        return new_pattern;
    }
    //else return the current pattern so the bullet continues to move
    else {
        return pattern;
    }
}

//uses rand crate to calculate a values for "pattern" variable used in the boss_shoot function
fn boss_should_shoot() -> u32 {
    let should_shoot = rand::thread_rng().gen_range(1, 4);

    return should_shoot;
}

//calculates whether the player hit the enemy object yet, doesnt fully work yet because im using length and witdh
//which results in the area of a square rather than a circle
fn player_collision(b_actor: &Actor, p_actor: &mut Actor) {
    let total_distance = p_actor.pos - b_actor.pos;    

    if total_distance.y > 110.95264 && total_distance.y < 233.90009 {
        if total_distance.x >= -58.999634 && total_distance.x <= 58.62108 {
            p_actor.pos = b_actor.pos; 
            println!("vector equals : {}", total_distance);
        }
    }
}
//uses the same logic as player_collision to determine whether the player bullet had made contact with the enemy object
fn player_shot_collision(input: &mut InputState, b_actor: &mut Actor, s_actor: &mut Actor) {
    let total_distance = s_actor.pos - b_actor.pos;    
    
    if total_distance.y > 110.95264 && total_distance.y < 233.90009 {
        if total_distance.x >= -58.999634 && total_distance.x <= 58.62108 {
            b_actor.life -= 1.0;
            s_actor.pos = Point2::new(800.0, 800.0);
            input.fire = 0.0;
            println!("BOSS HP: {}", b_actor.life);
        }
    }
}



//variables that will change based on KeyCode inputs and will be used to give some
//controll of the game objects
struct InputState {
    x_value: i32,
    y_value: i32,
    fire: f32,
    test: bool,
    //obselete function
    boss_shoot: f32,
}

//implements InputState values so they can be called from Mainstate later on
impl Default for InputState {
    fn default() -> Self {
        InputState {
            x_value: 0,
            y_value: 0,
            //probably could have done this part better
            fire: 1.1,
            test: false,
            boss_shoot: 0.0,
        }
    }
}

//create variable which will be binded to images from resource folder
struct Assets {
    player_image: graphics::Image,
    boss_image: graphics::Image,
    shot_image: graphics::Image,
}

//implementing Assets
impl Assets {
    //this function binds 3 variables with png images then binds those variables to the ones in the Assets struct
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_image = graphics::Image::new(ctx, "/player.png")?;
        let boss_image = graphics::Image::new(ctx, "/boss.png")?;
        let shot_image = graphics::Image::new(ctx, "/shot.png")?;

        Ok(Assets {
            player_image,
            boss_image,
            shot_image,
        })
    }
    //based on what actor im drawing it will bind a different picture to the drawparams
    fn actor_image(&mut self , actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::player => &mut self.player_image,
            ActorType::boss => &mut self.boss_image,
            ActorType::shot => &mut self.shot_image,
        }
    }
}



//holds everything that can be used in the Mainstate and the event loop, everything except functions
struct MainState {
    input: InputState,
    boss: Actor,
    player_bullet: Actor,
    player: Actor,
    boss_shot: Actor,
    f_loop: u32,
    loc: u32,
    path: Vector2,
    boss_pattern: u32,
    assets: Assets,
}

//this is like the loop but only runs once at the beginning to initalized Mainstate variables but never again
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        ctx.print_resource_stats();

        println!("Resource Path {:?}", ctx.filesystem);

        //assigning values to the variables in the struct 
        let player = create_player();
        let player_bullet = create_shot();
        let boss = create_boss();
        let assets = Assets::new(ctx)?;
        let boss_shot = create_shot();
        let boss_pattern = boss_should_shoot();
        let path = Vector2::new(0., 0.);

        let s = MainState {
            input: InputState::default(),
            boss,
            player_bullet,
            player,
            boss_shot,
            f_loop: 1,
            assets,
            loc: 0,
            path,
            boss_pattern,
        };

        Ok(s)

    }
}

//Where things will be drawn and functions activated
impl event::EventHandler for MainState {
    //this is where the framerate is set and the functions are called
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {

        //this is the framerate cap on the game, keeping it buttersmooth
        const  DESIRED_FPS: u32 = 60;

        while timer::check_update_time(ctx, DESIRED_FPS) {
            //thought this was what set the time (seconds over frames = framerate) however
            //it actually seems to do nothing 
            //im going to keep it till the final build and see if its still neccasary
            let _seconds = 1.0 / (DESIRED_FPS as f32);

            //more obselete functions, move along
            //self.x_position = move_player_x(&self.input, self.x_position);
            //self.y_position = move_player_y(&self.input, self.y_position);

            //self.pos = Point2::new(self.x_position, self.y_position); 

            //runs the turn function each frame to check if "A" or "S" has being pressed
            turn_rate(&self.input, &mut self.player);
            //if "W" has been pressed run move_player function
            if self.input.y_value == -1 {
                move_player(&mut self.player);
            }
            //always check to see if player is outside of screen bounds
            wrap_player(&mut self.player);

            //check if player fired, if true runs this function
            if self.input.fire <= 1.0 {
                player_shoot(&mut self.input, &mut self.player_bullet, &self.player);
            }
            //run this loop once per bullet cycle (from enemy location to offscreen)
            self.f_loop = tele_bbullet(&mut self.boss_shot, &self.boss, &self.player, self.f_loop);
            //run imediately after bullet teleportation but only run once
            if self.f_loop == 2 {
                //figures out slope between player and bullet
                self.path = bullet_path(&self.boss_shot, &self.player);
                //checks whether the player has moved since start of game
                self.loc = player_loc(&self.player);
            }
            //calls the boss_shoot function into the loop so the bullet can move
            self.boss_pattern = boss_shoot(&mut self.input, &mut self.boss_shot, self.boss_pattern, self.path, self.loc);
            
            //checks if player or player shot has collided
            player_collision(&self.boss, &mut self.player);
            player_shot_collision(&mut self.input, &mut self.boss, &mut self.player_bullet);
        }

        Ok(())
    }

    //drawing gameobjects onto the screen
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //clear all objects that were on in the previous loop so they dont overlap
        graphics::clear(ctx);

        //call assets on to the draw function
        let assets = &mut self.assets;
        //seperated the object drawings from everything else for cleanliness
        {
            //call the objects and draw the objects
            let player = &self.player;
            draw_limits(ctx, Point2::new(700.0, 350.0), player, assets).unwrap();

            let shot = &mut self.player_bullet;
            //making sure shot only draws if player wants to fire
            if self.input.fire <= 1.0 {
                draw_limits(ctx, Point2::new(700.0, 350.0), shot, assets).unwrap();
            }

            let boss = &self.boss;
            //makes sure boss disapears when they're dead (bullets still fire though since i havent implemented that yet)
            if boss.life > 0.0 { 
                draw_limits(ctx, Point2::new(700.0, 700.0), boss, assets).unwrap();
            }

            let boss_shot = &self.boss_shot;
            draw_limits(ctx, Point2::new(700.0, 700.0), boss_shot, assets).unwrap();
        }
        //draws everything onto the screen
        graphics::present(ctx);

        //gives CPU time to relax
        timer::yield_now();
        Ok(())

        
    }
    //when key pressed...
    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            //if w pressed then change y value to +
            Keycode::W => self.input.y_value = -1,
            //if s pressed then change y value to -
            Keycode::S => self.input.y_value = 1,
            //if d pressed then change x value to +
            Keycode::D => self.input.x_value = 1,
            //if a pressed then change x value to -
            Keycode::A => self.input.x_value = -1,
            //if spacebar is pressed change fire to true
            Keycode::Space => self.input.fire -= 0.1,
            //test new features
            Keycode::T => self.input.test = true,
            //quit if escape pressed
            Keycode::Escape => ctx.quit().unwrap(),
            //do nothing if any other key pressed
            _ => (),
        }
    }
    //when key released...
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool, ) {
        match keycode {
            //if w pressed then change y value to 0
            Keycode::W => {self.input.y_value = 0;
            self.input.boss_shoot += 1.0;},
            //if s pressed then change y value to 0
            Keycode::S => {self.input.y_value = 0;
            self.input.boss_shoot += 1.0;},
            //if d pressed then change x value to 0
            Keycode::A => {self.input.x_value = 0;
            self.input.boss_shoot += 1.0;},
            //if a pressed then change x value to 0
            Keycode::D => {self.input.x_value = 0;
            self.input.boss_shoot += 1.0;},
            //stop testing feature
            Keycode::T => {self.input.test = false;
            self.input.boss_shoot += 1.0},
            //if any other key released do nothing
            _ => self.input.boss_shoot += 1.0,
        }
    }
}

pub fn main() {
    //screen is created
    let mut cb = ContextBuilder::new("Boss Fight", "ggez")
        .window_setup(conf::WindowSetup::default().title("Boss Fight"))
        .window_mode(conf::WindowMode::default().dimensions(700, 700));
    
    //game doesnt require it to run, so im assuming it comes into effect when sharing the program
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        println!("Adding path {:?}", path);
            cb = cb.add_resource_path(path);
        }
        else {
            println!("Not building from cargo?  Ok.");
        }
    
    

    let ctx = &mut cb.build().unwrap();

    //check if its running and or been closed
    match MainState::new(ctx) {
        //if crashed print crash report
        Err(e) => {
            println!("Error encountered {}", e);
        }
        //if ok try running game loop
        Ok(ref mut game) => {
            let result = event::run(ctx, game);
                //if error caused during procces print error
                if let Err(e) = result {
                    println!("Error encountered during game {}", e);
                }
            //if game exited normally print message
            else {
                println!("Thanks for playing");
            }
        }
    }
    //
}