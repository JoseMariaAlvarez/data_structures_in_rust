fn main(){
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.pop();
    println!("{:?}", v);
    println!("{:?}", v.is_empty());
    println!("{:?}", v2.len());

    let j = v.get(2);

    println!("{:?}", j);

    let i = v.get_mut(2);

    if let Some(valor) = i {
        *valor = 10;
    }

    println!("{:?}", v);
}