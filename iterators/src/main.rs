//Documentation comments use three slashes, ///
/// This goes to HTML documentation
fn main() {

    let array = vec![1,2,3];
    let arr_iter = array.iter();

    for val in arr_iter {
        println!("{}", val);
     }

    let array2 = vec![1,2,3];
    //Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state 
    //that the iterator uses to keep track of where it is in the sequence. 
    //In other words, this code consumes, or uses up, the iterator. 
    let mut arr2_iter = array2.iter();

    // we have to give a type to total
    // let total :i32 = arr2_iter.sum(); 
    // println!("sum: {:?}", total); 
    
    let val = arr2_iter.next();
    //val is some(), need to unwrap
    println!("{:?}", val.unwrap());

    //we have to give a type to total
    //it starts fom arr index 1, so sum comes 5. so at takes the iterator where it breaks
    let total :i32 = arr2_iter.sum(); 
    println!("sum: {:?}", total);


    //Other methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different kinds of iterators.
    let v1: Vec<i32> = vec![1, 2, 3];
    //r. The closure here creates a new iterator in which each item from the vector has been incremented by 1
    // because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();


}
