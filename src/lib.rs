mod conv_chain;

pub use conv_chain::conv_chain as conv_chain_slow;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
