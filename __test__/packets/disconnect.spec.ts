import { DisconnectPacket } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  // Values needed for the constructor
  const hideScreen = false
  const message = "get booted from da server mwahahaha"

  // Create a new play status packet
  const packet = new DisconnectPacket(hideScreen, message)

  // Do some checks to ensure everything matches
  t.is(packet.hideDisconnectScreen, hideScreen)
  t.is(packet.message, message)

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: DisconnectPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], DisconnectPacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: DisconnectPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = DisconnectPacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

