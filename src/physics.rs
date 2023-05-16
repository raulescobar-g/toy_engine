use std::time::{Duration, Instant};

pub struct TimeLoop {
    accumulator: f32,
    past_time: Instant,
}

pub struct State {
    
}

pub fn integrator(mut time: TimeLoop, mut states: (State, State)) -> (State,State) {
    let (prev_state, state) = states;

    let dt = time.past_time.elapsed().as_secs_f32();
    time.past_time = Instant::now();

    time.accumulator += dt;

    while time.accumulator >= dt {
        states = integrate((prev_state, state), dt);
        time.accumulator -= dt;
    }
    return states;
}

fn integrate(states: (State, State), dt: f32) -> (State, State) {
    let (prev_state, state) = states;
    let new_state = State {};

    todo!();

    return (state, new_state);
}
