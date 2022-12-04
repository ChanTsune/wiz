use rand::prelude::SliceRandom;

const BASE_STR: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

pub fn ascii_string(length: usize) -> String {
    let mut rng = &mut rand::thread_rng();
    unsafe {
        String::from_utf8_unchecked(
            BASE_STR
                .as_bytes()
                .choose_multiple(&mut rng, length)
                .cloned()
                .collect(),
        )
    }
}
