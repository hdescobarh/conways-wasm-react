mod web;

use crate::universe::Observer;

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
    observer.forward_view(&width, &height);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![2, 3, 0], observer.map_row);

    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    observer.forward_view(&width, &height);
    assert_eq!(vec![max_col, 0, 1], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);

    observer.forward_view(&width, &height);
    assert_eq!(vec![0, 1, 2], observer.map_col);
    assert_eq!(vec![max_row, 0, 1], observer.map_row);
}
