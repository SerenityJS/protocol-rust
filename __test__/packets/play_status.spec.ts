import { PlayStatus, PlayStatusPacket } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  // Not gonna test every play status, just the first one
  const playStatus = PlayStatus.LoginSuccess

  // Create a new play status packet
  const packet = new PlayStatusPacket(playStatus)

  // Do some checks to ensure everything matches
  t.is(packet.status, playStatus)

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: PlayStatusPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], PlayStatusPacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: PlayStatusPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = PlayStatusPacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

