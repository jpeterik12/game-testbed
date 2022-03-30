/*
 * This file is part of game-testbed.
 * game-testbed is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * any later version.
 * game-testbed is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with game-testbed. If not, see <https://www.gnu.org/licenses/>.
 */

mod ecs;

use crate::ecs::systems::Query;

fn main() {
    let mut world = ecs::World::new();
    let entity = world.add();
    world.insert(entity, ecs::Position { x: 0.0, y: 1.0 });
    world.insert(entity, ecs::Velocity { x: 2.0, y: 3.0 });
    println!("world size: {}", world.size);
    let (p, v) = <(*mut ecs::Position, *mut ecs::Velocity)>::matches(&mut world, entity).unwrap();
    unsafe {
        println!(
            "entity position and velocity: {} {} {} {}",
            (*p).x,
            (*p).y,
            (*v).x,
            (*v).y
        );
    }
}
