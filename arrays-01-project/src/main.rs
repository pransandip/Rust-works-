// Collection Types: arrays, vectors <Vec>
//                 ARRAYS

fn arr_of_str() {
    let arr1 = ["one", "two"];
    let arr2 = ["a"; 10];
    println!("{:?}", arr1);
    println!("{:?}", arr2);
}

// Creates an iterator which gives the current iteration count as well as the next value.

// The iterator returned yields pairs (i, val), where i is the current
// index of iteration and val is the value returned by the iterator.

// enumerate() keeps its count as a usize. If you want to count by a
// different sized integer, the [zip] function provides similar functionality.

fn iter_example() {
    let ax: [char; 3] = ['a', 'b', 'c'];

    let mut iter = ax.iter().enumerate();

    println!("{:?}", iter);
    // println!("{:?}", iter.next());

    assert_eq!(iter.next(), Some((0, &'a')));
    assert_eq!(iter.next(), Some((1, &'b')));
    assert_eq!(iter.next(), Some((2, &'c')));
    assert_eq!(iter.next(), None);
}

fn main() {
    arr_of_str();
    iter_example();
}
