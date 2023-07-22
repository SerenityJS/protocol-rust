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
export const enum GameMode {
  Survival = 0,
  Creative = 1,
  Adventure = 2,
  SurvivalSpectator = 3,
  CreativeSpectator = 4,
  Fallback = 5,
  Spectator = 6
}
export const enum Dimension {
  Overworld = 0,
  Nether = 1,
  End = 2
}
export const enum PermissionLevel {
  Visitor = 0,
  Member = 1,
  Operator = 2,
  Custom = 3
}
export const enum ChatRestrictionLevel {
  None = 0,
  Dropped = 1,
  Disabled = 2
}
export const enum MovementAuthority {
  Client = 0,
  Server = 1,
  ServerWithRewind = 2
}
export const enum CompressionAlgorithm {
  Deflate = 0,
  Snappy = 1,
  None = 2
}
export function getPacketId(data: Buffer): number
export function framePackets(packets: Array<Buffer>): Buffer
export function unframePackets(data: Buffer): Array<Buffer>
export function makeMotd(motd: string, protocolVersion: number, version: string, currentPlayers: number, maxPlayers: number, serverId: string, worldName: string, gamemode: string, gamemodeId: number, portv4: number, portv6: number): Buffer
export type Vec3f = Vec3F
export class Vec3F {
  x: number
  y: number
  z: number
  constructor(x: number, y: number, z: number)
}
export type Vec2f = Vec2F
export class Vec2F {
  x: number
  y: number
}
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
export class ResourcePackStackPacket {
  mustAccept: boolean
  behaviourPacks: Array<BehaviourPackIdVersion>
  resourcePacks: Array<ResourcePackIdVersion>
  gameVersion: string
  experiments: Array<Experiment>
  experimentsPreviouslyUsed: boolean
  constructor(mustAccept: boolean, behaviourPacks: Array<BehaviourPackIdVersion>, resourcePacks: Array<ResourcePackIdVersion>, gameVersion: string, experiments: Array<Experiment>, experimentsPreviouslyUsed: boolean)
  static id(): number
  serialize(): Buffer
  static deserialize(buf: Buffer): ResourcePackStackPacket
}
export class BehaviourPackIdVersion {
  uuid: string
  version: string
  name: string
  constructor(uuid: string, version: string, name: string)
}
export class ResourcePackIdVersion {
  uuid: string
  version: string
  name: string
  constructor(uuid: string, version: string, name: string)
}
export class Experiment {
  name: string
  enabled: boolean
  constructor(name: string, enabled: boolean)
}
export class StartGamePacket {
  entityId: number
  runtimeEntityId: number
  playerGamemode: GameMode
  playerPosition: Vec3F
  playerRotation: Vec2F
  seed: bigint
  biomeType: number
  biomeName: string
  dimension: Dimension
  generator: number
  worldGamemode: GameMode
  difficulty: number
  spawnPosition: BlockCoordinates
  achievementsDisabled: boolean
  editorWorld: boolean
  createdInEditor: boolean
  exportedFromEditor: boolean
  dayCycleStopTime: number
  eduOffer: number
  eduFeaturesEnabled: boolean
  eduProductUuid: string
  rainLevel: number
  lightningLevel: number
  hasConfirmedPlatformLockedContent: boolean
  isMultiplayer: boolean
  broadcastToLan: boolean
  xboxLiveBroadcastMode: number
  platformBroadcastMode: number
  enableCommands: boolean
  isTexturepacksRequired: boolean
  gameRules: Array<GameRule>
  experiments: Array<Experiment>
  experimentsPreviouslyUsed: boolean
  bonusChest: boolean
  mapEnabled: boolean
  permissionLevel: PermissionLevel
  serverChunkTickRange: number
  hasLockedBehaviorPack: boolean
  isFromLockedWorldTemplate: boolean
  msaGamertagsOnly: boolean
  isFromWorldTemplate: boolean
  onlySpawnV1Villagers: boolean
  personaDisabled: boolean
  customSkinsDisabled: boolean
  emoteChatMuted: boolean
  gameVersion: string
  limitedWorldWidth: number
  limitedWorldLength: number
  isNewNether: boolean
  eduResourceUri: EducationSharedResourceURI
  experimentalGameplayOverride: boolean
  chatRestrictionLevel: ChatRestrictionLevel
  disabledPlayerInteractions: boolean
  levelId: string
  worldName: string
  premiumWorldTemplateId: string
  isTrial: boolean
  movementAuthority: MovementAuthority
  rewindHistorySize: number
  serverAuthoritativeBlockBreaking: boolean
  currentTick: number
  enchantmentSeed: number
  blockProperties: Array<BlockProperty>
  itemStates: Array<ItemState>
  multiplayerCorrelationId: string
  serverAuthoritativeInventory: boolean
  serverEngine: string
  propertyData: string
  blockPalletteChecksum: bigint
  worldTemplateId: string
  clientSideGeneration: boolean
  blockNetworkIdsAreHashes: boolean
  serverControlledSounds: boolean
  constructor(entityId: number, runtimeEntityId: number, playerGamemode: GameMode, playerPosition: Vec3F, playerRotation: Vec2F, seed: bigint, biomeType: number, biomeName: string, dimension: Dimension, generator: number, worldGamemode: GameMode, difficulty: number, spawnPosition: BlockCoordinates, achievementsDisabled: boolean, editorWorld: boolean, createdInEditor: boolean, exportedFromEditor: boolean, dayCycleStopTime: number, eduOffer: number, eduFeaturesEnabled: boolean, eduProductUuid: string, rainLevel: number, lightningLevel: number, hasConfirmedPlatformLockedContent: boolean, isMultiplayer: boolean, broadcastToLan: boolean, xboxLiveBroadcastMode: number, platformBroadcastMode: number, enableCommands: boolean, isTexturepacksRequired: boolean, gameRules: Array<GameRule>, experiments: Array<Experiment>, experimentsPreviouslyUsed: boolean, bonusChest: boolean, mapEnabled: boolean, permissionLevel: PermissionLevel, serverChunkTickRange: number, hasLockedBehaviorPack: boolean, isFromLockedWorldTemplate: boolean, msaGamertagsOnly: boolean, isFromWorldTemplate: boolean, onlySpawnV1Villagers: boolean, personaDisabled: boolean, customSkinsDisabled: boolean, emoteChatMuted: boolean, gameVersion: string, limitedWorldWidth: number, limitedWorldLength: number, isNewNether: boolean, eduResourceUri: EducationSharedResourceURI, experimentalGameplayOverride: boolean, chatRestrictionLevel: ChatRestrictionLevel, disabledPlayerInteractions: boolean, levelId: string, worldName: string, premiumWorldTemplateId: string, isTrial: boolean, movementAuthority: MovementAuthority, rewindHistorySize: number, serverAuthoritativeBlockBreaking: boolean, currentTick: number, enchantmentSeed: number, blockProperties: Array<BlockProperty>, itemStates: Array<ItemState>, multiplayerCorrelationId: string, serverAuthoritativeInventory: boolean, serverEngine: string, propertyData: string, blockPalletteChecksum: bigint, worldTemplateId: string, clientSideGeneration: boolean, blockNetworkIdsAreHashes: boolean, serverControlledSounds: boolean)
  static id(): number
}
export class BlockCoordinates {
  x: number
  y: number
  z: number
  constructor(x: number, y: number, z: number)
}
export class GameRule {
  name: string
  editable: boolean
  constructor(name: string, editable: boolean)
}
export type EducationSharedResourceURI = EducationSharedResourceUri
export class EducationSharedResourceUri {
  buttonName: string
  linkUri: string
  constructor(buttonName: string, linkUri: string)
}
export class BlockProperty {
  name: string
  state: string
  constructor(name: string, state: string)
}
export class ItemState {
  name: string
  runtimeId: number
  componentBased: boolean
  constructor(name: string, runtimeId: number, componentBased: boolean)
}
export class RequestNetworkSettingsPacket {
  protocolVersion: I32
  constructor(protocolVersion: I32)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): RequestNetworkSettingsPacket
}
export class NetworkSettingsPacket {
  compressionThreshold: number
  compressionAlgorithm: CompressionAlgorithm
  clientThrottle: boolean
  clientThrottleThreshold: number
  clientThrottleScalar: number
  constructor(compressionThreshold: number, compressionAlgorithm: CompressionAlgorithm, clientThrottle: boolean, clientThrottleThreshold: number, clientThrottleScalar: number)
  static id(): number
  serialize(): Buffer
  static deserialize(data: Buffer): NetworkSettingsPacket
}
