use kid::Arena;

fn main() {
    let mut arena = Arena::new();
    let id1 = arena.insert("A");
    println!("inserted id1: index={}, gen={}", id1.index, id1.generation);

    let removed = arena.remove(id1).unwrap();
    println!("removed: {}", removed);

    let id2 = arena.insert("B");
    println!("inserted id2: index={}, gen={}", id2.index, id2.generation);

    println!("get(id1) -> {:?}", arena.get(id1));
    println!("get(id2) -> {:?}", arena.get(id2));
}
