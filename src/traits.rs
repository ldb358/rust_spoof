
/*
 * This trait can be used to allow for multiple elements
 * to be chained together allowing for full stack packet
 * parsing/construction
 */
pub enum NetworkLayer {
    IPv4,
    Other
}

pub enum TransportLayer {
    TCP,
    UDP,
    Other
}

pub trait Chainable {
    fn get_end(&self) -> usize;
    fn to_binary(&self, higher_levels: &mut Vec<u8>); 
}

pub trait Datalink {
	fn get_next_level(&self) -> NetworkLayer;	
}

pub trait Network {
    fn get_next_level(&self) -> TransportLayer; 
}
