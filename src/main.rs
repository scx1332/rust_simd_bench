use packed_simd::*;
use std::time::Instant;

const INTERNAL_LOOPS: i32 = 100;

fn test_4(vec: &mut [f32]) {
    const VECTOR_SIZE: usize = 4;
    type VectorType = f32x4;

    if vec.len() % VECTOR_SIZE != 0 {
        panic!("Vector size must be a multiple of {}", VECTOR_SIZE);
    };
    let mut idx = 0;
    loop {
        let mut f = VectorType::from_slice_aligned(&vec[idx..idx + VECTOR_SIZE]);
        for _times in 0..INTERNAL_LOOPS {
            f = (f + f) / (f - VectorType::splat(1.0));
        }
        f.write_to_slice_aligned(&mut vec[idx..idx + VECTOR_SIZE]);
        idx += VECTOR_SIZE;
        if idx >= vec.len() {
            break;
        }
    }
}

fn main() {
    let mut vec_input = Vec::new();
    for _i in 0..100000 {
        vec_input.push(rand::random::<f32>());
    }

    let mut vec = vec_input.clone();

    let internal_loops = 100;
    //process scalar
    let time_start = Instant::now();
    for i in 0..vec.len() {
        for _j in 0..internal_loops {
            vec[i] = (vec[i] + vec[i]) / (vec[i] - 1.0f32);
        }
    }
    let time_end = Instant::now();

    println!("scalar: {:?}", time_end - time_start);
    println!("Result {:?}", vec.iter().sum::<f32>());

    let mut vec = vec_input.clone();
    test_4(&mut vec);

    let time_start = Instant::now();
    let time_end = Instant::now();
    println!("simd: {:?}", time_end - time_start);
    println!("Result {:?}", vec.iter().sum::<f32>());

    //println!("{:?}", a + b);
}
