import { LoginPacket, getPacketId } from '../index.js' 
import test from 'ava'

// Test Procedural Macro For Packet Ids
test('id proc_macro', t => {
  t.is(LoginPacket.id(), 0x01)
})

test('id unwrapper', t => {
  const buffer = Buffer.from([0x01, 0x02, 0x03])
  const pak_id = getPacketId(buffer)

  t.is(pak_id, 0x01)
})
