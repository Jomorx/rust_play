use crate::lib_list::list::Node;

#[test]
fn test_push(){
    let mut list = Node::new(5);
    list.push(11);
    assert_eq!(list.to_string(),"[5,11]")
}

#[test]
fn test_pop(){
    let mut list = Node::new(5);
    list.pop();
    assert_eq!(list.to_string(),"[]")
}