mod lists;

use crate::lists::LinkedList;

fn main() {

    let mut list = LinkedList::new(String::from("node_1"));

    list.append_end("node_2".to_string());
    list.append_start("some other text".to_string());

    list.append_start("node_1988".to_string());
    list.append_start("node_12".to_string());
    list.append_start("node_645".to_string());
    list.append_start("node_4".to_string());
    list.append_start("node_3".to_string());

    list.print_items();
    
    list.pop_head();    
    list.pop_head();    
    list.pop_end();    

    list.append_end("wa".to_string());
    list.print_items();
    
    list.pop_end();    
    list.print_items();
    
}
