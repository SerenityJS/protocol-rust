// Runs after napi to dynamically inject some type fixes into the index.d.ts file
const fs = require('fs');
const path = require('path');

// Recommended use template bierner.comment-tagged-templates
console.log('[Post Build] Injecting types into index.d.ts');

const FilePath = path.join(__dirname, 'index.d.ts');
let File = fs.readFileSync(FilePath, 'utf8');

// Function to inject a string into a file at a given index
const inject = (str, index) => {
  File = File.slice(0, index) + str + File.slice(index);

  return index + str.length;
}

// Function remove a string from a file at a given index
const remove = (str, index) => {
  File = File.slice(0, index) + File.slice(index + str.length);

  return index;
}

// TODO: we can probably actually read the rust file containing these type
// abstractions and inject them here instead of hardcoding them
// Injected into the top of index.d.ts these are type aliases inside of rust
// that napi doesn't know how to serialize so it just assumes the type exists.
const injection = /* ts */ `// Injected types by build.js
export type VarInt = number;
export type LU16 = number;
export type LF32 = number;
export type U64 = bigint;
`; 
inject(injection, 0);

// Type hacker does some advanced hacky parsing to ensure the serialize and deserialize
// methods are typed. This is not amazing but it works.
const packetEnumMatcher = /export const enum Packet \{\n(.*,\n)*.*\n}/;
const packetEnumFieldsMatcher = /.* = \d*/g;

const packetEnumMatch = File.match(packetEnumMatcher);
if (!packetEnumMatch || !packetEnumMatch[0]) {
  throw new Error('Failed to find Packet enum in index.d.ts');
}

const packetEnum = packetEnumMatch[0];
const packetEnumFields = packetEnum.match(packetEnumFieldsMatcher);
const packetEnumStrings = packetEnumFields.map((field) => {
  const [name] = field.split('=');
  return name.trim();
});

// This seems a bit hacky but because of how the proc macro works
// in rust we already follow a strict naming convention for packets
const enumToPacketInterfaceFields = packetEnumStrings.map((name) => {
  return /* ts */ `  [Packet.${name}]: ${name}Packet;`;
})

const enumToPacketInterface = /* ts */ `
/**
 * Injected during post build. Helps TypeScript link enum to packet type.
 */
export interface PacketEnumToPacketInjection {
${enumToPacketInterfaceFields.join('\n')}
}
`;

// Now we need to update the serialize and deserialize functions to use the new types
const serializeMatcher = /export function serialize\(id: Packet, data: object\): Buffer\n/;
const deserializeMatcher = /export function deserialize\(id: Packet, data: Buffer\): object\n/;

// Attempt to find the serialize function type so we can remove it
const serializeMatch = File.match(serializeMatcher);
if (!serializeMatch || !serializeMatch[0]) {
  throw new Error('Failed to find serialize function in index.d.ts');
}
const serializeStart = serializeMatch.index;
remove(serializeMatch[0], serializeStart);

// Attempt to find the deserialize function type so we can remove it
const deserializeMatch = File.match(deserializeMatcher);
if (!deserializeMatch || !deserializeMatch[0]) {
  throw new Error('Failed to find deserialize function in index.d.ts');
}
const deserializeStart = deserializeMatch.index;
remove(deserializeMatch[0], deserializeStart);

// We want to create a cursor since the next bit of injection should all be together
// Then we want to inject the interface which links the Packet enum to its Packet type
let injectionCursor = deserializeStart;
injectionCursor = inject(enumToPacketInterface, injectionCursor);

// New typed serialize method to replace the old one
const newSerialize = /* ts */ `
// Updated by build.js
export function serialize<T extends Packet>(id: T, data: PacketEnumToPacketInjection[T]): Buffer;
`;
injectionCursor = inject(newSerialize, injectionCursor);

// New typed deserialize method to replace the old one
const newDeserialize = /* ts */ `
// Updated by build.js
export function deserialize<T extends Packet>(id: T, data: Buffer): PacketEnumToPacketInjection[T];
`;
injectionCursor = inject(newDeserialize, injectionCursor);

// Write Changes
fs.writeFileSync(FilePath, File, 'utf8');

console.log('[Post Build] Finished injecting types into index.d.ts');
