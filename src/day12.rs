use crate::prelude::*;

type Int = i32;

pub fn day12() -> R<Answer<Int>> {
    let mut answer = Answer::new();
    let nav = parse_nav(&read_file("inputs/day12.txt")?)?;
    let boat = nav.iter().fold(Boat::new(), |boat, n| boat.navigate(*n));
    answer.part1(boat.coords.0.abs() + boat.coords.1.abs());

    let waypoint_boat = nav
        .iter()
        .fold(WaypointBoat::new(), |boat, n| boat.navigate(*n));
    answer.part2(waypoint_boat.boat.coords.0.abs() + waypoint_boat.boat.coords.1.abs());
    Ok(answer)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl Dir {
    fn signum(&self) -> Int {
        match self {
            Dir::N => 1,
            Dir::S => -1,
            Dir::E => 1,
            Dir::W => -1,
            Dir::L => 1,
            Dir::R => -1,
            Dir::F => 1,
        }
    }

    fn rotate_left(&self, r: Int) -> Dir {
        match r {
            90 => match self {
                Dir::N => Dir::W,
                Dir::W => Dir::S,
                Dir::S => Dir::E,
                Dir::E => Dir::N,
                _ => panic!("Invalid rotation"),
            },
            180 => match self {
                Dir::N => Dir::S,
                Dir::S => Dir::N,
                Dir::E => Dir::W,
                Dir::W => Dir::E,
                _ => panic!("Invalid rotation"),
            },
            270 => match self {
                Dir::N => Dir::E,
                Dir::E => Dir::S,
                Dir::S => Dir::W,
                Dir::W => Dir::N,
                _ => panic!("Invalid rotation"),
            },
            _ => panic!("Invalid rotation"),
        }
    }

    fn rotate_right(&self, r: Int) -> Dir {
        self.rotate_left(360 - r)
    }
}

#[derive(Copy, Clone, Debug)]
struct Nav {
    dir: Dir,
    r: Int,
}

impl Nav {
    fn abs(&self) -> Int {
        self.dir.signum() * self.r
    }
}

impl FromStr for Nav {
    type Err = String;
    fn from_str(input: &str) -> Result<Self, <Self as std::str::FromStr>::Err> {
        let first_character = input.chars().next().ok_or("Empty input for Nav")?;
        let dir = match first_character {
            'N' => Ok(Dir::N),
            'S' => Ok(Dir::S),
            'E' => Ok(Dir::E),
            'W' => Ok(Dir::W),
            'L' => Ok(Dir::L),
            'R' => Ok(Dir::R),
            'F' => Ok(Dir::F),
            _ => Err("Bad direction"),
        }?;
        let r = input[1..]
            .parse()
            .map_err(|e| format!("Invalid r value: {}", e))?;
        Ok(Nav { dir, r })
    }
}

fn parse_nav(input: &str) -> R<Vec<Nav>> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Copy, Clone, Debug)]
struct Boat {
    coords: (Int, Int),
    facing: Dir,
}

impl Boat {
    fn new() -> Boat {
        Boat {
            coords: (0, 0),
            facing: Dir::E,
        }
    }

    fn navigate(&self, nav: Nav) -> Boat {
        match nav.dir {
            Dir::N | Dir::S => match self.facing {
                Dir::N | Dir::S | Dir::E | Dir::W => Boat {
                    coords: (self.coords.0, self.coords.1 + nav.abs()),
                    facing: self.facing,
                },
                _ => panic!("Invalid navigation instruction: {:?}", nav),
            },
            Dir::E | Dir::W => match self.facing {
                Dir::N | Dir::S | Dir::E | Dir::W => Boat {
                    coords: (self.coords.0 + nav.abs(), self.coords.1),
                    facing: self.facing,
                },
                _ => panic!("Invalid navigation instruction: {:?}", nav),
            },
            Dir::L => Boat {
                coords: self.coords,
                facing: self.facing.rotate_left(nav.r),
            },
            Dir::R => Boat {
                coords: self.coords,
                facing: self.facing.rotate_right(nav.r),
            },
            Dir::F => Boat {
                coords: match self.facing {
                    Dir::N | Dir::S => {
                        (self.coords.0, self.coords.1 + nav.r * self.facing.signum())
                    }
                    Dir::E | Dir::W => {
                        (self.coords.0 + nav.r * self.facing.signum(), self.coords.1)
                    }
                    _ => self.coords,
                },
                facing: self.facing,
            },
        }
    }

    fn navigate_mut(&mut self, nav: Nav) {
        let moved = self.navigate(nav);
        self.coords = moved.coords;
        self.facing = moved.facing;
    }
}

#[derive(Debug, Clone, Copy)]
struct WaypointBoat {
    boat: Boat,
    waypoint: (Int, Int),
}

impl WaypointBoat {
    fn new() -> WaypointBoat {
        WaypointBoat {
            boat: Boat::new(),
            waypoint: (10, 1),
        }
    }

    fn ns(&self, amount: Int) -> WaypointBoat {
        WaypointBoat {
            boat: self.boat,
            waypoint: (self.waypoint.0, self.waypoint.1 + amount),
        }
    }

    fn ew(&self, amount: Int) -> WaypointBoat {
        WaypointBoat {
            boat: self.boat,
            waypoint: (self.waypoint.0 + amount, self.waypoint.1),
        }
    }

    fn rotate(&self, amount: Int) -> WaypointBoat {
        match amount {
            -90 | 270 => WaypointBoat {
                boat: self.boat,
                waypoint: (self.waypoint.1, -self.waypoint.0),
            },
            180 | -180 => WaypointBoat {
                boat: self.boat,
                waypoint: (-self.waypoint.0, -self.waypoint.1),
            },
            90 | -270 => WaypointBoat {
                boat: self.boat,
                waypoint: (-self.waypoint.1, self.waypoint.0),
            },
            _ => *self,
        }
    }

    fn navigate(&self, nav: Nav) -> WaypointBoat {
        match nav.dir {
            Dir::N | Dir::S => self.ns(nav.r * nav.dir.signum()),
            Dir::E | Dir::W => self.ew(nav.r * nav.dir.signum()),
            Dir::L | Dir::R => self.rotate(nav.r * nav.dir.signum()),
            Dir::F => WaypointBoat {
                waypoint: self.waypoint,
                boat: Boat {
                    facing: self.boat.facing,
                    coords: (
                        self.boat.coords.0 + self.waypoint.0 * nav.r,
                        self.boat.coords.1 + self.waypoint.1 * nav.r,
                    ),
                },
            },
        }
    }

    fn navigate_mut(mut self, nav: Nav) -> WaypointBoat {
        let moved = self.navigate(nav);
        self.boat = moved.boat;
        self.waypoint = moved.waypoint;
        self
    }
}
