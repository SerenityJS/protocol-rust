/* tslint:disable */
/* eslint-disable */
/* prettier-ignore */

/* auto-generated by NAPI-RS */

const { existsSync, readFileSync } = require('fs')
const { join } = require('path')

const { platform, arch } = process

let nativeBinding = null
let localFileExisted = false
let loadError = null

function isMusl() {
  // For Node 10
  if (!process.report || typeof process.report.getReport !== 'function') {
    try {
      const lddPath = require('child_process').execSync('which ldd').toString().trim()
      return readFileSync(lddPath, 'utf8').includes('musl')
    } catch (e) {
      return true
    }
  } else {
    const { glibcVersionRuntime } = process.report.getReport().header
    return !glibcVersionRuntime
  }
}

switch (platform) {
  case 'android':
    switch (arch) {
      case 'arm64':
        localFileExisted = existsSync(join(__dirname, 'protocol.android-arm64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.android-arm64.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-android-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm':
        localFileExisted = existsSync(join(__dirname, 'protocol.android-arm-eabi.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.android-arm-eabi.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-android-arm-eabi')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Android ${arch}`)
    }
    break
  case 'win32':
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(
          join(__dirname, 'protocol.win32-x64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.win32-x64-msvc.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-win32-x64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'ia32':
        localFileExisted = existsSync(
          join(__dirname, 'protocol.win32-ia32-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.win32-ia32-msvc.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-win32-ia32-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'protocol.win32-arm64-msvc.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.win32-arm64-msvc.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-win32-arm64-msvc')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Windows: ${arch}`)
    }
    break
  case 'darwin':
    localFileExisted = existsSync(join(__dirname, 'protocol.darwin-universal.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./protocol.darwin-universal.node')
      } else {
        nativeBinding = require('@serenityjs/protocol-darwin-universal')
      }
      break
    } catch {}
    switch (arch) {
      case 'x64':
        localFileExisted = existsSync(join(__dirname, 'protocol.darwin-x64.node'))
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.darwin-x64.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-darwin-x64')
          }
        } catch (e) {
          loadError = e
        }
        break
      case 'arm64':
        localFileExisted = existsSync(
          join(__dirname, 'protocol.darwin-arm64.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.darwin-arm64.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-darwin-arm64')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on macOS: ${arch}`)
    }
    break
  case 'freebsd':
    if (arch !== 'x64') {
      throw new Error(`Unsupported architecture on FreeBSD: ${arch}`)
    }
    localFileExisted = existsSync(join(__dirname, 'protocol.freebsd-x64.node'))
    try {
      if (localFileExisted) {
        nativeBinding = require('./protocol.freebsd-x64.node')
      } else {
        nativeBinding = require('@serenityjs/protocol-freebsd-x64')
      }
    } catch (e) {
      loadError = e
    }
    break
  case 'linux':
    switch (arch) {
      case 'x64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'protocol.linux-x64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./protocol.linux-x64-musl.node')
            } else {
              nativeBinding = require('@serenityjs/protocol-linux-x64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'protocol.linux-x64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./protocol.linux-x64-gnu.node')
            } else {
              nativeBinding = require('@serenityjs/protocol-linux-x64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm64':
        if (isMusl()) {
          localFileExisted = existsSync(
            join(__dirname, 'protocol.linux-arm64-musl.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./protocol.linux-arm64-musl.node')
            } else {
              nativeBinding = require('@serenityjs/protocol-linux-arm64-musl')
            }
          } catch (e) {
            loadError = e
          }
        } else {
          localFileExisted = existsSync(
            join(__dirname, 'protocol.linux-arm64-gnu.node')
          )
          try {
            if (localFileExisted) {
              nativeBinding = require('./protocol.linux-arm64-gnu.node')
            } else {
              nativeBinding = require('@serenityjs/protocol-linux-arm64-gnu')
            }
          } catch (e) {
            loadError = e
          }
        }
        break
      case 'arm':
        localFileExisted = existsSync(
          join(__dirname, 'protocol.linux-arm-gnueabihf.node')
        )
        try {
          if (localFileExisted) {
            nativeBinding = require('./protocol.linux-arm-gnueabihf.node')
          } else {
            nativeBinding = require('@serenityjs/protocol-linux-arm-gnueabihf')
          }
        } catch (e) {
          loadError = e
        }
        break
      default:
        throw new Error(`Unsupported architecture on Linux: ${arch}`)
    }
    break
  default:
    throw new Error(`Unsupported OS: ${platform}, architecture: ${arch}`)
}

if (!nativeBinding) {
  if (loadError) {
    throw loadError
  }
  throw new Error(`Failed to load native binding`)
}

const { PlayStatus, ResourceResponseStatus, TextType, GameMode, Dimension, GameRuleType, PermissionLevel, ChatRestrictionLevel, MovementAuthority, CommandPermissionLevel, MovePlayerMode, BlockEventType, EntityEventType, WindowId, InteractAction, SpawnType, AnimateActions, CompressionAlgorithm, BroadcastAction, ViolationType, ViolationSeverity, ShakeAction, SimulationType, DialogueAction, SubChunkTransitionType, Packet, serialize, deserialize, getPacketId, framePackets, unframePackets, makeMotd } = nativeBinding

module.exports.PlayStatus = PlayStatus
module.exports.ResourceResponseStatus = ResourceResponseStatus
module.exports.TextType = TextType
module.exports.GameMode = GameMode
module.exports.Dimension = Dimension
module.exports.GameRuleType = GameRuleType
module.exports.PermissionLevel = PermissionLevel
module.exports.ChatRestrictionLevel = ChatRestrictionLevel
module.exports.MovementAuthority = MovementAuthority
module.exports.CommandPermissionLevel = CommandPermissionLevel
module.exports.MovePlayerMode = MovePlayerMode
module.exports.BlockEventType = BlockEventType
module.exports.EntityEventType = EntityEventType
module.exports.WindowId = WindowId
module.exports.InteractAction = InteractAction
module.exports.SpawnType = SpawnType
module.exports.AnimateActions = AnimateActions
module.exports.CompressionAlgorithm = CompressionAlgorithm
module.exports.BroadcastAction = BroadcastAction
module.exports.ViolationType = ViolationType
module.exports.ViolationSeverity = ViolationSeverity
module.exports.ShakeAction = ShakeAction
module.exports.SimulationType = SimulationType
module.exports.DialogueAction = DialogueAction
module.exports.SubChunkTransitionType = SubChunkTransitionType
module.exports.Packet = Packet
module.exports.serialize = serialize
module.exports.deserialize = deserialize
module.exports.getPacketId = getPacketId
module.exports.framePackets = framePackets
module.exports.unframePackets = unframePackets
module.exports.makeMotd = makeMotd
