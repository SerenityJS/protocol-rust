import { ServerToClientHandshakePacket } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  const token = "some_random_string"

  // Create a new play status packet
  const packet = new ServerToClientHandshakePacket(token)

  // Do some checks to ensure everything matches
  t.is(packet.token, token)

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: ServerToClientHandshakePacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], ServerToClientHandshakePacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: ServerToClientHandshakePacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = ServerToClientHandshakePacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

