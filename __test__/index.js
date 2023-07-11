const { serializeResourcePacksInfo, deserializeResourcePacksInfo } = require('../index.js')

const buffer = serializeResourcePacksInfo({
  hasScripts: false,
  mustAccept: true,
  resourcePacks: ['hello_world'],
  behaviourPacks: ['foo_bar_baz']
})
const buf = Buffer.from(buffer)
console.log(buf)

const deserialized = deserializeResourcePacksInfo(Array.from(buf))
console.log(deserialized)