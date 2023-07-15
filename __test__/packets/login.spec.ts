import { LoginPacket, LoginToken } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  // Create login token data
  const identity = 'some.overly.large.jwt.token'
  const client = 'some_json_identity_chain'
  const tokenData = new LoginToken(identity, client)

  // Ensure login token data is assigned correctly
  t.is(tokenData.client, client)
  t.is(tokenData.identity, identity)

  // Create a new login packet
  const protocolVersion = 589
  const packet = new LoginPacket(protocolVersion, tokenData)

  // Do some checks to ensure everything matches
  t.is(packet.protocolVersion, protocolVersion)
  t.truthy(compareNativeObjects(packet.tokens, tokenData), 'Token data is not similar to the original object!')

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: LoginPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], LoginPacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: LoginPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = LoginPacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

