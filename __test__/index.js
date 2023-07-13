const { LoginPacket, LoginToken } = require('../index')

structuredClone()

const tokens = new LoginToken('testtttttttttt', 'testttttttttttttttttttttttttttt')
const pak = new LoginPacket(589, tokens)

const ser = pak.serialize()
console.log(ser)

