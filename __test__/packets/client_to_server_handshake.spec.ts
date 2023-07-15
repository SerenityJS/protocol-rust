import { ClientToServerHandshakePacket } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  // Create a new play status packet
  const packet = new ClientToServerHandshakePacket()

  // This packet is empty, so there's nothing to check
  t.pass()

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: ClientToServerHandshakePacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], ClientToServerHandshakePacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: ClientToServerHandshakePacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = ClientToServerHandshakePacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

