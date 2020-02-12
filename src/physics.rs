use std::collections::HashMap;
use crate::vectors::Vector2;

pub struct AABB
{
    pub x1: f32,
    pub x2: f32,
    pub y1: f32,
    pub y2: f32,
}

impl AABB
{
    pub fn from_center(position: Vector2, width: f32, height: f32) -> AABB
    {
        AABB {
            x1: position.x - width / 2.0,
            x2: position.x + width / 2.0,
            y1: position.y - height / 2.0,
            y2: position.y + height / 2.0,
        }
    }

    pub fn get_points(&self) -> Vec<Vector2>
    {
        let mut points = Vec::new();
        points.push(Vector2{x: self.x1, y: self.y1});
        points.push(Vector2{x: self.x2, y: self.y1});
        points.push(Vector2{x: self.x1, y: self.y2});
        points.push(Vector2{x: self.x2, y: self.y2});
        return points;
    } 
}

pub struct PhysicsManager
{
    id_counter: u32,
    aabb_colliders: HashMap<u32, AABB>,
}

impl PhysicsManager
{
    pub fn new() -> PhysicsManager
    {
        PhysicsManager{ id_counter: 0, aabb_colliders: HashMap::new() }
    }

    pub fn add_collider(&mut self, collider: AABB) -> u32
    {
        let id = self.id_counter;
        self.aabb_colliders.insert(id, collider);
        self.id_counter += 1;
        return id;
    }

    pub fn update_collider(&mut self, id: u32, new_value: AABB)
    {
        *self.aabb_colliders.get_mut(&id).unwrap() = new_value;
    }

    pub fn _log_colliders(&self)
    {
        for (id, collider) in &self.aabb_colliders
        {
            println!("{}. X1: {} X2: {} Y1: {} Y2: {}", id, collider.x1, collider.x2, collider.y1, collider.y2);
        }
    }

    pub fn check_collision(&self, collider: &AABB, exclude_id: &u32) -> bool
    {
        for (id, aabb) in &self.aabb_colliders
        {
            if id == exclude_id
            {
                continue;
            }
            for point in collider.get_points().iter()
            {
                if point.x >= aabb.x1 && point.x <= aabb.x2
                && point.y >= aabb.y1 && point.y <= aabb.y2
                {
                    return true;
                }
            }
        }
        return false;
    }
}