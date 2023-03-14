use ggez::glam::Vec2;

// only tells us if two objects are collided, nothing more
pub fn collide_rec_aabb(a_pos: Vec2, a_size: Vec2, b_pos: Vec2, b_size: Vec2) -> bool {
    let x = a_pos.x + a_size.x >= b_pos.x && b_pos.x + b_size.x >= a_pos.x;
    let y = a_pos.y + a_size.y >= b_pos.y && b_pos.y + b_size.y >= a_pos.y;
    
    x && y
}
