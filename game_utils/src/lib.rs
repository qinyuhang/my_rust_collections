#[cfg(not(target_family = "wasm"))]
use rand::{thread_rng, Rng};
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[cfg(not(target_family = "wasm"))]
pub fn log(s: &str) {
    println!("{}", s);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

#[cfg(not(target_family = "wasm"))]
pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    rng.gen_range(min..max)
}

#[cfg(target_family = "wasm")]
pub fn random_range(min: usize, max: usize) -> usize {
    (random() * (max - min) as f64).floor() as usize + min
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        assert_eq!(random_range(0, 1), 0);
    }
}
