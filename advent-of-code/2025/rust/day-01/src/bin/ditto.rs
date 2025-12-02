use ditto::List;

fn main() {
    // Create a List CRDT. The site that creates the CRDT
    // is automatically assigned id 1.
    let mut list1 = List::from(vec![68,30]);

    // dbg!(list1);

    // Send the list's state over a network to a second site with id 2.
    let encoded_state = serde_json::to_string(&list1.state()).unwrap();
    let decoded_state = serde_json::from_str(&encoded_state).unwrap();
    let mut list2 = List::from_state(decoded_state, Some(2)).unwrap();

    // Edit the list concurrently at both the first and second site.
    // Whenever you edit a CRDT, you receive an op that can be sent
    // to other sites.
    let op1 = list2.push(48).unwrap();
    list1.execute_op(op1);



    let op2 = list1.push(5).unwrap();
    list2.execute_op(op2);

    let op3 = list2.push(60).unwrap();
    list1.execute_op(op3);

    // let op4 = list1.merge(List::from(vec![55,1,99]));
    let op4 = list1.push(55).unwrap();
    list2.execute_op(op4);
    let op5 = list1.push(1).unwrap();
    list2.execute_op(op5);
    let op6 = list1.push(99).unwrap();
    list2.execute_op(op6);

    let op7 = list2.push(14).unwrap();
    list1.execute_op(op7);

    let op8 = list1.push(82).unwrap();
    list2.execute_op(op8);
    
    dbg!(&list1, &list2);

    assert_eq!(list1.state(), list2.state());
    assert_eq!(list1.local_value(), vec![68,30,48,5,60,55,1,99,14,82]);
}