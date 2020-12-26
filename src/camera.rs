use crate::extensions::NodeExt as _;
use gdnative::api::{Camera, CollisionShape, MeshInstance, KinematicBody, Spatial};
use gdnative::prelude::*;

/// The camera "class"
#[derive(NativeClass)]
#[inherit(Camera)]
pub struct PlayCamera {
    player: Ref<Node>,
    target: Ref<Node>,
}

//const SPEED: f32 = 4.0;
//const GRAV: Vector3 = Vector3::new(0.0,-1.0,0.0) * 20.0;


#[methods]
impl PlayCamera {

    fn new(_owner: &Camera) -> Self {
        PlayCamera {
            player: Node::new().into_shared(),
            target: Node::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Camera) {
        self.player = owner
            .get_node("../..")
            .expect("Player node Should Exist");

        self.target = owner
            .get_node("..")
            .expect("Target node Should Exist");

        /*let player = unsafe {self.player.assume_safe()};
        let player = player.cast::<KinematicBody>().unwrap();

        let gt = player.global_transform();
        let point = gt.origin;
        let offset = Vector3::new(0.0, 4.0, 10.0);
        let cam_point = point + offset;
        owner.set_translation(cam_point);*/
    }

    #[export]
    fn _physics_process(&mut self, owner: &Camera, delta: f32) {
        let target = unsafe {self.target.assume_safe()};
        let target = target.cast::<Spatial>().unwrap();

        /*
        let gt = player.global_transform();
        let point = gt.origin;*/

        //let initial_rot = owner.rotation().y;

        //owner.look_at(point, Vector3::new(0.0,1.0,0.0));
       // let offset = Vector3::new(0.0, 4.0, 10.0);
        //let cam_point = point + offset;

        //owner.set_translation(cam_point);
        //owner.look_at(point, Vector3::new(0.0,1.0,0.0));

        // take 2
        let input = Input::godot_singleton();

        let mut ang: f32 = 0.0;

        if Input::is_action_pressed(&input, "cam_right") {
            ang +=  delta;
        }
        if Input::is_action_pressed(&input, "cam_left") {
            ang -=  delta;
        }

        target.rotate_y(ang.into());
        target.orthonormalize();


        //let q = owner.rotation();



        //let rot = ang + owner.rotation().y;
        /*let rot = ang + initial_rot;
        let t_start = point;
        owner.global_translate(t_start);
        let nbasis = owner.transform().basis.rotated(Vector3::new(0.0,1.0,0.0), rot);
        let mut new_trans = owner.transform();
        new_trans.basis = nbasis;
        owner.set_transform(new_trans);
        owner.global_translate(t_start);*/

        // let's see if this works
        // let mut rot = owner.global_transform();
        // rot.origin = point;
        // owner.set_global_transform(rot);

        //owner.rotate_y(f32::PI() * delta);

        //let input = Input::godot_singleton();

        /*let mut ang: f32 = 0.0;

        if Input::is_action_pressed(&input, "cam_right") {
            ang += delta;
        }
        if Input::is_action_pressed(&input, "cam_left") {
            ang -= delta;
        }*/

        //owner.rotate_y(ang.into());

    }
}
