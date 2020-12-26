use std::io::prelude::*;
use std::collections::HashMap;

/*struct GroundState {
    name: String,
    //pub machine: Option<&'a StateMachine>,
}*/

pub(crate) trait State {
    //fn new(name: String) -> Self;

    fn update(&self) -> String;

}

impl<T: State> State for Option<T> {
    /*fn new(name: String) -> Self {
        Some(T::new(name))
    }*/

    fn update(&self) -> String {
        if let Some(inner) = self {
            return inner.update()
        }
        else {
            "bad".to_string()
        }
    }
}



impl<T:State> State for &T {
   /* fn new(name: String) -> Self {
        &*Box::new(T::new(name))
    }*/


    fn update(&self) -> String {
        (*self).update()
    }
}

/*impl<'a, T> State for T
    where & 'a T: State {
    fn new(name: String) -> Self {
        unimplemented!()
    }

    fn update(&self) -> String {
        unimplemented!()

    }
}
*/


/*impl GroundState {

}*/

/*impl State for GroundState {
    /*fn new(name: String) -> GroundState {
        GroundState {
            name,
            //machine: None //
            }
    }*/

    fn update(&self) -> String {

        "Something".to_string()
    }
}*/

struct StateMachine<T: State> {
    states: HashMap<String,T>,
    current_state: String,
    next_state: String,
}

impl<T> StateMachine<T>
where
    T: State{
    fn new(states: HashMap<String, T>, current_state: String) -> StateMachine<T> {
        StateMachine {
            states,
            current_state,
            next_state: "none".to_string(),
        }
    }

    fn update(&mut self) {



        self.next_state = self.states.get(&self.current_state).update();
    }

    fn change_current(&mut self, new_state: String) {
        self.current_state = String::from(new_state);
    }

    fn step(&mut self) {
        //let mut current_set = self.current_state.clone();
        self.update();
        if self.current_state != self.next_state {
            self.change_current(self.next_state.clone());
        }

        self.next_state = "none".to_string();
    }

    /*fn iter_add_machine(&self) {
        for (_name, &mut state) in self.states {
            state.machine = Some(&self);
        }
    }*/

}