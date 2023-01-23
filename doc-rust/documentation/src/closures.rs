use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    //Define closures
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x: u32| x + 1;
    let add_one_v4 = |x| x + 1;

    add_one_v1(12);
    add_one_v2(12);
    add_one_v3(12);
    add_one_v4(12);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrow_mut = || list.push(7);

    borrow_mut();

    println!("After calling closure: {:?}", list);

    let second_list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", second_list);
    //move keyword force closure to take ownership of second list
    //The new thread generated here might be finish after the main thread  and need to take ownership
    //of the list, if the main thread finished to execute the list will be dropped and second list here
    //then will become a dengling reference.
    thread::spawn(move || println!("From thread: {:?}", second_list))
        .join()
        .unwrap();

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}
