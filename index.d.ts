// Injected types by build.js
export type VarInt = number;
export type VarLong = bigint;
export type ZigZag = number;
export type ZigZong = bigint;
export type LU16 = number;
export type LI16 = number;
export type LI32 = number;
export type LI64 = bigint;
export type LF32 = number;
export type LittleString = string;
export type LU64 = bigint;
export type UUID = string;

/**
 * This type internally is used very weirdly. When deserializing it will be the NBT in object format.
 * All metadata is stripped. When serializing it will expect metadata to be present. Use the exported NBT
 * namespace to easily create NBT objects.
 * 
 * This package does not export extensive NBT types. It is recommended to extend the NBT type with your own.
 */
export type NBT = Record<string, any>;
/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Vec3F {
  x: LF32
  y: LF32
  z: LF32
}
export interface Vec2F {
  x: LF32
  z: LF32
}
export interface Experiment {
  name: string
  enabled: boolean
}
export interface LoginPacket {
  protocolVersion: number
  tokens: LoginTokens
}
export interface LoginTokens {
  identity: LittleString
  client: LittleString
}
export interface PlayStatusPacket {
  status: PlayStatus
}
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
export interface ServerToClientHandshakePacket {
  token: string
}
export interface ClientToServerHandshakePacket {
  
}
export interface DisconnectPacket {
  hideDisconnectScreen: boolean
  message: string
}
export interface ResourcePacksInfoPacket {
  mustAccept: boolean
  hasScripts: boolean
  forceServerPacks: boolean
  behaviourPacks: Array<BehaviourPackInfo>
  resourcePacks: Array<ResourcePackInfo>
}
export interface BehaviourPackInfo {
  uuid: string
  version: string
  size: LU64
  contentKey: string
  subPackName: string
  contentIdentity: string
  hasScripts: boolean
}
export interface ResourcePackInfo {
  uuid: string
  version: string
  size: LU64
  contentKey: string
  subPackName: string
  contentIdentity: string
  hasScripts: boolean
  rtxEnabled: boolean
}
export interface ResourcePacksStackPacket {
  mustAccept: boolean
  behaviourPacks: Array<PackIdVersion>
  resourcePacks: Array<PackIdVersion>
  gameVersion: string
  experiments: Array<Experiment>
  experimentsPreviouslyUsed: boolean
}
export interface PackIdVersion {
  uuid: string
  version: string
  name: string
}
export interface StartGamePacket {
  entityId: ZigZong
  runtimeEntityId: VarLong
  playerGamemode: GameMode
  playerPosition: Vec3F
  playerRotation: Vec2F
  seed: LU64
  biomeType: LI16
  biomeName: string
  dimension: Dimension
  generator: ZigZag
  worldGamemode: GameMode
  difficulty: ZigZag
  spawnPosition: BlockCoordinates
  achievementsDisabled: boolean
  editorWorld: boolean
  createdInEditor: boolean
  exportedFromEditor: boolean
  dayCycleStopTime: ZigZag
  eduOffer: ZigZag
  eduFeaturesEnabled: boolean
  eduProductId: string
  rainLevel: LF32
  lightningLevel: LF32
  confirmedPlatformLockedContent: boolean
  multiplayerGame: boolean
  broadcastToLan: boolean
  xblBroadcastMode: VarInt
  platformBroadcastMode: VarInt
  enableCommands: boolean
  texturePacksRequired: boolean
  gamerules: Array<GameRule>
  experiments: Array<Experiment>
  experimentsPreviouslyUsed: boolean
  bonusChest: boolean
  mapEnabled: boolean
  permissionLevel: PermissionLevel
  serverChunkTickRange: LI32
  hasLockedBehaviourPacks: boolean
  hasLockedResourcePacks: boolean
  fromLockedWorldTemplate: boolean
  msaGamertagsOnly: boolean
  fromWorldTemplate: boolean
  worldTemplateOptionLocked: boolean
  onlySpawnV1Villagers: boolean
  personaDisabled: boolean
  customSkinsDisabled: boolean
  emoteChatMuted: boolean
  gameVersion: string
  limitedWorldWidth: LI32
  limitedWorldLength: LI32
  newNether: boolean
  eduResourceUri: EducationSharedResource
  experimentalGameplayOverride: boolean
  chatRestrictionLevel: ChatRestrictionLevel
  disablePlayerInteractions: boolean
  levelId: string
  worldName: string
  premiumWorldTemplateId: string
  isTrial: boolean
  movementAuthority: MovementAuthority
  rewindHistorySize: ZigZag
  serverAuthoritiveBlockBreaking: boolean
  currentTick: LI64
  enchantmentSeed: ZigZag
  blockProperties: Array<BlockProperty>
  itemStates: Array<ItemState>
  multiplayerCorrelationId: string
  serverAuthoritativeInventory: boolean
  engine: string
  propertyData: NBT
  blockPalletteChecksum: LU64
  worldTemplateId: UUID
  clientSideGeneration: boolean
  blockNetworkIdsAreHashes: boolean
  serverControlledSound: boolean
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
export interface BlockCoordinates {
  x: ZigZag
  y: VarInt
  z: ZigZag
}
export interface GameRule {
  name: string
  editable: boolean
  fieldType: GameRuleType
  value: boolean | number
}
export const enum GameRuleType {
  Bool = 1,
  Int = 2,
  Float = 3
}
export const enum PermissionLevel {
  Visitor = 0,
  Member = 1,
  Operator = 2,
  Custom = 3
}
export interface EducationSharedResource {
  buttonName: string
  linkUri: string
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
export interface BlockProperty {
  name: string
  state: NBT
}
export interface ItemState {
  name: string
  runtimeId: LI16
  componentBased: boolean
}
export interface NetworkSettingsPacket {
  compressionThreshold: LU16
  compressionAlgorithm: CompressionAlgorithm
  clientThrottle: boolean
  clientThrottleThreshold: number
  clientThrottleScalar: LF32
}
export const enum CompressionAlgorithm {
  Deflate = 0,
  Snappy = 1
}
export interface RequestNetworkSettingsPacket {
  protocolVersion: number
}
export interface UpdatePlayerGameTypePacket {
  gamemode: GameMode
  playerUniqueId: ZigZong
}
export interface EmoteListPacket {
  playerUniqueId: ZigZong
  emoteIds: Array<UUID>
}
export interface PositionTrackingDbRequestPacket {
  action: BroadcastAction
  trackingId: ZigZag
}
export const enum BroadcastAction {
  Update = 0,
  Destroy = 1,
  NotFound = 2
}
export interface PositionTrackingDbBroadcastPacket {
  action: BroadcastAction
  trackingId: ZigZag
}
export interface PacketViolationWarningPacket {
  violationType: ViolationType
  severity: ViolationSeverity
  packetyId: ZigZag
  reason: string
}
export const enum ViolationType {
  Malformed = 0
}
export const enum ViolationSeverity {
  Warning = 0,
  Severe = 1,
  Fatal = 2
}
export interface MotionPredictionHintsPacket {
  entityRuntimeId: VarLong
  velocity: Vec3F
  onGround: boolean
}
export interface AnimateEntityPacket {
  animation: string
  nextState: string
  stopCondition: string
  stopConditionVerion: LI32
  controller: string
  blendOutTime: LF32
  entityRuntimeIds: Array<VarLong>
}
export interface CameraShakePacket {
  intensity: LF32
  duration: LF32
  shakeType: number
  action: ShakeAction
}
export const enum ShakeAction {
  Add = 0,
  Stop = 1
}
export interface PlayerFogPacket {
  stack: Array<string>
}
export interface CorrectPlayerMovePredictionPacket {
  position: Vec3F
  delta: Vec3F
  onGround: boolean
  tick: VarLong
}
export interface ItemComponentPacket {
  entries: Array<ItemComponentList>
}
export interface ItemComponentList {
  name: string
  nbt: NBT
}
export interface FilterTextPacket {
  text: string
  fromServer: boolean
}
export interface SyncEntityPropertyPacket {
  nbt: NBT
}
export interface AddVolumeEntityPacket {
  runtimeId: VarLong
  nbt: NBT
  encodingIdentifier: string
  instanceName: string
  bounds: EntityBounds
  dimension: ZigZag
  engineVersion: string
}
export interface EntityBounds {
  min: BlockCoordinates
  max: BlockCoordinates
}
export interface RemoveVolumeEntityPacket {
  runtimeId: VarLong
}
export interface SimulationTypePacket {
  status: SimulationType
}
export const enum SimulationType {
  Game = 0,
  Editor = 1,
  Test = 2,
  Invalid = 3
}
export interface NpcDialoguePacket {
  entityId: LU64
  action: DialogueAction
  dialogue: string
  screenName: string
  npcName: string
  actionJson: string
}
export const enum DialogueAction {
  Open = 0,
  Close = 1
}
export const enum Packet {
  Login = 1,
  PlayStatus = 2,
  ServerToClientHandshake = 3,
  ClientToServerHandshake = 4,
  Disconnect = 5,
  ResourcePacksInfo = 6,
  ResourcePacksStack = 7,
  StartGame = 11,
  NetworkSettings = 143,
  RequestNetworkSettings = 193,
  UpdatePlayerGameType = 151,
  EmoteList = 152,
  PositionTrackingDbRequest = 154,
  PositionTrackingDbBroadcast = 153,
  PacketViolationWarning = 156,
  MotionPredictionHints = 157,
  AnimateEntity = 158,
  CameraShake = 159,
  PlayerFog = 160,
  CorrectPlayerMovePrediction = 161,
  ItemComponent = 162,
  FilterText = 163,
  SyncEntityProperty = 165,
  AddVolumeEntity = 166,
  RemoveVolumeEntity = 167,
  SimulationType = 168,
  NpcDialogue = 169
}

/**
 * Injected during post build. Helps TypeScript link enum to packet type.
 */
export interface PacketEnumToPacketInjection {
  [Packet.Login]: LoginPacket;
  [Packet.PlayStatus]: PlayStatusPacket;
  [Packet.ServerToClientHandshake]: ServerToClientHandshakePacket;
  [Packet.ClientToServerHandshake]: ClientToServerHandshakePacket;
  [Packet.Disconnect]: DisconnectPacket;
  [Packet.ResourcePacksInfo]: ResourcePacksInfoPacket;
  [Packet.ResourcePacksStack]: ResourcePacksStackPacket;
  [Packet.StartGame]: StartGamePacket;
  [Packet.NetworkSettings]: NetworkSettingsPacket;
  [Packet.RequestNetworkSettings]: RequestNetworkSettingsPacket;
  [Packet.UpdatePlayerGameType]: UpdatePlayerGameTypePacket;
  [Packet.EmoteList]: EmoteListPacket;
  [Packet.PositionTrackingDbRequest]: PositionTrackingDbRequestPacket;
  [Packet.PositionTrackingDbBroadcast]: PositionTrackingDbBroadcastPacket;
  [Packet.PacketViolationWarning]: PacketViolationWarningPacket;
  [Packet.MotionPredictionHints]: MotionPredictionHintsPacket;
  [Packet.AnimateEntity]: AnimateEntityPacket;
  [Packet.CameraShake]: CameraShakePacket;
  [Packet.PlayerFog]: PlayerFogPacket;
  [Packet.CorrectPlayerMovePrediction]: CorrectPlayerMovePredictionPacket;
  [Packet.ItemComponent]: ItemComponentPacket;
  [Packet.FilterText]: FilterTextPacket;
  [Packet.SyncEntityProperty]: SyncEntityPropertyPacket;
  [Packet.AddVolumeEntity]: AddVolumeEntityPacket;
  [Packet.RemoveVolumeEntity]: RemoveVolumeEntityPacket;
  [Packet.SimulationType]: SimulationTypePacket;
  [Packet.NpcDialogue]: NpcDialoguePacket;
}

// Updated by build.js
export function serialize<T extends Packet>(id: T, data: PacketEnumToPacketInjection[T]): Buffer;

// Updated by build.js
export function deserialize<T extends Packet>(id: T, data: Buffer): PacketEnumToPacketInjection[T];
export function getPacketId(data: Buffer): number
export function framePackets(packets: Array<Buffer>): Buffer
export function unframePackets(data: Buffer): Array<Buffer>
export function makeMotd(motd: string, protocolVersion: number, version: string, currentPlayers: number, maxPlayers: number, serverId: string, worldName: string, gamemode: string, gamemodeId: number, portv4: number, portv6: number): Buffer
