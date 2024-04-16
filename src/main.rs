use std::time::Instant;

fn main() {
    _first_test();
    // _copy_from_slice_test_set();
    // _run_a_lot_forwards();
    // print!("\n");
    // _run_a_lot_backwards();
}

//Returns:
// At 10000: Speed up is 25.3137x faster
// At 100000: Speed up is 6.6592813x faster
// At 1000000: Speed up is 1.7620945x faster
// At 10000000: Speed up is 1.8490372x faster
// At 100000000: Speed up is 1.7937524x faster
fn _run_a_lot_forwards() {
    let mut i = 10_000;
    while i <= 100_000_000 {
        let mut sum: f32 = 0.0;
        for _j in 0..10 {
            sum += _copy_from_slice_test(i);
        }
        println!("At {}: Speed up is {}x faster",i, sum/10.0);
        i *= 10;
    }
}

//Returns:
// At 100000000: Speed up is 1.7929674x faster
// At 10000000: Speed up is 1.9418052x faster
// At 1000000: Speed up is 1.6404991x faster
// At 100000: Speed up is 4.636007x faster
// At 10000: Speed up is 25.574482x faster

fn _run_a_lot_backwards() {
    let mut i = 100_000_000;
    while i >= 10_000 {
        let mut sum: f32 = 0.0;
        for _j in 0..10 {
            sum += _copy_from_slice_test(i);
        }
        println!("At {}: Speed up is {}x faster",i, sum/10.0);
        i /= 10;
    }
}


fn _first_test() {
    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    for i in big_vec {
        if i < 0 {
            println!("this never prints");
        }
    }
    let z = now.elapsed().as_secs_f32();

    println!("First loop took {}s", z);
     
    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.iter().for_each(|i| {
        if *i < 0 {
            println!("this never prints");
        }
    });
    let a = now.elapsed().as_secs_f32();
    println!("Second loop took {}s", a);
    println!("Second loop is {}x faster",z/a);

    let big_vec = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec.into_iter().for_each(|i| {
        if i < 0 {
            println!("this never prints");
        }
    });

    println!("Third loop took {}s\n", now.elapsed().as_secs_f32());

}

//for some reason, when using a variable for size the 
// For 100000000: Fast copy is 1.9046377x faster
// For 10000000: Fast copy is 1.8610786x faster
// For 1000000: Fast copy is 1.8175623x faster
// For 100000: Fast copy is 12.967105x faster
// For 10000: Fast copy is 91.22222x faster

fn _copy_from_slice_test(size: usize) -> f32 {
    let big_vec_source: Vec<i32> = vec![0; size];
    let mut big_vec_target: Vec<i32> = Vec::<i32>::with_capacity(size);           
    let now = Instant::now();
    big_vec_source
    .into_iter()
    .for_each(|i| big_vec_target.push(i));
    let a = now.elapsed().as_secs_f32();
    // println!("Naive copy took {}s", a);

let big_vec_source: Vec<i32> = vec![0; size];
    let mut big_vec_target: Vec<i32> = vec![0; size];
    let now = Instant::now();
    big_vec_target.copy_from_slice(&big_vec_source);
    let b = now.elapsed().as_secs_f32();
    // println!("Fast copy took {}s", b);
    
    // println!("For {}: Fast copy is {}x faster",size, a/b);
    a/b
}

//Copy from slice works significantly faster on my machine then the example
// Naive copy took 0.0242707s
// Fast copy took 0.0000001s
// For 10000000: Fast copy is infx faster

fn _copy_from_slice_test_set() {
    let big_vec_source: Vec<i32> = vec![0; 10_000_000];
    let mut big_vec_target: Vec<i32> = Vec::<i32>::with_capacity(10_000_000);           
    let now = Instant::now();
    big_vec_source
        .into_iter()
        .for_each(|i| big_vec_target.push(i));
    let a = now.elapsed().as_secs_f32();
    println!("Naive copy took {}s", a);
    
    let big_vec_source: Vec<i32> = vec![0; 10_000_000];
    let mut big_vec_target: Vec<i32> = vec![0; 10_000_000];
    let now = Instant::now();
    big_vec_target.copy_from_slice(&big_vec_source);
    let b = now.elapsed().as_secs_f32();
    println!("Fast copy took {}s", b);

    println!("For {}: Fast copy is {}x faster",10_000_000, a/b);
}