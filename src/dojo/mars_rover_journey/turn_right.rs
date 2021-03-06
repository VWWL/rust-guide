use crate::dojo::mars_rover_journey::command::Command;
use crate::dojo::mars_rover_journey::coordinate::Coordinate;
use crate::dojo::mars_rover_journey::turn::Turn;

pub struct TurnRight {}

impl TurnRight {
    pub fn make() -> Box<dyn Command> {
        Box::new(Self {})
    }
}

impl Command for TurnRight {
    fn execute(&self, coordinate: &mut Coordinate) {
        coordinate.turn_right();
    }
}

impl Turn for TurnRight {}
