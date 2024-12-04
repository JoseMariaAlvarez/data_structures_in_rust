use data_structures_in_rust::array_queue_box::ArrayQueueBoxLifetime;

#[derive(Debug, Default)] 
struct Triplet {
    x: i32,
    y: i32,
    z: String
}

impl Triplet {
    fn new(x: i32, y: i32, z: String) -> Self {
        Triplet { x, y, z}
    }

    fn set_z(&mut self, z: String) {
        self.z = z;
    }
}

fn main() {
    let mut a_q = ArrayQueueBoxLifetime::<Triplet>::new();
    println!("{:?}", a_q);
    let triplet0 = & mut Triplet::new(0, 0, 0.to_string());
    a_q.enqueue(triplet0);
    let triplet1 = & mut Triplet::new(1, 1, 1.to_string());
    a_q.enqueue(triplet1);
    let triplet2 = & mut Triplet::new(2, 2, 2.to_string());
    a_q.enqueue(triplet2);
    println!("{:?}", a_q);
    let triplet5 = & mut Triplet::new(5, 5, 5.to_string());
    a_q.enqueue(triplet5);
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    let elem = a_q.dequeue().unwrap();
    elem.set_z("five".to_string());
    println!("{:?}", elem);
    let elem = a_q.peek();
    println!("{:?}", elem);
    a_q.dequeue();
    a_q.dequeue();
    println!("{:?}", a_q);
    let triplet6 = & mut Triplet::new(6, 6, 6.to_string());
    a_q.enqueue(triplet6);
    println!("{:?}", a_q);
    let triplet7 = & mut Triplet::new(7, 7, 7.to_string());
    a_q.enqueue(triplet7);
    println!("{:?}", a_q);
    let triplet8 = & mut Triplet::new(8, 8, 8.to_string());
    a_q.enqueue(triplet8);
    println!("{:?}", a_q);
    let triplet9 = & mut Triplet::new(9, 9, 9.to_string());
    a_q.enqueue(triplet9);
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.dequeue());
    }
    println!("{:?}", a_q.dequeue());
}
