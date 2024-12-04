use data_structures_in_rust::ok_stack_generic::List;
use data_structures_in_rust::ok_stack_generic::Node;

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
}fn main(){

//    let mut a_q: List<Triplet> = List::new();
    let mut a_q: List<Triplet> = List{ head : None};
    a_q = List{ head : Some(Box::new(Node{ elem: Triplet::new(0, 0, "0".to_string()), next: None}))};
    println!("{:?}", a_q);
    for i in 1..3 {
        a_q.push(Triplet::new(i, i, i.to_string()));
    }
    a_q.push_last(Triplet::new(15, 15, 15.to_string()));
    a_q.traverse_queue();
    println!("{:?}", a_q);
    a_q.traverse_queue();
    let trip = a_q.pop();
    println!("{:?}", a_q);
    println!("{:?}", a_q.peek());
    println!("{:?}", a_q.pop());
    println!("{:?}", a_q.peek());
    let trip =a_q.pop();
    let trip =a_q.pop();
    println!("{:?}", a_q);
    a_q.push(Triplet::new(6, 6, 6.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(7, 7, 7.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(8, 8, 8.to_string()));
    println!("{:?}", a_q);
    a_q.push(Triplet::new(9, 9, 9.to_string()));
    println!("{:?}", a_q);
    println!("{:?}", a_q.is_empty());
    while !a_q.is_empty() {
        println!("{:?}", a_q.pop());
    }
    println!("{:?}", a_q.pop());

}

#[cfg(test)]      
mod test {
    use data_structures_in_rust::ok_stack_generic::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
