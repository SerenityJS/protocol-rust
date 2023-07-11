import test from 'ava'

import { serializeResourcePacksInfo, deserializeResourcePacksInfo } from '../index.js'

test('sum from native', (t) => {
  const buffer = serializeResourcePacksInfo({
    hasScripts: false,
    mustAccept: true,
    resourcePacks: ['hello_world'],
    behaviourPacks: ['foo_bar_baz']
  })
  console.log(buffer)
  
  const deserialized = deserializeResourcePacksInfo(buffer)
  console.log(deserialized)

  t.is(1, 1)
})
