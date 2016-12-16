Add a parser that builds the struct:
```
layers {
	datalink: Datalink type
	network: network layer type
	transport: transport layer type
}
```
The parser then reads the packet and fills in as many layers as possible or up to a specified layer.


Have each layer define an enum of higher level protocols that it can disguish for example datalink could have:

```
enum TransportProtocols {
	TCP,
	UDP,
	ICMP
}
```

and all Datalink layer implementations would have to implement

```
trait Datalink {
	pub get_next_level() -> Result<TransportProtocols, ()>;	
}
trait Chainable { 
    pub get_offset() -> usize;
}
```

The our matcher could do something like:
```
pub fn get_datalink(packet: Packet) -> Box<Datalink + Chainable>;
pub fn get_transport(packet: Packet, dl: &Box<Datalink + Chainable> ) -> Box< {
	let protocol: TransportProtocols = dl.get_next_level();
	match protocol {
		TransportProtocols::TCP => TCP::new(packet, dl.get_offset())
		// ...
		_ => {} // we don't know how to support this protocol
	}
}
// ect... for however many layers we can support.
```

This method has the advantage that we can:
1. Allows the parent layer to specify what sub layers it can identify. This is
useful because it allows for us to locate the Ethernet level logic of parsing 
the type field based on the Ethernet standard inside the Ethernet class leading
to cleaner and less repetitive parsing code.
2. It makes the client more readable since you can just match TCP instead of some
number. If we don't support the client wants they can still implement it the
'other' way.
