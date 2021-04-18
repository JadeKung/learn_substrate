fn summation_of_sets(vec:&[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for number in vec.iter(){
         match sum.checked_add(*number) {
             None => return None,
             Some(i) => sum = sum + i,
         };
    }
    Some(sum)
}

fn main() {
    let set_a = [1,2,3,4,5];
    let sum_seta = summation_of_sets(&set_a);
    println!("Summation of seta is {:?}",sum_seta);

    let set_b = [1,2,3,4294967295];
    let sum_setb = summation_of_sets(&set_b);
    println!("Summation of seta is {:?}",sum_setb);
}
