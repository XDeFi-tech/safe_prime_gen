use wasm_bindgen::prelude::*;
use glass_pumpkin::safe_prime;

#[wasm_bindgen]
pub fn safe_prime_gen(len: usize) -> String {
    return safe_prime::new(len).unwrap().to_string();
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
