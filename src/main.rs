mod utils;
use utils::stack::Stack;
use utils::queue::Queue;
use std::collections::HashMap; //Libreria para el uso de Hashmap es mejor usarla que implementarlo de 0

fn main() {
    //Stack
    let mut stack = Stack::new();

    stack.push("prueba Stack");
    stack.pop();
    stack.push("prueba2 Stack");
    stack.push("prueba Stack");


    println!("{:?}",stack.top.unwrap().data);

    //Queue

    let mut queue = Queue::new();

    // Add some items
    queue.enqueue("Prueba");
    queue.enqueue("valor2");
    queue.enqueue("3"); 

    
    println!("Antes de dequeue: {:?}",queue.peek());
    

    // Remove an item
    queue.dequeue();

    // Check results
    println!("Despues dequeue {:?}",queue.peek());

    //HashMap
    let mut oHashMap = HashMap::new();
    oHashMap.insert("llave1", "valor1");
    oHashMap.insert("llave2", "valor2");
    oHashMap.insert("llave3", "valor3");
    println!("Found word:  {:?}", Some(oHashMap.get("llave1") ));


   
}
