pub mod algo;
pub mod dict;
pub mod gear;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests;
