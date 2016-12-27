Add the ability to write packets back to a wire.

This will require three new pieces:

First a `fn to_binary(&self, Vec<u8> higher_levels) -> Vector<u8>` that 
returns a vector containing the binary data for the current layer and all 
higher layers(via the vec arg). This should be added to the chainable trait.

Then a function `fn build(Vec<Box<Chainable>> layers) -> Vec<u8>` where 
layers[0] is the lowest level for example data link and layers[n] could be all
the way up to application layer.

The last part will be `fn send(Vec<u8> data) -> bool` which takes a vector of
bytes and turns it into a Pcap packet and then sends it. The function returns
whether sending the packet was successful(just putting it on the wire not if
the packet was received.)

