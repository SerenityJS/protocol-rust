import { ResourcePackInfo, BehaviourPackInfo, ResourcePacksInfoPacket } from '../../index.js'
import { compareNativeObjects } from '../napi.js'
import test from 'ava'

test.before('constructor', t => {
  // Create a resource pack info
  const resourceUuid = '0e71d229-62c3-46e7-811d-c2d2a0c71b0e'
  const resourceVersion = '1.0.0'
  const resourceSize = 1024n
  const resourceContentKey = 'some_random_string'
  const resourceSubPackName = 'some_random_string'
  const resourceContentId = 'some_random_string'
  const resourceHasScripts = true
  const resourceRaytracingCapable = true

  const resourcePack = new ResourcePackInfo(
    resourceUuid,
    resourceVersion,
    resourceSize,
    resourceContentKey,
    resourceSubPackName,
    resourceContentId,
    resourceHasScripts,
    resourceRaytracingCapable
  )

  // Ensure ResourcePackInfo data is assigned correctly
  t.is(resourcePack.uuid, resourceUuid)
  t.is(resourcePack.version, resourceVersion)
  t.is(resourcePack.size, resourceSize)
  t.is(resourcePack.contentKey, resourceContentKey)
  t.is(resourcePack.subPackName, resourceSubPackName)
  t.is(resourcePack.contentIdentity, resourceContentId)
  t.is(resourcePack.hasScripts, resourceHasScripts)
  t.is(resourcePack.rtxEnabled, resourceRaytracingCapable)

  // Create a behaviour pack info
  const behaviourUuid = '0e71d229-62c3-46e7-811d-c2d2a0c71b0e'
  const behaviourVersion = '1.0.0'
  const behaviourSize = 1024n
  const behaviourContentKey = 'some_random_string'
  const behaviourSubPackName = 'some_random_string'
  const behaviourContentId = 'some_random_string'
  const behaviourHasScripts = true

  const behaviourPack = new BehaviourPackInfo(
    behaviourUuid,
    behaviourVersion,
    behaviourSize,
    behaviourContentKey,
    behaviourSubPackName,
    behaviourContentId,
    behaviourHasScripts
  )

  // Ensure BehaviourPackInfo data is assigned correctly
  t.is(behaviourPack.uuid, behaviourUuid)
  t.is(behaviourPack.version, behaviourVersion)
  t.is(behaviourPack.size, behaviourSize)
  t.is(behaviourPack.contentKey, behaviourContentKey)
  t.is(behaviourPack.subPackName, behaviourSubPackName)
  t.is(behaviourPack.contentIdentity, behaviourContentId)
  t.is(behaviourPack.hasScripts, behaviourHasScripts)

  // Create a new resource packs info packet
  const mustAccept = true
  const hasScripts = true
  const forceServerPacks = true
  const packet = new ResourcePacksInfoPacket(
    mustAccept,
    hasScripts,
    forceServerPacks,
    [behaviourPack],
    [resourcePack]
  )

  // Do some checks to ensure everything matches
  t.is(packet.mustAccept, mustAccept)
  t.is(packet.hasScripts, hasScripts)
  t.is(packet.forceServerPacks, forceServerPacks)
  t.truthy(compareNativeObjects(packet.behaviourPacks[0], behaviourPack), 'Behaviour packs are not similar to the original object!')
  t.truthy(compareNativeObjects(packet.resourcePacks[0], resourcePack), 'Resource packs are not similar to the original object!')

  // Assign the bibically correct packet to the test context
  t.context = { packet }
})

test('serialization', t => {
  const { packet } = t.context as { packet: ResourcePacksInfoPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  
  // this doesn't really ensure that the packet is serialized correctly
  // the only way to tell is if the deserializaion test passes.
  t.is(serialize[0], ResourcePacksInfoPacket.id())
})

test('deserialization', t => {
  const { packet } = t.context as { packet: ResourcePacksInfoPacket }

  // Serialize and deserialize the packet
  const serialize = packet.serialize()
  const deserialize = ResourcePacksInfoPacket.deserialize(serialize)

  // Compare the two objects
  t.truthy(compareNativeObjects(packet, deserialize), 'Deserialized object is not similar to the original object!')
})

