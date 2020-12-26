use crate::extensions::NodeExt as _;
use gdnative::api::{KinematicBody, CollisionShape, MeshInstance, Camera};
//use gdnative::api::*;
use gdnative::prelude::*;
use crate::player::JumpStage::{LATER_JUMP, INITIAL_JUMP, NO_JUMP};
use crate::fsm::State;

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
}

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
}

const SPEED: f32 = 15.0;
const GRAV: f32 = 400.0;
const JUMP: f32 = 10.0;
//const GRAV: Vector3 = Vector3::new(0.0,-1.0,0.0) * 20.0;

#[derive(PartialEq)]
enum JumpStage {
    NO_JUMP,
    INITIAL_JUMP,
    LATER_JUMP,
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
            jump: NO_JUMP,
            jump_held: false,
            end_jump_timer: Node::new().into_shared(),
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

        let cam = unsafe {self.cam.assume_safe()};
        let cam = cam.cast::<Camera>().unwrap();
        let camgt = cam.global_transform();

        let timer = unsafe { self.jump_timer.assume_safe() };
        let timer = timer.cast::<Timer>().unwrap();

        let end_timer = unsafe { self.end_jump_timer.assume_safe() };
        let end_timer = end_timer.cast::<Timer>().unwrap();

        //let initial_rotation = cam.rotation().y;

        let input = Input::godot_singleton();

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
            self.jump = INITIAL_JUMP;
            self.jump_held = true;
            timer.start(0.3);
            end_timer.start(0.4);
        }

        else if self.jump == INITIAL_JUMP {
            if !Input::is_action_pressed(&input, "ui_jump") {
                self.jump_held = false;
                self.velocity.y = JUMP;
            }
            else {
                self.velocity.y = JUMP;
            }
        }
        else if self.jump == LATER_JUMP {
            if self.jump_held {
                if !Input::is_action_pressed(&input, "ui_jump") {
                    self.jump_held = false;
                    self.velocity.y = -1.0 * GRAV * delta;
                    self.jump = NO_JUMP;
                }
                else {
                    self.velocity.y = JUMP;
                }

            }
            else {
                self.jump = NO_JUMP;
                self.velocity.y = -1.0 * GRAV * delta;
            }
        }

        //self.velocity = owner.move_and_slide(self.velocity,self.up, false, 4, 0.785, true);
        self.velocity = owner.move_and_slide_with_snap(self.velocity, grounded, self.up, false, 4, 0.785, true);

    }

    #[export]
    fn _on_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = LATER_JUMP;
        godot_print!("Does this work?");

    }

    #[export]
    fn _on_end_timer_timeout(&mut self, owner: &KinematicBody) {
        self.jump = NO_JUMP;
    }

}
