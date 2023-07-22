// Injected types by build.js
export type U8 = number;
export type I32 = number;
export type VarInt = number;

/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface RequestNetworkSettingsPacket {
  protocolVersion: I32
}
export const enum Packet {
  RequestNetworkSettings = 193
}

/**
 * Injected during post build. Helps TypeScript link enum to packet type.
 */
export interface PacketEnumToPacketInjection {
  [Packet.RequestNetworkSettings]: RequestNetworkSettingsPacket
}

// Updated by build.js
export function serialize<T extends Packet>(id: T, data: PacketEnumToPacketInjection[T]): Buffer;

// Updated by build.js
export function deserialize<T extends Packet>(id: T, data: Buffer): PacketEnumToPacketInjection[T];
