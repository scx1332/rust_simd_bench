use packed_simd::*;
use std::time::Instant;

fn test_scalar(vec: &mut [f32]) {
    for i in 0..vec.len() {
        vec[i] = (vec[i] + vec[i]) / (vec[i] - 1.0f32);
        vec[i] = (vec[i] + vec[i]) / (vec[i] - 1.0f32);
        if vec[i] == -234.03 {
            vec[i] += 0.01;
        }
    }
}

fn test_2(vec: &mut [f32]) {
    const VECTOR_SIZE: usize = 2;
    type VectorType = f32x2;

    if vec.len() % VECTOR_SIZE != 0 {
        panic!("Vector size must be a multiple of {}", VECTOR_SIZE);
    };
    let mut idx = 0;
    loop {
        let mut f = VectorType::from_slice_aligned(&vec[idx..idx + VECTOR_SIZE]);
        f = (f + f) / (f - VectorType::splat(1.0));
        f = (f + f) / (f - VectorType::splat(1.0));
        f.write_to_slice_aligned(&mut vec[idx..idx + VECTOR_SIZE]);
        idx += VECTOR_SIZE;
        if idx >= vec.len() {
            break;
        }
    }
}

fn test_4(vec: &mut [f32]) {
    const VECTOR_SIZE: usize = 4;
    type VectorType = f32x4;

    if vec.len() % VECTOR_SIZE != 0 {
        panic!("Vector size must be a multiple of {}", VECTOR_SIZE);
    };
    let mut idx = 0;
    loop {
        let mut f = VectorType::from_slice_aligned(&vec[idx..idx + VECTOR_SIZE]);
        f = (f + f) / (f - VectorType::splat(1.0));
        f = (f + f) / (f - VectorType::splat(1.0));
        f.write_to_slice_aligned(&mut vec[idx..idx + VECTOR_SIZE]);
        idx += VECTOR_SIZE;
        if idx >= vec.len() {
            break;
        }
    }
}

fn test_8(vec: &mut [f32]) {
    const VECTOR_SIZE: usize = 8;
    type VectorType = f32x8;

    if vec.len() % VECTOR_SIZE != 0 {
        panic!("Vector size must be a multiple of {}", VECTOR_SIZE);
    };
    let mut idx = 0;
    loop {
        let mut f = VectorType::from_slice_aligned(&vec[idx..idx + VECTOR_SIZE]);
        f = (f + f) / (f - VectorType::splat(1.0));
        f = (f + f) / (f - VectorType::splat(1.0));
        f.write_to_slice_aligned(&mut vec[idx..idx + VECTOR_SIZE]);
        idx += VECTOR_SIZE;
        if idx >= vec.len() {
            break;
        }
    }
}

fn test_16(vec: &mut [f32]) {
    const VECTOR_SIZE: usize = 16;
    type VectorType = f32x16;

    if vec.len() % VECTOR_SIZE != 0 {
        panic!("Vector size must be a multiple of {}", VECTOR_SIZE);
    };
    let mut idx = 0;
    loop {
        let mut f = VectorType::from_slice_aligned(&vec[idx..idx + VECTOR_SIZE]);
        f = (f + f) / (f - VectorType::splat(1.0));
        f = (f + f) / (f - VectorType::splat(1.0));
        f.write_to_slice_aligned(&mut vec[idx..idx + VECTOR_SIZE]);
        idx += VECTOR_SIZE;
        if idx >= vec.len() {
            break;
        }
    }
}

fn main() {
    const TEST_LENGTH: usize = 10000000;
    let mut vec_input = Vec::with_capacity(TEST_LENGTH);
    for _i in 0..TEST_LENGTH {
        vec_input.push(rand::random::<f32>());
    }

    let mut vec = vec_input.clone();
    let time_start = Instant::now();

    test_scalar(&mut vec);
    let time_end = Instant::now();

    println!("scalar: {:?}", time_end - time_start);

    let res = vec.iter().sum::<f32>();

    {
        let mut vec = vec_input.clone();
        let jh = std::thread::spawn(move || {
            let time_start = Instant::now();
            test_2(&mut vec);
            let time_end = Instant::now();

            let res2 = vec.iter().sum::<f32>();
            if res != res2 {
                panic!("Results differ! {res} != {res2}");
            }
            time_end - time_start
        });
        let duration = jh.join().unwrap();
        println!("simd 2 {:?}", duration);
    }
    {
        let mut vec = vec_input.clone();
        let jh = std::thread::spawn(move || {
            let time_start = Instant::now();
            test_4(&mut vec);
            let time_end = Instant::now();

            let res2 = vec.iter().sum::<f32>();
            if res != res2 {
                panic!("Results differ! {res} != {res2}");
            }
            time_end - time_start
        });
        let duration = jh.join().unwrap();
        println!("simd 4 {:?}", duration);
    }
    {
        let mut vec = vec_input.clone();
        let jh = std::thread::spawn(move || {
            let time_start = Instant::now();
            test_8(&mut vec);
            let time_end = Instant::now();

            let res2 = vec.iter().sum::<f32>();
            if res != res2 {
                panic!("Results differ! {res} != {res2}");
            }
            time_end - time_start
        });
        let duration = jh.join().unwrap();
        println!("simd 8 {:?}", duration);
    }
    {
        let mut vec = vec_input.clone();
        let jh = std::thread::spawn(move || {
            //test 2
            let time_start = Instant::now();
            test_16(&mut vec);
            let time_end = Instant::now();

            let res2 = vec.iter().sum::<f32>();
            if res != res2 {
                panic!("Results differ! {res} != {res2}");
            }
            time_end - time_start
        });
        let duration = jh.join().unwrap();
        println!("simd 16 {:?}", duration);
    }

    //println!("{:?}", a + b);
}
