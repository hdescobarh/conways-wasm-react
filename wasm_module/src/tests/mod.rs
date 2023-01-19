//#[cfg(target_arch = "wasm32")]
pub mod web;
use crate::universe::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_observer_forward() {
    let width = 4usize;
    let height = 4usize;

    let max_row = height - 1;
    let max_col = width - 1;

    let mut observer = observer::Observer::new(&max_row, &max_col);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![0, 1, 2], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![1, 2, 3], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![2, 3, 0], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![0, 1, 2], observer.map_row);

    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![1, 2, 3], observer.map_row);

    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    let jump = observer.forward_view(&width, &height);
    assert_eq!(true, jump);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![2, 3, 0], observer.map_row);

    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    let jump = observer.forward_view(&width, &height);
    assert_eq!(true, jump);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![0, 1, 2], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);
}

#[test]
fn test_universe_read() {
    let init_space = vec![
        true, true, false, false, true, true, false, false, false, false, true, false, false,
        false, false, false,
    ];
    // instance of Universe
    let universe = Universe::new(4, 4, init_space);
    // Reading i,j coordinates correctly
    assert_eq!(true, *universe.read_at_location(&1, &1));
    assert_eq!(false, *universe.read_at_location(&3, &3));
    assert_eq!(true, *universe.read_at_location(&0, &1));
}

#[test]
fn universe_single_time_step() {
    let init_space = vec![
        true, true, false, false, false, true, true, false, false, false, false, false, true,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    let space_one = vec![
        true, true, false, false, false, true, false, true, false, false, false, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_one, *universe.get_space_raw());
    assert_eq!(1, *universe.get_age());
}

#[test]
fn universe_pattern_a() {
    // empty after 2 steps
    let init_space = vec![
        false, true, false, false, false, true, false, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_t1 = vec![
        false, false, false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_t2 = vec![false; 25];

    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_t1, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_t2, *universe.get_space_raw());
    assert_eq!(2, *universe.get_age());
}

#[test]
fn universe_pattern_b() {
    // cyclic with period 2
    let init_space = vec![
        false, false, false, false, false, false, true, false, false, false, false, true, false,
        false, false, false, true, false, false, false, false, false, false, false, false,
    ];

    let space_t0 = init_space.clone();

    let space_t1 = vec![
        false, false, false, false, false, false, false, false, false, false, true, true, true,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];
    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_t1, *universe.get_space_raw());
    // other cycle
    universe.time_step();
    assert_eq!(space_t0, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_t1, *universe.get_space_raw());
    //other cycle
    universe.time_step();
    assert_eq!(space_t0, *universe.get_space_raw());
}

#[test]
fn universe_pattern_c() {
    // stable after first step
    let init_space = vec![
        true, true, false, false, false, true, false, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_tn = vec![
        true, true, false, false, false, true, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_tn, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_tn, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_tn, *universe.get_space_raw());
    assert_eq!(3, *universe.get_age());
}

#[test]
fn universe_pattern_d() {
    // empty after 2 steps
    let init_space = vec![
        false, false, false, false, false, false, false, true, false, false, false, true, false,
        false, false, true, false, false, false, false, false, false, false, false, false,
    ];

    let space_t1 = vec![
        false, false, false, false, false, false, false, false, false, false, false, true, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_t2 = vec![false; 25];

    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_t1, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_t2, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_t2, *universe.get_space_raw());
}

#[test]
fn universe_pattern_e() {
    // stable after 3 steps
    let init_space = vec![
        false, false, false, false, false, true, true, true, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_t1 = vec![
        false, true, false, false, false, true, true, false, false, false, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_t2 = vec![
        true, true, false, false, false, true, true, false, false, false, true, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ];

    let space_tn = vec![
        true, true, false, false, false, false, false, true, false, true, true, true, false, false,
        false, false, false, false, false, false, false, false, false, false, false,
    ];

    let mut universe = Universe::new(5, 5, init_space);
    universe.time_step();
    assert_eq!(space_t1, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_t2, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_tn, *universe.get_space_raw());
    universe.time_step();
    assert_eq!(space_tn, *universe.get_space_raw());
}
