use crate::extensions::NodeExt as _;
use gdnative::api::{KinematicBody, CollisionShape, MeshInstance, Camera};
//use gdnative::api::*;
use gdnative::prelude::*;
use crate::player::JumpStage::{LaterJump, InitialJump, NoJump};
use crate::player::PlayerState::{AirState, GroundState};
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
    //speed: f32,
    velocity: Vector3,
    up: Vector3,
    cam: Ref<Node>,
    jump_timer: Ref<Node>,
    jump: JumpStage,
    jump_held: bool,
    end_jump_timer: Ref<Node>,
    player_state: PlayerState,
}

const SPEED: f32 = 20.0;
const AIR_SPEED: f32 = 15.0;
const GRAV: f32 = 400.0;
const JUMP: f32 = 10.0;
const SHORT_JUMP_TIME: f64 = 0.3;
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
}


#[methods]
impl Player {

    fn new(_owner: &KinematicBody) -> Self {
        Player {
            gravity: Vector3::new(0.0,-1.0,0.0) * GRAV,
            //speed: 4.0,
            velocity: Vector3::zero(),
            up: Vector3::new(0.0, 1.0, 0.0),
            cam: Node::new().into_shared(),
            jump_timer: Node::new().into_shared(),
            jump: NoJump,
            jump_held: false,
            end_jump_timer: Node::new().into_shared(),
            player_state: AirState,
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
        self.velocity = Vector3::new(0.0,0.0,0.0);
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

    //used inside _physics_process
    #[export]
    fn player_move(&mut self, owner: &KinematicBody, delta: f32) {
        let cam = unsafe {self.cam.assume_safe()};
        let cam = cam.cast::<Camera>().unwrap();
        let camgt = cam.global_transform();

        let timer = unsafe { self.jump_timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();

        let end_timer = unsafe { self.end_jump_timer.assume_safe() };
        let end_timer = end_timer.cast::<Timer>().unwrap();

        let mut return_state: PlayerState = PlayerState::GroundState;

        let mut grounded: Vector3 = Vector3::new(0.0,-1.0,0.0);
        let input = Input::godot_singleton();
        //godot_print!("Does this work?");

        match &self.player_state {
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
        }

        if owner.is_on_floor() {
            return_state = GroundState;
        }
        else {
            return_state = AirState;
        }
        self.player_state = return_state;

        let length: f32 = self.velocity.length();
        //godot_print!("Velocity: {}", length);
        //godot_print!("JumpState: {:?}", self.jump)
    }

    #[export]
    fn _on_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = LaterJump;
        godot_print!("Does this work?");

    }

    #[export]
    fn _on_end_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = NoJump;
    }

}
