use std::collections::LinkedList;

fn merge_2countries_top_list(l1: &mut LinkedList<i32>, l2: &mut LinkedList<i32>) -> LinkedList<i32> {
    let mut result: LinkedList<i32> = LinkedList::new();

    while !l1.is_empty() && !l2.is_empty() {
        if l1.front() <= l2.front() {
            result.push_back(l1.pop_front().unwrap());
        } else {
            result.push_back(l2.pop_front().unwrap());            
        }
    }
    if l1.is_empty() {
        result.append(l2);
    } else {
        result.append(l1)
    }

    result
}

pub fn merge_kcountries_top_list(lists: &Vec<LinkedList<i32>>) -> Option<LinkedList<i32>> {
    if lists.len() == 0 {
        return None;
    } else {
        let mut result = lists[0].clone();
        for idx in 1..lists.len() {
            let mut list2 = lists[idx].clone();
            result = merge_2countries_top_list(&mut result, &mut list2);
        }

        Some(result)
    }
}



#[cfg(test)]
mod test {

    use std::collections::LinkedList;
    use crate::lib::fetch_top_movies::merge_kcountries_top_list;

    #[test]
    fn fetch_top_movies() {
        let mut a: LinkedList<i32> = LinkedList::new();

        a.push_back(11);
        a.push_back(41);
        a.push_back(51);
    
        let mut b: LinkedList<i32> = LinkedList::new();
        b.push_back(21);
        b.push_back(23);
        b.push_back(42);
    
        let mut c: LinkedList<i32> = LinkedList::new();
        c.push_back(25);
        c.push_back(56);
        c.push_back(66);
        c.push_back(72);
    
        let mut list1: Vec<LinkedList<i32>> = Vec::new();
        list1.push(a);
        list1.push(b);
        list1.push(c);

        let mut expected: LinkedList<i32> = LinkedList::new();
        expected.push_back(11);
        expected.push_back(21);
        expected.push_back(23);
        expected.push_back(25);
        expected.push_back(41);
        expected.push_back(42);
        expected.push_back(51);
        expected.push_back(56);
        expected.push_back(66);
        expected.push_back(72);

        let result = merge_kcountries_top_list(&list1).unwrap();
        println!("expected: {:?}", expected);
        println!("results: {:?}", result);
        assert_eq!(expected, result);
    }
}