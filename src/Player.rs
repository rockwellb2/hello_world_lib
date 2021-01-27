use crate::extensions::NodeExt as _;
use gdnative::api::{KinematicBody, CollisionShape, MeshInstance, Camera};
//use gdnative::api::*;
use gdnative::prelude::*;
use crate::player::JumpStage::{LaterJump, InitialJump, NoJump};
use crate::player::PlayerState::{AirState, GroundState, WallState};
/*use crate::fsm::State;

struct GroundState<'a> {
    name: String,
    owner: &'a KinematicBody
}

impl GroundState<'_> {
    fn new(owner: &KinematicBody) -> GroundState {
        GroundState {
            name: "GroundState".to_string(),
            owner,
        }
    }
}


impl State for GroundState<'_> {
    /*fn new(name: String) -> NewState {
        NewState { name }
    }*/

    fn update(&self) -> String {
        let mut return_state = self.name.clone();

        return_state


    }
}*/

/// The player "class"
#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
    gravity: Vector3,
    j_gravity: Vector3,
    //speed: f32,
    velocity: Vector3,
    up: Vector3,
    cam: Ref<Node>,
    jump_timer: Ref<Node>,
    jump: JumpStage,
    jump_held: bool,
    end_jump_timer: Ref<Node>,
    player_state: PlayerState,
    grounded: Vector3,
    velo_xz: Vector3,
}

const SPEED: f32 = 25.0;
const ACCEL: f32 = 200.0;
const DECEL: f32 = 50.0;
const AIR_DECEL: f32 = 50.0;
const AIR_ACCEL: f32 = 70.0;
const AIR_SPEED: f32 = 25.0;
const GRAV: f32 = 200.0;
const JUMP_GRAV: f32 = 100.0;
const JUMP: f32 = 50.0;
const SHORT_JUMP_TIME: f64 = 0.1;
const FULL_JUMP_TIME: f64 = 0.5;
//const GRAV: Vector3 = Vector3::new(0.0,-1.0,0.0) * 20.0;

#[derive(PartialEq)]
#[derive(Debug)]
enum JumpStage {
    NoJump,
    InitialJump,
    LaterJump,
}

/*impl std::fmt::Display for JumpStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}*/

#[derive(PartialEq)]
enum PlayerState {
    GroundState,
    AirState,
    WallState
}


#[methods]
impl Player {

