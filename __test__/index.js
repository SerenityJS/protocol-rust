const { LoginPacket, LoginToken } = require('../index')

const tokens = new LoginToken('testtttttttttt', 'testttttttttttttttttttttttttttt')
const pak = new LoginPacket(589, tokens)

const ser = pak.serialize()
console.log(ser)

const destruct = (obj) => {
  const properties = new Set()
  let currentObj = obj
  do {
    Object.getOwnPropertyNames(currentObj).map((item) => properties.add(item))
  } while ((currentObj = Object.getPrototypeOf(currentObj)))
  return [...properties.keys()]
}

const des = LoginPacket.deserialize(ser)
console.log(destruct(des))
