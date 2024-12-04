use data_structures_in_rust::array_queue_generic::ArrayQueueGeneric;

#[derive(Debug, Default, Copy, Clone)] 
struct Pair {
    x: i32,
    y: i32
}

impl Pair {
    fn new(x: i32, y: i32) -> Self {
        Pair { x, y}
    }
}

fn main() {
    let mut a_q = ArrayQueueGeneric::<Pair>::new();
    println!("{:?}", a_q);
    for i in 0..5 {
        a_q.enqueue(Pair::new(i, i));
    }
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(5, 5));
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.dequeue());
    println!("{:?}", a_q.peek());
    a_q.dequeue();
    a_q.dequeue();
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(6, 6));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(7, 7));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(8, 8));
    println!("{:?}", a_q);
    a_q.enqueue(Pair::new(9, 9));
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.dequeue());
    }
    println!("{:?}", a_q.dequeue());
}
