use rand::Rng;

pub fn random_f32_between(a: f32, b: f32) -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(a..=b);
}

pub fn random_usize_between(a: usize, b: usize) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(a..=b);
}
