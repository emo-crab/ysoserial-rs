use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

pub(crate) fn get_int_rand(len: usize) -> String {
    let rand_string: Vec<String> = thread_rng()
        .sample_iter(&Uniform::new(0, 9))
        .take(len)
        .map(|x| x.to_string())
        .collect();
    return rand_string.join("");
}