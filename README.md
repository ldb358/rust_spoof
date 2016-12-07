Add a parser that builds the struct:

layers {
	datalink: Datalink type
	network: network layer type
	transport: transport layer type
}


The parser then reads the packet and fills in as many layers as possible or up to a specified layer.
