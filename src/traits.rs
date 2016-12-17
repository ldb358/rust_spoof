
/*
 * This trait can be used to allow for multiple elements
 * to be chained together allowing for full stack packet
 * parsing/construction
 */
pub enum NetworkLayer {
    IPv4,
    Other
}

pub trait Chainable {
    fn get_end(&self) -> usize;
}

pub trait Datalink {
	fn get_next_level(&self) -> NetworkLayer;	
}
