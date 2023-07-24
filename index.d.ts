// Injected types by build.js
export type VarInt = number;

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface LoginPacket {
  protocolVersion: number
  tokens: LoginTokens
}
export interface LoginTokens {
  identity: string
  client: string
}
export interface DisconnectPacket {
  hideDisconnectScreen: boolean
  message: string
}
export interface RequestNetworkSettingsPacket {
  protocolVersion: number
}
export const enum Packet {
  Login = 1,
  Disconnect = 5,
  RequestNetworkSettings = 193
}

/**
 * Injected during post build. Helps TypeScript link enum to packet type.
 */
export interface PacketEnumToPacketInjection {
  [Packet.Login]: LoginPacket;
  [Packet.Disconnect]: DisconnectPacket;
  [Packet.RequestNetworkSettings]: RequestNetworkSettingsPacket;
}

// Updated by build.js
export function serialize<T extends Packet>(id: T, data: PacketEnumToPacketInjection[T]): Buffer;

// Updated by build.js
export function deserialize<T extends Packet>(id: T, data: Buffer): PacketEnumToPacketInjection[T];
