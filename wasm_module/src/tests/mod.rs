mod web;
use crate::universe::Observer;
use crate::universe::Universe;

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

    let mut observer = Observer::new(&max_row, &max_col);
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
fn test_universe_single_time_step() {
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
}
