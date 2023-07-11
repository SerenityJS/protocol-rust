const { serializeResourcePacksInfo, deserializeResourcePacksInfo } = require('../index.js')

let d = Date.now()
const buffer = serializeResourcePacksInfo({
  hasScripts: false,
  mustAccept: true,
  resourcePacks: [
    {
      'uuid': '59addd86-1fee-11ee-be56-0242ac120002',
      'size': 10000n,
      'version': '1.0.0',
      'subPackName': 'foo bar baz',
      'contentKey': 'foo_bar_baz',
      'contentIdentity': 'foo_bar_baz',
      'hasScripts': false,
      'rtxEnabled': false,
    }  
  ],
  behaviourPacks: []
})
const buf = Buffer.from(buffer)
console.log('Took ' + (Date.now() - d) + 'ms to serialize')
console.log(buf)

d = Date.now()
const des = deserializeResourcePacksInfo(Array.from(buf))
console.log('Took ' + (Date.now() - d) + 'ms to deserialize')
console.log(des)
