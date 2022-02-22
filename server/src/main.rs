use agent::Agent;
use glam::Vec2;


fn main() {
    println!("Hello, world!");

    /*
        Priorities:
        Basic traits
        Vector math (import of homemade)
        Basic agent
        Server
    */

    let a = Agent{
        position: Vec2::new(0.0, 0.0),
        velocity: Vec2::new(0.0, 0.0),
        acceleration: Vec2::new(0.0, 0.0),
        friction: 0.0
    };
}
