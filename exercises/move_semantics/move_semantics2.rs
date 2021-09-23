// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::new();

    // a borrow
    let mut vec1 = fill_vec(&vec0);
    
    // generating entirely new vector and passing to original fill_vect
    let mut vec2 = vec0.clone();
    let mut vec3 = fill_vec_original(vec2);

    // we can't do this because vec2 is no longer valid -- it has been moved
    // println!("{} has length {} content `{:?}`", "vec2", vec2.len(), vec2);
    
    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    println!("{} has length {} content `{:?}`", "vec3", vec3.len(), vec3);
}



fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_owned();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec_original(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
