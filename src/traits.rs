
/*
 * This trait can be used to allow for multiple elements
 * to be chained together allowing for full stack packet
 * parsing/construction
 */
pub trait Chainable {
    fn get_end(&self) -> usize;
}
