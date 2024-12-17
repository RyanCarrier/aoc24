use crate::util::{Direction, Problem};

pub const PROBLEM: Problem = Problem {
    part1,
    part2,
    test_data: Some(test_data),
};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Entity {
    Wall,
    Box,
    Empty,
    Box1,
    Box2,
}
struct Data {
    grid: Vec<Vec<Entity>>,
    robot: (isize, isize),
    movements: Vec<Direction>,
}
impl Data {
    fn run(&mut self) {
        self.movements.iter().for_each(|d| {
            let (mut y, mut x) = self.robot;
            let hopeful_robot = d.map_offset(self.robot);
            //no need for bounds as wall exist
            while self.grid[y as usize][x as usize] != Entity::Wall {
                (y, x) = d.map_offset((y, x));
                if self.grid[y as usize][x as usize] == Entity::Empty {
                    if self.robot.0.abs_diff(y) + self.robot.1.abs_diff(x) >= 1 {
                        // this just means there is ? boxes infront, but we can only
                        // move by one space anyway
                        self.grid[y as usize][x as usize] = Entity::Box;
                        self.grid[hopeful_robot.0 as usize][hopeful_robot.1 as usize] =
                            Entity::Empty;
                    }
                    self.robot = hopeful_robot;
                    break;
                }
            }
        });
    }

    fn run2(&mut self) {
        self.movements.clone().iter().for_each(|d| {
            let (mut y, mut x) = self.robot;
            let hopeful_robot = d.map_offset(self.robot);
            //no need for bounds as wall exist
            match self.grid[hopeful_robot.0 as usize][hopeful_robot.1 as usize] {
                Entity::Empty => return self.robot = hopeful_robot,
                Entity::Wall => return,
                _ => {}
            }

            if d.is_horizontal() {
                //honestly i just cbf so here we are
                while self.grid[y as usize][x as usize] != Entity::Wall {
                    (y, x) = d.map_offset((y, x));
                    if self.grid[y as usize][x as usize] == Entity::Empty {
                        let offset = d.get_offset().1;
                        while x != self.robot.1 {
                            self.grid[y as usize][x as usize] =
                                self.grid[y as usize][(x - offset) as usize];
                            x -= offset;
                        }
                        self.robot = hopeful_robot;
                        return;
                    }
                }
            } else {
                let mut all_boxes: Vec<Vec<(isize, isize)>> = vec![vec![self.robot]];
                //we have to clone as we need to mutate the vector
                // there are ways around this but this is AOC and idc
                // haha nvm
                loop {
                    let next_it = all_boxes
                        .last()
                        .unwrap()
                        .iter()
                        .map(|e| d.map_offset(*e))
                        .map(|e| (e, self.grid[e.0 as usize][e.1 as usize]));
                    //ok also look i know everything is just 'e' but thats what
                    //copilot initially filled with and I'm not changing it
                    if next_it.clone().any(|(_, e)| e == Entity::Wall) {
                        //EXIT CONDITION 1
                        return;
                    }
                    if next_it.clone().all(|(_, e)| e == Entity::Empty) {
                        //EXIT CONDITION 2
                        //walk through from the last row back to the initial row
                        // set everything to empty that it's moved from even though
                        // we will rewrite it on next iter
                        // this is just easier get over it
                        // not doing whole rows incase there is a box inside the set that is not pushed
                        // (imagine boxes surrounding an untouched box)
                        all_boxes.iter().rev().for_each(|row| {
                            row.iter().for_each(|pos| {
                                let next_pos = d.map_offset(*pos);
                                self.grid[next_pos.0 as usize][next_pos.1 as usize] =
                                    self.grid[pos.0 as usize][pos.1 as usize];
                                self.grid[pos.0 as usize][pos.1 as usize] = Entity::Empty;
                            });
                        });
                        self.robot = hopeful_robot;
                        return;
                    }
                    //now we know we have more BOXES
                    let mut new_next_row = vec![];
                    next_it
                        .filter(|(_, e)| *e == Entity::Box1 || *e == Entity::Box2)
                        .for_each(|(pos, e)| {
                            let mut not_contain_push = |p: (isize, isize)| {
                                if !new_next_row.contains(&p) {
                                    new_next_row.push(p);
                                }
                            };
                            not_contain_push(pos);
                            if e == Entity::Box1 {
                                not_contain_push((pos.0, pos.1 + 1));
                                // new_next_row.push((pos.0, pos.1 + 1));
                            } else {
                                not_contain_push((pos.0, pos.1 - 1));
                                // new_next_row.push((pos.0, pos.1 - 1));
                            }
                        });
                    all_boxes.push(new_next_row);
                }
            }
        });
    }
    fn sum(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, e)| (y, x, e)))
            .filter_map(|e| {
                if *e.2 == Entity::Box || *e.2 == Entity::Box1 {
                    Some(100 * e.0 + e.1)
                } else {
                    None
                }
            })
            .sum()
    }
    #[allow(dead_code)]
    fn print(&self) {
        self.grid.iter().enumerate().for_each(|(y, l)| {
            l.iter().enumerate().for_each(|(x, e)| {
                if (y as isize, x as isize) == self.robot {
                    print!("@");
                } else {
                    match e {
                        Entity::Wall => print!("#"),
                        Entity::Box => print!("O"),
                        Entity::Empty => print!("."),
                        Entity::Box1 => print!("["),
                        Entity::Box2 => print!("]"),
                    }
                }
            });
            println!();
        });
        println!("{:?}", self.robot);
    }
}

pub fn part1(lines: &[String]) -> String {
    let mut d = import(lines);
    d.run();
    d.sum().to_string()
}

pub fn part2(lines: &[String]) -> String {
    //118170 too low
    let mut d = import(lines);
    d.grid = d
        .grid
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|e| match e {
                    Entity::Box => [Entity::Box1, Entity::Box2],
                    Entity::Wall => [Entity::Wall, Entity::Wall],
                    _ => [*e, *e],
                })
                .collect()
        })
        .collect();
    d.robot = (d.robot.0, d.robot.1 * 2);
    d.run2();
    // d.print();
    d.sum().to_string()
}
pub fn test_data() -> &'static str {
    [
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
    ][0]
}

fn import(lines: &[String]) -> Data {
    let (grid_raw, movements_raw) =
        lines.split_at(lines.iter().position(|x| x.is_empty()).unwrap());
    let mut robot = (0_isize, 0_isize);
    let grid = grid_raw
        .iter()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => Entity::Wall,
                    'O' => Entity::Box,
                    '@' => {
                        robot = (y as isize, x as isize);
                        Entity::Empty
                    }
                    '.' => Entity::Empty,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();
    let movements = movements_raw
        .iter()
        .flat_map(|l| {
            l.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Invalid character"),
            })
        })
        .collect();

    Data {
        grid,
        robot,
        movements,
    }
}
