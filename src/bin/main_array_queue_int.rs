fn main() {
    
    let mut a_q = data_structures_in_rust::array_int_queue::ArrayIntQueue::new();
    println!("{:?}", a_q);
    for i in 1..4 {
        a_q.enqueue(i);
    }
    println!("{:?}", a_q);
    a_q.enqueue(5);
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.dequeue());
    println!("{:?}", a_q.peek());
    a_q.dequeue();
    a_q.dequeue();
    println!("{:?}", a_q);
    a_q.enqueue(6);
    println!("{:?}", a_q);
    a_q.enqueue(7);
    println!("{:?}", a_q);
    a_q.enqueue(8);
    println!("{:?}", a_q);
    a_q.enqueue(9);
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.dequeue());
    }
    println!("{:?}", a_q.dequeue());
    
}

