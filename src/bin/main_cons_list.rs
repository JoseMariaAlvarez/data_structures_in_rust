use data_structures_in_rust::cons_list::List;

fn list_sum_recursive(list: &List<i32>) -> i32 {
    match list.head() {
        Some(value) => value + list_sum_recursive(&list.tail()),
        None => 0,
    }
}

fn list_sum(list: &List<i32>) -> i32 {
    let mut sum = 0;
    let mut current = list.clone();
    while let Some(value) = current.head() {
        sum += value;
        current = current.tail().clone();
    }
    sum
}

fn main(){
    
}

#[cfg(test)]
mod test {
    use data_structures_in_rust::cons_list::List;
    use super::list_sum;
    use super::list_sum_recursive;

    #[test]
    fn basics() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        
        assert_eq!(list.head(), None);

    }

    #[test]
    fn test_cons() {
        let cons_list = List::cons(1, &List::cons(2, &List::cons(3, &List::new())));
        assert_eq!(6, list_sum_recursive(&cons_list));
        assert_eq!(6, list_sum(&cons_list));
        assert_eq!(cons_list.head(), Some(&1));
        assert_eq!(cons_list.tail().head(), Some(&2));
        assert_eq!(cons_list.head(), Some(&1));
        let cons_list = cons_list.tail();
        assert_eq!(cons_list.head(), Some(&2));
        let cons_list = cons_list.tail();
        assert_eq!(cons_list.head(), Some(&3));
        let cons_list = cons_list.tail();
        assert_eq!(cons_list.head(), None);
    }
}