mod lib_list;

use lib_list::list::Node;
fn main() {
    let mut list = Node::new(2);
    list.push(5);
    list.push(1);
    list.push(2);
    println!("{}",list);//[2,5,1,2]
    list.pop();
    println!("{}",list);//[2,5,1]
    list[1] = 1;
    println!("{}",list);//[2,5,1]

}