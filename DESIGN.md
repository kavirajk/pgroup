# Pgroup
A Peer Group library used to build cluster of process (both local and remote). It is based on [http://www.cs.cornell.edu/Info/Projects/Spinglass/public_pdfs/SWIM.pdf](SWIM) protocol

# Components

## Failure Detectors
Send Ping or PingReq and mark if node failed to respond for neither of those messages. Ask Dissemination component to send the message about marked node.

## Dissemination
Two sub components.
1. Multicast Sender - Take a message from the queue and do IP multicast.
2. Multicast Reciver - Listens for IP multicast message, and change local memember state accordingly.

# State

Maintain two list of peers.

1. Active peers

2. Failed peers

If Failure Detectors marks some peers to be failed, it moves it from active to failed, and ask Dissemination to announce via IP multicast.

# v0.01 (Failure)
- IP multicast is so hard to make it work on GCP cloud(the one I use to test). Its is disabled by default. infact not even supported. https://cloud.google.com/vpc/docs/vpc#specifications

## Membership management only via IP multicast
- Each peers have both IP multicast sender and reciver.
- No failure detectors. When each peer is up, it starts announcing itself to everyone on the network via IP multicast via sender.
- Each peers listening on IP multicast when got `NewPeer` msg that is not in its active peers, just add that peers to its active list.

### Problems:
- Q: How does newly joined peers find about all other old peers already in the network?
  A: How about every listener to the multicast of newly joined peer to that peer via direct UDP message?

### NOTES
Q: How does multicast UDP works? particulary, how does datagram send and received in muticast
https://www.tldp.org/HOWTO/Multicast-HOWTO-2.html

1. Send sends datagram to (MulticastIP:Port), a socket. - That's it! NOTE: TTL matters (ttl - 0 (only localhost), ttl-1, (only subnet))
2. Reciver, ask kernel to join the multicast group, and should 'bind' to port, to which datagram is about to be sent via sender.
NOTE(for reciver, just listening on IP with any port is not sufficient, as UDP works with port level. It needs to know port to multiplex or demultiplex the datagrams)

# v0.02 (Probe single node for failure)

Basically, doing single round of protocol period.

## Msg Types
- Ping
- Ack
- PingReq
- IndirectAck
- Alive
- Dead

## UDP sender and receiver
- Sender sends Ping periodically
- Reciver sends ACK back when recvies Ping.

## No Membership management
- Meaning every peer will start with its own seed peers and just detect failures of those peers only.
- No peer is aware of other peers newly joining or voluntary leaving.

## Rough Design
- Start each peer with its own seed peers
- Schedule Probe thread for every protocol period.
  - Send and wait for direct acks
  - Send and wait for indirect acks (if direct ack fails)
- UDP listener for packet.
  - Ping - HandlePing -> which sends acks -> done
  - Ack - HandleAck -> send to ack channel may be? with specific seqno.
  - PingReq - HandlePingReq -> send pings to peers requested -> wait for ack via ack channel -> respond with indirectAck
  - IndirectAck - HandleIndirectAck -> send to indirect channel may be? with specific seqno.