    fn new(_owner: &KinematicBody) -> Self {
        Player {
            gravity: Vector3::new(0.0,-1.0,0.0) * GRAV,
            j_gravity: Vector3:: new(0.0, -1.0, 0.0) * JUMP_GRAV,
            //speed: 4.0,
            velocity: Vector3::zero(),
            up: Vector3::new(0.0, 1.0, 0.0),
            cam: Node::new().into_shared(),
            jump_timer: Node::new().into_shared(),
            jump: NoJump,
            jump_held: false,
            end_jump_timer: Node::new().into_shared(),
            player_state: AirState,
            grounded: Vector3::new(0.0,-1.0,0.0),
            velo_xz: Vector3::zero(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &KinematicBody) {
        self.cam = owner
            .get_node("Target/PlayCamera")
            .expect("PlayCamera node Should Exist");

        self.jump_timer = owner
            .get_node("JumpTimer")
            .expect("JumpTimer node Should Exist");

        self.end_jump_timer = owner
            .get_node("EndJumpTimer")
            .expect("EndJumpTimer node Should Exist");


    }

    #[export]
    fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
        //self.velocity = Vector3::new(0.0,0.0,0.0);
        //self.velocity = self.gravity * delta;

        self.player_move(owner, delta);
        /*let cam = unsafe {self.cam.assume_safe()};
        let cam = cam.cast::<Camera>().unwrap();
        let camgt = cam.global_transform();

        let timer = unsafe { self.jump_timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();

        let end_timer = unsafe { self.end_jump_timer.assume_safe() };
        let end_timer = end_timer.cast::<Timer>().unwrap();*/

        /*match self.player_state {
            PlayerState::GroundState => {


            }

            PlayerState::AirState => {

            }
        }*/

        //let initial_rotation = cam.rotation().y;

       /* let input = Input::godot_singleton();

        if Input::is_action_pressed(&input, "ui_right") {
            self.velocity += camgt.basis.x() * SPEED;
        }
        if Input::is_action_pressed(&input, "ui_left") {
            self.velocity -= camgt.basis.x() * SPEED;
        }
        if Input::is_action_pressed(&input, "ui_down") {
            self.velocity += camgt.basis.z() * SPEED;
        }
        if Input::is_action_pressed(&input, "ui_up") {
            self.velocity -= camgt.basis.z() * SPEED;
        }
        self.velocity.normalize();
        self.velocity.y = 0.0;
        self.velocity += self.gravity * delta;

        let mut grounded: Vector3 = Vector3::new(0.0,-1.0,0.0);

        if owner.is_on_floor() && Input::is_action_pressed(&input, "ui_jump") {
            grounded = Vector3::zero();
            self.velocity.y = JUMP;
            self.jump = InitialJump;
            self.jump_held = true;
            timer.start(0.3);
            end_timer.start(0.4);
        }

        else if self.jump == InitialJump {
            if !Input::is_action_pressed(&input, "ui_jump") {
                self.jump_held = false;
                self.velocity.y = JUMP;
            }
            else {
                self.velocity.y = JUMP;
            }
        }
        else if self.jump == LaterJump {
            if self.jump_held {
                if !Input::is_action_pressed(&input, "ui_jump") {
                    self.jump_held = false;
                    self.velocity.y = -1.0 * GRAV * delta;
                    self.jump = NoJump;
                }
                else {
                    self.velocity.y = JUMP;
                }

            }
            else {
                self.jump = NoJump;
                self.velocity.y = -1.0 * GRAV * delta;
            }
        }

        //self.velocity = owner.move_and_slide(self.velocity,self.up, false, 4, 0.785, true);
        self.velocity = owner.move_and_slide_with_snap(self.velocity, grounded, self.up, false, 4, 0.785, true);
*/
    }

    // Used inside _physics_process
    #[export]
    fn player_move(&mut self, owner: &KinematicBody, delta: f32) {
        // Get PlayCamera and its global transform
        /*let cam = unsafe {self.cam.assume_safe()};
        let cam = cam.cast::<Camera>().unwrap();
        let camgt = cam.global_transform();*/

        /*// Get JumpTimer
        let timer = unsafe { self.jump_timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();

        // Get EndTimer
        let end_timer = unsafe { self.end_jump_timer.assume_safe() };
        let end_timer = end_timer.cast::<Timer>().unwrap();*/

        /*// Get Input singleton
        let input = Input::godot_singleton();*/

        //let mut return_state: PlayerState = PlayerState::GroundState;

        self.grounded = Vector3::new(0.0,-1.0,0.0);
        //let input = Input::godot_singleton();

        // Get movement inputs to set velocity, and handle jump actions
        self.move_input(owner, delta);
        self.handle_jump(owner, delta);

        self.velocity.x = self.velo_xz.x;
        self.velocity.z = self.velo_xz.z;

        self.velocity = owner.move_and_slide_with_snap(self.velocity, self.grounded, self.up, false, 4, 0.785, true);

        /*match &self.player_state {
            &PlayerState::GroundState => {
                self.jump = NoJump;
                self.jump_held = false;

                //godot_print!("Does this work?");

                //let input = Input::godot_singleton();

                if Input::is_action_pressed(&input, "ui_right") {
                    self.velocity += camgt.basis.x() * SPEED;
                }
                if Input::is_action_pressed(&input, "ui_left") {
                    self.velocity -= camgt.basis.x() * SPEED;
                }
                if Input::is_action_pressed(&input, "ui_down") {
                    self.velocity += camgt.basis.z() * SPEED;
                }
                if Input::is_action_pressed(&input, "ui_up") {
                    self.velocity -= camgt.basis.z() * SPEED;
                }

                //self.velocity.normalize();
                self.velocity.y = 0.0;
                //self.velocity += self.gravity * delta;

                if Input::is_action_pressed(&input, "ui_jump") {
                    grounded = Vector3::zero();
                    self.velocity.y = JUMP;
                    self.jump = InitialJump;
                    self.jump_held = true;
                    timer.start(SHORT_JUMP_TIME);
                    end_timer.start(FULL_JUMP_TIME);
                }

                self.velocity = self.velocity.with_max_length(SPEED);

                self.velocity = owner.move_and_slide_with_snap(self.velocity, grounded, self.up, false, 4, 0.785, true);
            }

            &PlayerState::AirState => {
                let mut grounded: Vector3 = Vector3::new(0.0, -1.0, 0.0);

                if Input::is_action_pressed(&input, "ui_right") {
                    self.velocity += camgt.basis.x() * AIR_SPEED;
                }
                if Input::is_action_pressed(&input, "ui_left") {
                    self.velocity -= camgt.basis.x() * AIR_SPEED;
                }
                if Input::is_action_pressed(&input, "ui_down") {
                    self.velocity += camgt.basis.z() * AIR_SPEED;
                }
                if Input::is_action_pressed(&input, "ui_up") {
                    self.velocity -= camgt.basis.z() * AIR_SPEED;
                }

                self.velocity.y = 0.0;

                if self.jump == InitialJump {
                    self.velocity.y = JUMP;
                    if !Input::is_action_pressed(&input, "ui_jump") {
                        self.jump_held = false;
                    }
                } else if self.jump == LaterJump {
                    if self.jump_held {
                        if !Input::is_action_pressed(&input, "ui_jump") {
                            self.jump_held = false;
                            self.velocity.y = -1.0 * GRAV * delta;
                            self.jump = NoJump;
                        } else {
                            self.velocity.y = JUMP;
                        }
                    } else {
                        self.jump = NoJump;
                        self.velocity.y = -1.0 * GRAV * delta;
                    }
                }

                else {
                    self.velocity += self.gravity * delta;
                }

                self.velocity = owner.move_and_slide_with_snap(self.velocity, grounded, self.up, false, 4, 0.785, true);
            }
        }*/

       /* if owner.is_on_floor() {
            return_state = GroundState;
        }
        else {
            return_state = AirState;
        }
        self.player_state = return_state;*/

        if owner.is_on_floor() {
            self.player_state = GroundState;
        }

        /*else if owner.is_on_wall() {

        }*/

        else {
            self.player_state = AirState;
        }

        //let length: f32 = self.velocity.length();
        //godot_print!("Velocity: {}", length);
        //godot_print!("JumpState: {:?}", self.jump)
    }

    // Method to get movement Inputs
    #[export]
    fn move_input(&mut self, owner: &KinematicBody, delta: f32) {
        // Get PlayCamera and its global transform
        let cam = unsafe {self.cam.assume_safe()};
        let cam = cam.cast::<Camera>().unwrap();
        let cam_trans = cam.global_transform();

        // Get Input singleton
        let input = Input::godot_singleton();

        let speed = match self.player_state {
            GroundState => {SPEED}
            AirState => {AIR_SPEED}
            WallState => {AIR_SPEED}
        };

        let accel = match self.player_state {
            GroundState => {ACCEL}
            AirState => {AIR_ACCEL}
            WallState => {AIR_ACCEL}
        };

        let decel = match self.player_state {
            GroundState => {DECEL}
            AirState => {AIR_DECEL}
            WallState => {AIR_DECEL}
        };


        //godot_print!("Does this work?");

        let mut input_v: Vector3 = Vector3::zero();

        if Input::is_action_pressed(input, "ui_right") {
            input_v += cam_trans.basis.x() * 100.0;
        }
        if Input::is_action_pressed(input, "ui_left") {
            input_v -= cam_trans.basis.x() * 100.0;
        }
        if Input::is_action_pressed(input, "ui_down") {
            input_v += cam_trans.basis.z() * 100.0;
        }
        if Input::is_action_pressed(input, "ui_up") {
            input_v -= cam_trans.basis.z() * 100.0;
        }

        let new_input_v = input_v.normalize();



        if input_v != Vector3::zero() {
            self.velo_xz = self.move_toward(self.velo_xz, new_input_v * speed, accel * delta);
        }
        else  {
            self.velo_xz = self.move_toward(self.velo_xz, Vector3::zero(), decel * delta);
        }

        //self.velo_xz = self.velo_xz.with_max_length(speed);
        self.velo_xz.y = 0.0;
    }

    #[export]
    fn handle_jump(&mut self, owner: &KinematicBody, delta:f32) {
        // Get Input singleton
        let input = Input::godot_singleton();

       /*if owner.is_on_wall() {
            let kc = owner.get_slide_collision(0);

            // makes sure the collision exists
            let collision = match kc {
                Some(x) => x,
                None => panic!(),
            };

            //gets normal of the collision
            let normal: Vector3 = unsafe { collision.assume_safe().normal() };

            // vector3 of the input
            let mut dir: Vector3 = Vector3::zero();

            if Input::is_action_pressed(input, "ui_right") {
               dir.x += 1.0;
            }
            if Input::is_action_pressed(input, "ui_left") {
                dir.x -= 1.0;
            }
            if Input::is_action_pressed(input, "ui_down") {
                dir.z += 1.0;
            }
            if Input::is_action_pressed(input, "ui_up") {
                dir.z -= 1.0;
            }

            if normal.x == -dir.x {

            }

            else if normal.z == -dir.z {

            }


            //godot_print!("Normal x: {}, y: {}, z: {}", normal.x, normal.y, normal.z);

        }*/



        // Get JumpTimer
        let timer = unsafe { self.jump_timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();

        // Get EndTimer
        let end_timer = unsafe { self.end_jump_timer.assume_safe() };
        let end_timer = end_timer.cast::<Timer>().unwrap();


        if Input::is_action_pressed(input, "ui_jump") && self.player_state == GroundState {
            self.grounded = Vector3::zero();
            self.velocity.y = JUMP;
            self.jump = InitialJump;
            self.jump_held = true;
            timer.start(SHORT_JUMP_TIME);
            end_timer.start(FULL_JUMP_TIME);
        }

        else if !Input::is_action_pressed(input, "ui_jump") {
            self.jump_held = false;
        }

        if self.jump == LaterJump && !self.jump_held {
            self.jump = NoJump;
        }



        /*if self.jump == LaterJump && !self.jump_held {
            self.velocity.y = 0.0;
        }*/

        /*if self.player_state == AirState {
            self.velocity += self.gravity * delta;
        }*/

        if self.jump == InitialJump || self.jump == LaterJump {
            self.velocity += self.j_gravity * delta;
        }

        else if self.jump == NoJump {
            self.velocity += self.gravity * delta;
        }




        /*if (self.jump == InitialJump) || (self.jump_held && self.jump == LaterJump) {
            self.velocity.y = JUMP;
        }

        else {
            self.velocity += self.gravity * delta;
        }*/


    }

    // Timer signal that signifies end of regular jump
    #[export]
    fn _on_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = LaterJump;
        godot_print!("Does this work?");

    }

    // Timer signal that signifies end of longest jump
    #[export]
    fn _on_end_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = NoJump;
    }

    fn move_toward(&self, v: Vector3, p_to: Vector3, p_delta: f32) -> Vector3 {
        let vd: Vector3 = p_to - v;
        let len: f32 = vd.length();
        if len <=  p_delta || len < 0.00001 {
            p_to
        }
        else {
            v + vd / len * p_delta
        }
    }

}
