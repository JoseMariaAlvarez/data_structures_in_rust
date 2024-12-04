use data_structures_in_rust::array_queue_box_option::ArrayQueueBoxOption;

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
fn main(){
    
        let mut a_q = ArrayQueueBoxOption::<Triplet>::new();
        println!("{:?}", a_q);
        for i in 0..5 {
            a_q.enqueue(Triplet::new(i, i, i.to_string()));
        }
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(5, 5, 5.to_string()));
        println!("{:?}", a_q);
        println!("{:?}", a_q.peek());
        println!("{:?}", a_q.dequeue());
        println!("{:?}", a_q.peek());
        a_q.dequeue();
        a_q.dequeue();
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(6, 6, 6.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(7, 7, 7.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(8, 8, 8.to_string()));
        println!("{:?}", a_q);
        a_q.enqueue(Triplet::new(9, 9, 9.to_string()));
        println!("{:?}", a_q);
        println!("{:?}", a_q.is_empty());
        while !a_q.is_empty() {
            println!("{:?}", a_q.dequeue());
        }
        println!("{:?}", a_q.dequeue());
}