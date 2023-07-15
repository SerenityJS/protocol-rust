/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export const enum PlayStatus {
  LoginSuccess = 0,
  FailedClient = 1,
  FailedSpawn = 2,
  PlayerSpawn = 3,
  FailedInvalidTenant = 4,
  FailedVanillaEdu = 5,
  FailedEduVanilla = 6,
  FailedServerFull = 7,
  FailedEditorVanillaMismatch = 8,
  FailedVanillaEditorMismatch = 9
}
export function getPacketId(data: Buffer): number
export class LoginPacket {
  protocolVersion: number
  tokens: LoginToken
  constructor(protocolVersion: number, tokens: LoginToken)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): LoginPacket
}
export class LoginToken {
  identity: string
  client: string
  constructor(identity: string, client: string)
}
export class PlayStatusPacket {
  status: PlayStatus
  constructor(status: PlayStatus)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): PlayStatusPacket
}
export class ServerToClientHandshakePacket {
  token: string
  constructor(token: string)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): ServerToClientHandshakePacket
}
export class ClientToServerHandshakePacket {
  
  constructor()
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): ClientToServerHandshakePacket
}
export class DisconnectPacket {
  hideDisconnectScreen: boolean
  message: string
  constructor(hideDisconnectScreen: boolean, message: string)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): DisconnectPacket
}
export class ResourcePacksInfoPacket {
  mustAccept: boolean
  hasScripts: boolean
  forceServerPacks: boolean
  behaviourPacks: Array<BehaviourPackInfo>
  resourcePacks: Array<ResourcePackInfo>
  constructor(mustAccept: boolean, hasScripts: boolean, forceServerPacks: boolean, behaviourPacks: Array<BehaviourPackInfo>, resourcePacks: Array<ResourcePackInfo>)
  static id(): number
  serialize(): Buffer
  static deserialize(buf: Buffer): ResourcePacksInfoPacket
}
export class BehaviourPackInfo {
  uuid: string
  version: string
  size: bigint
  contentKey: string
  subPackName: string
  contentIdentity: string
  hasScripts: boolean
  constructor(uuid: string, version: string, size: bigint, contentKey: string, subPackName: string, contentIdentity: string, hasScripts: boolean)
}
export class ResourcePackInfo {
  uuid: string
  version: string
  size: bigint
  contentKey: string
  subPackName: string
  contentIdentity: string
  hasScripts: boolean
  rtxEnabled: boolean
  constructor(uuid: string, version: string, size: bigint, contentKey: string, subPackName: string, contentIdentity: string, hasScripts: boolean, rtxEnabled: boolean)
}
