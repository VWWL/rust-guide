use crate::dojo::mars_rover_journey::area::Area;
use crate::dojo::mars_rover_journey::degree::Degree;
use crate::dojo::mars_rover_journey::direction::Direction;
use crate::dojo::mars_rover_journey::location::Location;

pub struct Coordinate {
    location: Location,
    degree: Degree,
}

impl Coordinate {
    pub fn set(x: i32, y: i32, direction: Direction, area: Area) -> Coordinate {
        Self {
            location: Location::set(x, y, area),
            degree: Degree::from_direction(direction),
        }
    }

    pub fn report(&self) -> String {
        format!("{} and {}.", self.location.report(), self.degree.report())
    }

    pub fn turn_left(&mut self) {
        self.degree.turn_left();
    }

    pub fn turn_right(&mut self) {
        self.degree.turn_right();
    }

    pub fn move_toward(&mut self) {
        let (x, y) = self.degree.calculate_length();
        self.location.move_toward(x * 1.0, y * 1.0);
    }

    pub fn move_back(&mut self) {
        let (x, y) = self.degree.calculate_length();
        self.location.move_toward(-x * 1.0, -y * 1.0);
    }
}
