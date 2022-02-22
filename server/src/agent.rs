use glam{Vec2};

pub impl Agent {
    pub fn update(delta: f32);
    pub fn apply_force(delta: f32, force: Vec2);
}

pub struct Agent {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub friction: f32,
}

pub impl Agent for Agent {
    pub fn update(delta: f32) {
        self.velocity += self.acceleration;
        self.velocity *= self.friction;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    pub fn apply_force(delta: f32, force: Vec2) {
        self.acceleration += force;
    }
}
