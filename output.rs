#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec3f { pub x : LF32, pub y : LF32, pub z : LF32, } impl crate :: packets ::
prelude :: PacketChildSerialization for Vec3f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.y.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let y
        : LF32 = stream.read_lf32() ? ; let z : LF32 = stream.read_lf32() ? ;
        Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec3f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let y : LF32 = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec3f { pub x : LF32, pub y : LF32, pub z : LF32, } impl crate :: packets ::
prelude :: PacketChildSerialization for Vec3f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.y.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let y
        : LF32 = stream.read_lf32() ? ; let z : LF32 = stream.read_lf32() ? ;
        Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec3f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let y : LF32 = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec2f { pub x : LF32, pub z : LF32, } impl crate :: packets :: prelude ::
PacketChildSerialization for Vec2f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let z
        : LF32 = stream.read_lf32() ? ; Ok(Self { x, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec2f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec2f { pub x : LF32, pub z : LF32, } impl crate :: packets :: prelude ::
PacketChildSerialization for Vec2f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let z
        : LF32 = stream.read_lf32() ? ; Ok(Self { x, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec2f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Experiment { pub name : String, pub enabled : bool, } impl crate :: packets ::
prelude :: PacketChildSerialization for Experiment
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_bool(self.enabled.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let enabled : bool = stream.read_bool() ? ; Ok(Self { name, enabled })
    }
} impl crate :: packets :: prelude :: PacketConversion for Experiment
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("enabled", self.enabled.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let enabled : bool = match data.get_named_property("enabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "enabled",
            stringify! (bool), err),))
        } ; Ok(Self { name, enabled })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Experiment { pub name : String, pub enabled : bool, } impl crate :: packets ::
prelude :: PacketChildSerialization for Experiment
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_bool(self.enabled.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let enabled : bool = stream.read_bool() ? ; Ok(Self { name, enabled })
    }
} impl crate :: packets :: prelude :: PacketConversion for Experiment
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("enabled", self.enabled.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let enabled : bool = match data.get_named_property("enabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "enabled",
            stringify! (bool), err),))
        } ; Ok(Self { name, enabled })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
StartGamePacket
{
    pub entity_id : ZigZong, pub runtime_entity_id : VarLong,
    #[napi(ts_type = "GameMode")] pub player_gamemode : ZigZag, pub
    player_position : Vec3f, pub player_rotation : Vec2f, pub seed : LU64, pub
    biome_type : LI16, pub biome_name : String, #[napi(ts_type = "Dimension")]
    pub dimension : ZigZag, pub generator : ZigZag,
    #[napi(ts_type = "GameMode")] pub world_gamemode : ZigZag, pub difficulty
    : ZigZag, pub spawn_position : BlockCoordinates, pub achievements_disabled
    : bool, pub editor_world : bool, pub created_in_editor : bool, pub
    day_cycle_stop_time : ZigZag, pub edu_offer : ZigZag, pub
    edu_features_enabled : bool, pub edu_product_id : String, pub rain_level :
    LF32, pub lightning_level : LF32, pub confirmed_platform_locked_content :
    bool, pub multiplayer_game : bool, pub broadcast_to_lan : bool, pub
    xbl_broadcast_mode : VarInt, pub platform_broadcast_mode : VarInt, pub
    enable_commands : bool, pub texture_packs_required : bool, #[VarInt] pub
    gamerules : Vec < GameRule >, #[LI32] pub experiments : Vec < Experiment
    >, pub experiments_previously_used : bool, pub bonus_chest : bool, pub
    map_enabled : bool, #[napi(ts_type = "PermissionLevel")] pub
    permission_level : u8, pub server_chunk_tick_range : LI32, pub
    has_locked_behaviour_packs : bool, pub has_locked_resource_packs : bool,
    pub from_locked_world_template : bool, pub msa_gamertags_only : bool, pub
    from_world_template : bool, pub world_template_option_locked : bool, pub
    only_spawn_v1_villagers : bool, pub persona_disabled : bool, pub
    custom_skins_disabled : bool, pub emote_chat_muted : bool, pub
    game_version : String, pub limited_world_width : LI32, pub
    limited_world_length : LI32, pub new_nether : bool, pub edu_resource_uri :
    EducationSharedResource, pub experimental_gameplay_override : bool,
    #[napi(ts_type = "ChatRestrictionLevel")] pub chat_restriction_level : u8,
    pub disable_player_interactions : bool, pub level_id : String, pub
    world_name : String, pub premium_world_template_id : String, pub is_trial
    : bool, #[napi(ts_type = "MovementAuthority")] pub movement_authority :
    ZigZag, pub rewind_history_size : ZigZag, pub
    server_authoritive_block_breaking : bool, pub current_tick : LI64, pub
    enchantment_seed : ZigZag, #[VarInt] pub block_properties : Vec <
    BlockProperty >, #[VarInt] pub item_states : Vec < ItemState >, pub
    multiplayer_correlation_id : String, pub server_authoritative_inventory :
    bool, pub engine : String, pub property_data : NBT, pub
    block_pallette_checksum : LU64, pub world_template_id : UUID, pub
    client_side_generation : bool, pub block_network_ids_are_hashes : bool,
    pub server_controlled_sound : bool
} impl StartGamePacket
{ pub const ID : crate :: binary :: prelude :: VarInt = 11u32 ; } impl crate
:: packets :: prelude :: PacketSerialization for StartGamePacket
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < napi ::
    bindgen_prelude :: Buffer >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_varint(StartGamePacket :: ID) ? ;
        stream.write_zigzong(self.entity_id.to_owned()) ? ;
        stream.write_varlong(self.runtime_entity_id.to_owned()) ? ;
        stream.write_zigzag(self.player_gamemode.to_owned()) ? ;
        stream.append(& mut self.player_position.serialize() ?) ;
        stream.append(& mut self.player_rotation.serialize() ?) ;
        stream.write_lu64(self.seed.to_owned()) ? ;
        stream.write_li16(self.biome_type.to_owned()) ? ;
        stream.write_string(self.biome_name.to_owned()) ? ;
        stream.write_zigzag(self.dimension.to_owned()) ? ;
        stream.write_zigzag(self.generator.to_owned()) ? ;
        stream.write_zigzag(self.world_gamemode.to_owned()) ? ;
        stream.write_zigzag(self.difficulty.to_owned()) ? ;
        stream.append(& mut self.spawn_position.serialize() ?) ;
        stream.write_bool(self.achievements_disabled.to_owned()) ? ;
        stream.write_bool(self.editor_world.to_owned()) ? ;
        stream.write_bool(self.created_in_editor.to_owned()) ? ;
        stream.write_zigzag(self.day_cycle_stop_time.to_owned()) ? ;
        stream.write_zigzag(self.edu_offer.to_owned()) ? ;
        stream.write_bool(self.edu_features_enabled.to_owned()) ? ;
        stream.write_string(self.edu_product_id.to_owned()) ? ;
        stream.write_lf32(self.rain_level.to_owned()) ? ;
        stream.write_lf32(self.lightning_level.to_owned()) ? ;
        stream.write_bool(self.confirmed_platform_locked_content.to_owned()) ?
        ; stream.write_bool(self.multiplayer_game.to_owned()) ? ;
        stream.write_bool(self.broadcast_to_lan.to_owned()) ? ;
        stream.write_varint(self.xbl_broadcast_mode.to_owned()) ? ;
        stream.write_varint(self.platform_broadcast_mode.to_owned()) ? ;
        stream.write_bool(self.enable_commands.to_owned()) ? ;
        stream.write_bool(self.texture_packs_required.to_owned()) ? ; let
        gamerules_len = self.gamerules.len() ;
        stream.write_varint(gamerules_len as VarInt) ? ; for i in 0 ..
        gamerules_len
        {
            let mut bin = self.gamerules [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let experiments_len = self.experiments.len() ;
        stream.write_li32(experiments_len as LI32) ? ; for i in 0 ..
        experiments_len
        {
            let mut bin = self.experiments [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_bool(self.experiments_previously_used.to_owned()) ? ;
        stream.write_bool(self.bonus_chest.to_owned()) ? ;
        stream.write_bool(self.map_enabled.to_owned()) ? ;
        stream.write_u8(self.permission_level.to_owned()) ? ;
        stream.write_li32(self.server_chunk_tick_range.to_owned()) ? ;
        stream.write_bool(self.has_locked_behaviour_packs.to_owned()) ? ;
        stream.write_bool(self.has_locked_resource_packs.to_owned()) ? ;
        stream.write_bool(self.from_locked_world_template.to_owned()) ? ;
        stream.write_bool(self.msa_gamertags_only.to_owned()) ? ;
        stream.write_bool(self.from_world_template.to_owned()) ? ;
        stream.write_bool(self.world_template_option_locked.to_owned()) ? ;
        stream.write_bool(self.only_spawn_v1_villagers.to_owned()) ? ;
        stream.write_bool(self.persona_disabled.to_owned()) ? ;
        stream.write_bool(self.custom_skins_disabled.to_owned()) ? ;
        stream.write_bool(self.emote_chat_muted.to_owned()) ? ;
        stream.write_string(self.game_version.to_owned()) ? ;
        stream.write_li32(self.limited_world_width.to_owned()) ? ;
        stream.write_li32(self.limited_world_length.to_owned()) ? ;
        stream.write_bool(self.new_nether.to_owned()) ? ;
        stream.append(& mut self.edu_resource_uri.serialize() ?) ;
        stream.write_bool(self.experimental_gameplay_override.to_owned()) ? ;
        stream.write_u8(self.chat_restriction_level.to_owned()) ? ;
        stream.write_bool(self.disable_player_interactions.to_owned()) ? ;
        stream.write_string(self.level_id.to_owned()) ? ;
        stream.write_string(self.world_name.to_owned()) ? ;
        stream.write_string(self.premium_world_template_id.to_owned()) ? ;
        stream.write_bool(self.is_trial.to_owned()) ? ;
        stream.write_zigzag(self.movement_authority.to_owned()) ? ;
        stream.write_zigzag(self.rewind_history_size.to_owned()) ? ;
        stream.write_bool(self.server_authoritive_block_breaking.to_owned()) ?
        ; stream.write_li64(self.current_tick.to_owned()) ? ;
        stream.write_zigzag(self.enchantment_seed.to_owned()) ? ; let
        block_properties_len = self.block_properties.len() ;
        stream.write_varint(block_properties_len as VarInt) ? ; for i in 0 ..
        block_properties_len
        {
            let mut bin = self.block_properties [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let item_states_len = self.item_states.len() ;
        stream.write_varint(item_states_len as VarInt) ? ; for i in 0 ..
        item_states_len
        {
            let mut bin = self.item_states [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_string(self.multiplayer_correlation_id.to_owned()) ? ;
        stream.write_bool(self.server_authoritative_inventory.to_owned()) ? ;
        stream.write_string(self.engine.to_owned()) ? ;
        stream.append(& mut self.property_data.serialize() ?) ;
        stream.write_lu64(self.block_pallette_checksum.to_owned()) ? ;
        stream.write_uuid(self.world_template_id.to_owned()) ? ;
        stream.write_bool(self.client_side_generation.to_owned()) ? ;
        stream.write_bool(self.block_network_ids_are_hashes.to_owned()) ? ;
        stream.write_bool(self.server_controlled_sound.to_owned()) ? ;
        Ok(stream.data.into())
    } fn deserialize(data : napi :: bindgen_prelude :: Buffer) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let mut stream = crate :: binary :: BinaryStream :: from(data.into())
        ; let _id = stream.read_varint() ? ; let entity_id : ZigZong =
        stream.read_zigzong() ? ; let runtime_entity_id : VarLong =
        stream.read_varlong() ? ; let player_gamemode : ZigZag =
        stream.read_zigzag() ? ; let player_position : Vec3f = Vec3f ::
        deserialize(& mut stream) ? ; let player_rotation : Vec2f = Vec2f ::
        deserialize(& mut stream) ? ; let seed : LU64 = stream.read_lu64() ? ;
        let biome_type : LI16 = stream.read_li16() ? ; let biome_name : String
        = stream.read_string() ? ; let dimension : ZigZag =
        stream.read_zigzag() ? ; let generator : ZigZag = stream.read_zigzag()
        ? ; let world_gamemode : ZigZag = stream.read_zigzag() ? ; let
        difficulty : ZigZag = stream.read_zigzag() ? ; let spawn_position :
        BlockCoordinates = BlockCoordinates :: deserialize(& mut stream) ? ;
        let achievements_disabled : bool = stream.read_bool() ? ; let
        editor_world : bool = stream.read_bool() ? ; let created_in_editor :
        bool = stream.read_bool() ? ; let day_cycle_stop_time : ZigZag =
        stream.read_zigzag() ? ; let edu_offer : ZigZag = stream.read_zigzag()
        ? ; let edu_features_enabled : bool = stream.read_bool() ? ; let
        edu_product_id : String = stream.read_string() ? ; let rain_level :
        LF32 = stream.read_lf32() ? ; let lightning_level : LF32 =
        stream.read_lf32() ? ; let confirmed_platform_locked_content : bool =
        stream.read_bool() ? ; let multiplayer_game : bool =
        stream.read_bool() ? ; let broadcast_to_lan : bool =
        stream.read_bool() ? ; let xbl_broadcast_mode : VarInt =
        stream.read_varint() ? ; let platform_broadcast_mode : VarInt =
        stream.read_varint() ? ; let enable_commands : bool =
        stream.read_bool() ? ; let texture_packs_required : bool =
        stream.read_bool() ? ; let gamerules_len = stream.read_varint() ? ;
        let mut gamerules : Vec < GameRule > = Vec :: new() ; for _ in 0 ..
        gamerules_len
        { gamerules.push(GameRule :: deserialize(& mut stream) ?) ; } let
        experiments_len = stream.read_li32() ? ; let mut experiments : Vec <
        Experiment > = Vec :: new() ; for _ in 0 .. experiments_len
        { experiments.push(Experiment :: deserialize(& mut stream) ?) ; } let
        experiments_previously_used : bool = stream.read_bool() ? ; let
        bonus_chest : bool = stream.read_bool() ? ; let map_enabled : bool =
        stream.read_bool() ? ; let permission_level : u8 = stream.read_u8() ?
        ; let server_chunk_tick_range : LI32 = stream.read_li32() ? ; let
        has_locked_behaviour_packs : bool = stream.read_bool() ? ; let
        has_locked_resource_packs : bool = stream.read_bool() ? ; let
        from_locked_world_template : bool = stream.read_bool() ? ; let
        msa_gamertags_only : bool = stream.read_bool() ? ; let
        from_world_template : bool = stream.read_bool() ? ; let
        world_template_option_locked : bool = stream.read_bool() ? ; let
        only_spawn_v1_villagers : bool = stream.read_bool() ? ; let
        persona_disabled : bool = stream.read_bool() ? ; let
        custom_skins_disabled : bool = stream.read_bool() ? ; let
        emote_chat_muted : bool = stream.read_bool() ? ; let game_version :
        String = stream.read_string() ? ; let limited_world_width : LI32 =
        stream.read_li32() ? ; let limited_world_length : LI32 =
        stream.read_li32() ? ; let new_nether : bool = stream.read_bool() ? ;
        let edu_resource_uri : EducationSharedResource =
        EducationSharedResource :: deserialize(& mut stream) ? ; let
        experimental_gameplay_override : bool = stream.read_bool() ? ; let
        chat_restriction_level : u8 = stream.read_u8() ? ; let
        disable_player_interactions : bool = stream.read_bool() ? ; let
        level_id : String = stream.read_string() ? ; let world_name : String =
        stream.read_string() ? ; let premium_world_template_id : String =
        stream.read_string() ? ; let is_trial : bool = stream.read_bool() ? ;
        let movement_authority : ZigZag = stream.read_zigzag() ? ; let
        rewind_history_size : ZigZag = stream.read_zigzag() ? ; let
        server_authoritive_block_breaking : bool = stream.read_bool() ? ; let
        current_tick : LI64 = stream.read_li64() ? ; let enchantment_seed :
        ZigZag = stream.read_zigzag() ? ; let block_properties_len =
        stream.read_varint() ? ; let mut block_properties : Vec <
        BlockProperty > = Vec :: new() ; for _ in 0 .. block_properties_len
        {
            block_properties.push(BlockProperty :: deserialize(& mut stream)
            ?) ;
        } let item_states_len = stream.read_varint() ? ; let mut item_states :
        Vec < ItemState > = Vec :: new() ; for _ in 0 .. item_states_len
        { item_states.push(ItemState :: deserialize(& mut stream) ?) ; } let
        multiplayer_correlation_id : String = stream.read_string() ? ; let
        server_authoritative_inventory : bool = stream.read_bool() ? ; let
        engine : String = stream.read_string() ? ; let property_data : NBT =
        NBT :: deserialize(& mut stream) ? ; let block_pallette_checksum :
        LU64 = stream.read_lu64() ? ; let world_template_id : UUID =
        stream.read_uuid() ? ; let client_side_generation : bool =
        stream.read_bool() ? ; let block_network_ids_are_hashes : bool =
        stream.read_bool() ? ; let server_controlled_sound : bool =
        stream.read_bool() ? ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
} impl crate :: packets :: prelude :: PacketConversion for StartGamePacket
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("entityId", self.entity_id.to_owned()) ? ;
        object.set_named_property("runtimeEntityId",
        self.runtime_entity_id.to_owned()) ? ;
        object.set_named_property("playerGamemode",
        self.player_gamemode.to_owned()) ? ;
        object.set_named_property("playerPosition",
        self.player_position.to_object(env) ?) ? ;
        object.set_named_property("playerRotation",
        self.player_rotation.to_object(env) ?) ? ;
        object.set_named_property("seed", self.seed.to_owned()) ? ;
        object.set_named_property("biomeType", self.biome_type.to_owned()) ? ;
        object.set_named_property("biomeName", self.biome_name.to_owned()) ? ;
        object.set_named_property("dimension", self.dimension.to_owned()) ? ;
        object.set_named_property("generator", self.generator.to_owned()) ? ;
        object.set_named_property("worldGamemode",
        self.world_gamemode.to_owned()) ? ;
        object.set_named_property("difficulty", self.difficulty.to_owned()) ?
        ;
        object.set_named_property("spawnPosition",
        self.spawn_position.to_object(env) ?) ? ;
        object.set_named_property("achievementsDisabled",
        self.achievements_disabled.to_owned()) ? ;
        object.set_named_property("editorWorld", self.editor_world.to_owned())
        ? ;
        object.set_named_property("createdInEditor",
        self.created_in_editor.to_owned()) ? ;
        object.set_named_property("dayCycleStopTime",
        self.day_cycle_stop_time.to_owned()) ? ;
        object.set_named_property("eduOffer", self.edu_offer.to_owned()) ? ;
        object.set_named_property("eduFeaturesEnabled",
        self.edu_features_enabled.to_owned()) ? ;
        object.set_named_property("eduProductId",
        self.edu_product_id.to_owned()) ? ;
        object.set_named_property("rainLevel", self.rain_level.to_owned()) ? ;
        object.set_named_property("lightningLevel",
        self.lightning_level.to_owned()) ? ;
        object.set_named_property("confirmedPlatformLockedContent",
        self.confirmed_platform_locked_content.to_owned()) ? ;
        object.set_named_property("multiplayerGame",
        self.multiplayer_game.to_owned()) ? ;
        object.set_named_property("broadcastToLan",
        self.broadcast_to_lan.to_owned()) ? ;
        object.set_named_property("xblBroadcastMode",
        self.xbl_broadcast_mode.to_owned()) ? ;
        object.set_named_property("platformBroadcastMode",
        self.platform_broadcast_mode.to_owned()) ? ;
        object.set_named_property("enableCommands",
        self.enable_commands.to_owned()) ? ;
        object.set_named_property("texturePacksRequired",
        self.texture_packs_required.to_owned()) ? ; let mut gamerules =
        env.create_array_with_length(self.gamerules.len()) ? ; for(i, item) in
        self.gamerules.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            gamerules.set_element(i as u32, obj) ? ;
        } object.set_named_property("gamerules", gamerules) ? ; let mut
        experiments = env.create_array_with_length(self.experiments.len()) ? ;
        for(i, item) in self.experiments.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            experiments.set_element(i as u32, obj) ? ;
        } object.set_named_property("experiments", experiments) ? ;
        object.set_named_property("experimentsPreviouslyUsed",
        self.experiments_previously_used.to_owned()) ? ;
        object.set_named_property("bonusChest", self.bonus_chest.to_owned()) ?
        ; object.set_named_property("mapEnabled", self.map_enabled.to_owned())
        ? ;
        object.set_named_property("permissionLevel",
        self.permission_level.to_owned()) ? ;
        object.set_named_property("serverChunkTickRange",
        self.server_chunk_tick_range.to_owned()) ? ;
        object.set_named_property("hasLockedBehaviourPacks",
        self.has_locked_behaviour_packs.to_owned()) ? ;
        object.set_named_property("hasLockedResourcePacks",
        self.has_locked_resource_packs.to_owned()) ? ;
        object.set_named_property("fromLockedWorldTemplate",
        self.from_locked_world_template.to_owned()) ? ;
        object.set_named_property("msaGamertagsOnly",
        self.msa_gamertags_only.to_owned()) ? ;
        object.set_named_property("fromWorldTemplate",
        self.from_world_template.to_owned()) ? ;
        object.set_named_property("worldTemplateOptionLocked",
        self.world_template_option_locked.to_owned()) ? ;
        object.set_named_property("onlySpawnV1Villagers",
        self.only_spawn_v1_villagers.to_owned()) ? ;
        object.set_named_property("personaDisabled",
        self.persona_disabled.to_owned()) ? ;
        object.set_named_property("customSkinsDisabled",
        self.custom_skins_disabled.to_owned()) ? ;
        object.set_named_property("emoteChatMuted",
        self.emote_chat_muted.to_owned()) ? ;
        object.set_named_property("gameVersion", self.game_version.to_owned())
        ? ;
        object.set_named_property("limitedWorldWidth",
        self.limited_world_width.to_owned()) ? ;
        object.set_named_property("limitedWorldLength",
        self.limited_world_length.to_owned()) ? ;
        object.set_named_property("newNether", self.new_nether.to_owned()) ? ;
        object.set_named_property("eduResourceUri",
        self.edu_resource_uri.to_object(env) ?) ? ;
        object.set_named_property("experimentalGameplayOverride",
        self.experimental_gameplay_override.to_owned()) ? ;
        object.set_named_property("chatRestrictionLevel",
        self.chat_restriction_level.to_owned()) ? ;
        object.set_named_property("disablePlayerInteractions",
        self.disable_player_interactions.to_owned()) ? ;
        object.set_named_property("levelId", self.level_id.to_owned()) ? ;
        object.set_named_property("worldName", self.world_name.to_owned()) ? ;
        object.set_named_property("premiumWorldTemplateId",
        self.premium_world_template_id.to_owned()) ? ;
        object.set_named_property("isTrial", self.is_trial.to_owned()) ? ;
        object.set_named_property("movementAuthority",
        self.movement_authority.to_owned()) ? ;
        object.set_named_property("rewindHistorySize",
        self.rewind_history_size.to_owned()) ? ;
        object.set_named_property("serverAuthoritiveBlockBreaking",
        self.server_authoritive_block_breaking.to_owned()) ? ;
        object.set_named_property("currentTick", self.current_tick.to_owned())
        ? ;
        object.set_named_property("enchantmentSeed",
        self.enchantment_seed.to_owned()) ? ; let mut block_properties =
        env.create_array_with_length(self.block_properties.len()) ? ;
        for(i, item) in self.block_properties.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            block_properties.set_element(i as u32, obj) ? ;
        } object.set_named_property("blockProperties", block_properties) ? ;
        let mut item_states =
        env.create_array_with_length(self.item_states.len()) ? ; for(i, item)
        in self.item_states.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            item_states.set_element(i as u32, obj) ? ;
        } object.set_named_property("itemStates", item_states) ? ;
        object.set_named_property("multiplayerCorrelationId",
        self.multiplayer_correlation_id.to_owned()) ? ;
        object.set_named_property("serverAuthoritativeInventory",
        self.server_authoritative_inventory.to_owned()) ? ;
        object.set_named_property("engine", self.engine.to_owned()) ? ;
        object.set_named_property("propertyData",
        self.property_data.to_owned()) ? ;
        object.set_named_property("blockPalletteChecksum",
        self.block_pallette_checksum.to_owned()) ? ;
        object.set_named_property("worldTemplateId",
        self.world_template_id.to_owned()) ? ;
        object.set_named_property("clientSideGeneration",
        self.client_side_generation.to_owned()) ? ;
        object.set_named_property("blockNetworkIdsAreHashes",
        self.block_network_ids_are_hashes.to_owned()) ? ;
        object.set_named_property("serverControlledSound",
        self.server_controlled_sound.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let entity_id : ZigZong = match data.get_named_property("entityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "entityId",
            stringify! (ZigZong), err),))
        } ; let runtime_entity_id : VarLong = match
        data.get_named_property("runtimeEntityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "runtimeEntityId", stringify! (VarLong), err),))
        } ; let player_gamemode : ZigZag = match
        data.get_named_property("playerGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerGamemode", stringify! (ZigZag), err),))
        } ; let player_position : Vec3f = match Vec3f ::
        from_object(data.get_named_property("playerPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerPosition", stringify! (Vec3f), err),))
        } ; let player_rotation : Vec2f = match Vec2f ::
        from_object(data.get_named_property("playerRotation") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerRotation", stringify! (Vec2f), err),))
        } ; let seed : LU64 = match data.get_named_property("seed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "seed",
            stringify! (LU64), err),))
        } ; let biome_type : LI16 = match data.get_named_property("biomeType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeType",
            stringify! (LI16), err),))
        } ; let biome_name : String = match
        data.get_named_property("biomeName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeName",
            stringify! (String), err),))
        } ; let dimension : ZigZag = match
        data.get_named_property("dimension")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "dimension",
            stringify! (ZigZag), err),))
        } ; let generator : ZigZag = match
        data.get_named_property("generator")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "generator",
            stringify! (ZigZag), err),))
        } ; let world_gamemode : ZigZag = match
        data.get_named_property("worldGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldGamemode",
            stringify! (ZigZag), err),))
        } ; let difficulty : ZigZag = match
        data.get_named_property("difficulty")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "difficulty",
            stringify! (ZigZag), err),))
        } ; let spawn_position : BlockCoordinates = match BlockCoordinates ::
        from_object(data.get_named_property("spawnPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "spawnPosition",
            stringify! (BlockCoordinates), err),))
        } ; let achievements_disabled : bool = match
        data.get_named_property("achievementsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "achievementsDisabled", stringify! (bool), err),))
        } ; let editor_world : bool = match
        data.get_named_property("editorWorld")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editorWorld",
            stringify! (bool), err),))
        } ; let created_in_editor : bool = match
        data.get_named_property("createdInEditor")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "createdInEditor", stringify! (bool), err),))
        } ; let day_cycle_stop_time : ZigZag = match
        data.get_named_property("dayCycleStopTime")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "dayCycleStopTime", stringify! (ZigZag), err),))
        } ; let edu_offer : ZigZag = match data.get_named_property("eduOffer")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduOffer",
            stringify! (ZigZag), err),))
        } ; let edu_features_enabled : bool = match
        data.get_named_property("eduFeaturesEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduFeaturesEnabled", stringify! (bool), err),))
        } ; let edu_product_id : String = match
        data.get_named_property("eduProductId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduProductId",
            stringify! (String), err),))
        } ; let rain_level : LF32 = match data.get_named_property("rainLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "rainLevel",
            stringify! (LF32), err),))
        } ; let lightning_level : LF32 = match
        data.get_named_property("lightningLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "lightningLevel", stringify! (LF32), err),))
        } ; let confirmed_platform_locked_content : bool = match
        data.get_named_property("confirmedPlatformLockedContent")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "confirmedPlatformLockedContent", stringify! (bool), err),))
        } ; let multiplayer_game : bool = match
        data.get_named_property("multiplayerGame")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerGame", stringify! (bool), err),))
        } ; let broadcast_to_lan : bool = match
        data.get_named_property("broadcastToLan")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "broadcastToLan", stringify! (bool), err),))
        } ; let xbl_broadcast_mode : VarInt = match
        data.get_named_property("xblBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "xblBroadcastMode", stringify! (VarInt), err),))
        } ; let platform_broadcast_mode : VarInt = match
        data.get_named_property("platformBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "platformBroadcastMode", stringify! (VarInt), err),))
        } ; let enable_commands : bool = match
        data.get_named_property("enableCommands")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enableCommands", stringify! (bool), err),))
        } ; let texture_packs_required : bool = match
        data.get_named_property("texturePacksRequired")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "texturePacksRequired", stringify! (bool), err),))
        } ; let gamerules_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("gamerules")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gamerules",
            stringify! (Vec < GameRule >), err),))
        } ; let mut gamerules : Vec < GameRule > = Vec :: new() ; for item in
        gamerules_napi { gamerules.push(GameRule :: from_object(item) ?) ; }
        let experiments_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("experiments")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "experiments",
            stringify! (Vec < Experiment >), err),))
        } ; let mut experiments : Vec < Experiment > = Vec :: new() ; for item
        in experiments_napi
        { experiments.push(Experiment :: from_object(item) ?) ; } let
        experiments_previously_used : bool = match
        data.get_named_property("experimentsPreviouslyUsed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentsPreviouslyUsed", stringify! (bool), err),))
        } ; let bonus_chest : bool = match
        data.get_named_property("bonusChest")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "bonusChest",
            stringify! (bool), err),))
        } ; let map_enabled : bool = match
        data.get_named_property("mapEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "mapEnabled",
            stringify! (bool), err),))
        } ; let permission_level : u8 = match
        data.get_named_property("permissionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "permissionLevel", stringify! (u8), err),))
        } ; let server_chunk_tick_range : LI32 = match
        data.get_named_property("serverChunkTickRange")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverChunkTickRange", stringify! (LI32), err),))
        } ; let has_locked_behaviour_packs : bool = match
        data.get_named_property("hasLockedBehaviourPacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedBehaviourPacks", stringify! (bool), err),))
        } ; let has_locked_resource_packs : bool = match
        data.get_named_property("hasLockedResourcePacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedResourcePacks", stringify! (bool), err),))
        } ; let from_locked_world_template : bool = match
        data.get_named_property("fromLockedWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromLockedWorldTemplate", stringify! (bool), err),))
        } ; let msa_gamertags_only : bool = match
        data.get_named_property("msaGamertagsOnly")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "msaGamertagsOnly", stringify! (bool), err),))
        } ; let from_world_template : bool = match
        data.get_named_property("fromWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromWorldTemplate", stringify! (bool), err),))
        } ; let world_template_option_locked : bool = match
        data.get_named_property("worldTemplateOptionLocked")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateOptionLocked", stringify! (bool), err),))
        } ; let only_spawn_v1_villagers : bool = match
        data.get_named_property("onlySpawnV1Villagers")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "onlySpawnV1Villagers", stringify! (bool), err),))
        } ; let persona_disabled : bool = match
        data.get_named_property("personaDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "personaDisabled", stringify! (bool), err),))
        } ; let custom_skins_disabled : bool = match
        data.get_named_property("customSkinsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "customSkinsDisabled", stringify! (bool), err),))
        } ; let emote_chat_muted : bool = match
        data.get_named_property("emoteChatMuted")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "emoteChatMuted", stringify! (bool), err),))
        } ; let game_version : String = match
        data.get_named_property("gameVersion")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gameVersion",
            stringify! (String), err),))
        } ; let limited_world_width : LI32 = match
        data.get_named_property("limitedWorldWidth")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldWidth", stringify! (LI32), err),))
        } ; let limited_world_length : LI32 = match
        data.get_named_property("limitedWorldLength")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldLength", stringify! (LI32), err),))
        } ; let new_nether : bool = match data.get_named_property("newNether")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "newNether",
            stringify! (bool), err),))
        } ; let edu_resource_uri : EducationSharedResource = match
        EducationSharedResource ::
        from_object(data.get_named_property("eduResourceUri") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduResourceUri", stringify! (EducationSharedResource), err),))
        } ; let experimental_gameplay_override : bool = match
        data.get_named_property("experimentalGameplayOverride")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentalGameplayOverride", stringify! (bool), err),))
        } ; let chat_restriction_level : u8 = match
        data.get_named_property("chatRestrictionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "chatRestrictionLevel", stringify! (u8), err),))
        } ; let disable_player_interactions : bool = match
        data.get_named_property("disablePlayerInteractions")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "disablePlayerInteractions", stringify! (bool), err),))
        } ; let level_id : String = match data.get_named_property("levelId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "levelId",
            stringify! (String), err),))
        } ; let world_name : String = match
        data.get_named_property("worldName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldName",
            stringify! (String), err),))
        } ; let premium_world_template_id : String = match
        data.get_named_property("premiumWorldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "premiumWorldTemplateId", stringify! (String), err),))
        } ; let is_trial : bool = match data.get_named_property("isTrial")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "isTrial",
            stringify! (bool), err),))
        } ; let movement_authority : ZigZag = match
        data.get_named_property("movementAuthority")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "movementAuthority", stringify! (ZigZag), err),))
        } ; let rewind_history_size : ZigZag = match
        data.get_named_property("rewindHistorySize")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "rewindHistorySize", stringify! (ZigZag), err),))
        } ; let server_authoritive_block_breaking : bool = match
        data.get_named_property("serverAuthoritiveBlockBreaking")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritiveBlockBreaking", stringify! (bool), err),))
        } ; let current_tick : LI64 = match
        data.get_named_property("currentTick")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "currentTick",
            stringify! (LI64), err),))
        } ; let enchantment_seed : ZigZag = match
        data.get_named_property("enchantmentSeed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enchantmentSeed", stringify! (ZigZag), err),))
        } ; let block_properties_napi : Vec < napi :: bindgen_prelude ::
        Object > = match data.get_named_property("blockProperties")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockProperties", stringify! (Vec < BlockProperty >), err),))
        } ; let mut block_properties : Vec < BlockProperty > = Vec :: new() ;
        for item in block_properties_napi
        { block_properties.push(BlockProperty :: from_object(item) ?) ; } let
        item_states_napi : Vec < napi :: bindgen_prelude :: Object > = match
        data.get_named_property("itemStates")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "itemStates",
            stringify! (Vec < ItemState >), err),))
        } ; let mut item_states : Vec < ItemState > = Vec :: new() ; for item
        in item_states_napi
        { item_states.push(ItemState :: from_object(item) ?) ; } let
        multiplayer_correlation_id : String = match
        data.get_named_property("multiplayerCorrelationId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerCorrelationId", stringify! (String), err),))
        } ; let server_authoritative_inventory : bool = match
        data.get_named_property("serverAuthoritativeInventory")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritativeInventory", stringify! (bool), err),))
        } ; let engine : String = match data.get_named_property("engine")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "engine",
            stringify! (String), err),))
        } ; let property_data : NBT = match
        data.get_named_property("propertyData")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "propertyData",
            stringify! (NBT), err),))
        } ; let block_pallette_checksum : LU64 = match
        data.get_named_property("blockPalletteChecksum")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockPalletteChecksum", stringify! (LU64), err),))
        } ; let world_template_id : UUID = match
        data.get_named_property("worldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateId", stringify! (UUID), err),))
        } ; let client_side_generation : bool = match
        data.get_named_property("clientSideGeneration")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "clientSideGeneration", stringify! (bool), err),))
        } ; let block_network_ids_are_hashes : bool = match
        data.get_named_property("blockNetworkIdsAreHashes")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockNetworkIdsAreHashes", stringify! (bool), err),))
        } ; let server_controlled_sound : bool = match
        data.get_named_property("serverControlledSound")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverControlledSound", stringify! (bool), err),))
        } ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
StartGamePacket
{
    pub entity_id : ZigZong, pub runtime_entity_id : VarLong,
    #[napi(ts_type = "GameMode")] pub player_gamemode : ZigZag, pub
    player_position : Vec3f, pub player_rotation : Vec2f, pub seed : LU64, pub
    biome_type : LI16, pub biome_name : String, #[napi(ts_type = "Dimension")]
    pub dimension : ZigZag, pub generator : ZigZag,
    #[napi(ts_type = "GameMode")] pub world_gamemode : ZigZag, pub difficulty
    : ZigZag, pub spawn_position : BlockCoordinates, pub achievements_disabled
    : bool, pub editor_world : bool, pub created_in_editor : bool, pub
    day_cycle_stop_time : ZigZag, pub edu_offer : ZigZag, pub
    edu_features_enabled : bool, pub edu_product_id : String, pub rain_level :
    LF32, pub lightning_level : LF32, pub confirmed_platform_locked_content :
    bool, pub multiplayer_game : bool, pub broadcast_to_lan : bool, pub
    xbl_broadcast_mode : VarInt, pub platform_broadcast_mode : VarInt, pub
    enable_commands : bool, pub texture_packs_required : bool, #[VarInt] pub
    gamerules : Vec < GameRule >, #[LI32] pub experiments : Vec < Experiment
    >, pub experiments_previously_used : bool, pub bonus_chest : bool, pub
    map_enabled : bool, #[napi(ts_type = "PermissionLevel")] pub
    permission_level : u8, pub server_chunk_tick_range : LI32, pub
    has_locked_behaviour_packs : bool, pub has_locked_resource_packs : bool,
    pub from_locked_world_template : bool, pub msa_gamertags_only : bool, pub
    from_world_template : bool, pub world_template_option_locked : bool, pub
    only_spawn_v1_villagers : bool, pub persona_disabled : bool, pub
    custom_skins_disabled : bool, pub emote_chat_muted : bool, pub
    game_version : String, pub limited_world_width : LI32, pub
    limited_world_length : LI32, pub new_nether : bool, pub edu_resource_uri :
    EducationSharedResource, pub experimental_gameplay_override : bool,
    #[napi(ts_type = "ChatRestrictionLevel")] pub chat_restriction_level : u8,
    pub disable_player_interactions : bool, pub level_id : String, pub
    world_name : String, pub premium_world_template_id : String, pub is_trial
    : bool, #[napi(ts_type = "MovementAuthority")] pub movement_authority :
    ZigZag, pub rewind_history_size : ZigZag, pub
    server_authoritive_block_breaking : bool, pub current_tick : LI64, pub
    enchantment_seed : ZigZag, #[VarInt] pub block_properties : Vec <
    BlockProperty >, #[VarInt] pub item_states : Vec < ItemState >, pub
    multiplayer_correlation_id : String, pub server_authoritative_inventory :
    bool, pub engine : String, pub property_data : NBT, pub
    block_pallette_checksum : LU64, pub world_template_id : UUID, pub
    client_side_generation : bool, pub block_network_ids_are_hashes : bool,
    pub server_controlled_sound : bool
} impl StartGamePacket
{ pub const ID : crate :: binary :: prelude :: VarInt = 11u32 ; } impl crate
:: packets :: prelude :: PacketSerialization for StartGamePacket
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < napi ::
    bindgen_prelude :: Buffer >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_varint(StartGamePacket :: ID) ? ;
        stream.write_zigzong(self.entity_id.to_owned()) ? ;
        stream.write_varlong(self.runtime_entity_id.to_owned()) ? ;
        stream.write_zigzag(self.player_gamemode.to_owned()) ? ;
        stream.append(& mut self.player_position.serialize() ?) ;
        stream.append(& mut self.player_rotation.serialize() ?) ;
        stream.write_lu64(self.seed.to_owned()) ? ;
        stream.write_li16(self.biome_type.to_owned()) ? ;
        stream.write_string(self.biome_name.to_owned()) ? ;
        stream.write_zigzag(self.dimension.to_owned()) ? ;
        stream.write_zigzag(self.generator.to_owned()) ? ;
        stream.write_zigzag(self.world_gamemode.to_owned()) ? ;
        stream.write_zigzag(self.difficulty.to_owned()) ? ;
        stream.append(& mut self.spawn_position.serialize() ?) ;
        stream.write_bool(self.achievements_disabled.to_owned()) ? ;
        stream.write_bool(self.editor_world.to_owned()) ? ;
        stream.write_bool(self.created_in_editor.to_owned()) ? ;
        stream.write_zigzag(self.day_cycle_stop_time.to_owned()) ? ;
        stream.write_zigzag(self.edu_offer.to_owned()) ? ;
        stream.write_bool(self.edu_features_enabled.to_owned()) ? ;
        stream.write_string(self.edu_product_id.to_owned()) ? ;
        stream.write_lf32(self.rain_level.to_owned()) ? ;
        stream.write_lf32(self.lightning_level.to_owned()) ? ;
        stream.write_bool(self.confirmed_platform_locked_content.to_owned()) ?
        ; stream.write_bool(self.multiplayer_game.to_owned()) ? ;
        stream.write_bool(self.broadcast_to_lan.to_owned()) ? ;
        stream.write_varint(self.xbl_broadcast_mode.to_owned()) ? ;
        stream.write_varint(self.platform_broadcast_mode.to_owned()) ? ;
        stream.write_bool(self.enable_commands.to_owned()) ? ;
        stream.write_bool(self.texture_packs_required.to_owned()) ? ; let
        gamerules_len = self.gamerules.len() ;
        stream.write_varint(gamerules_len as VarInt) ? ; for i in 0 ..
        gamerules_len
        {
            let mut bin = self.gamerules [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let experiments_len = self.experiments.len() ;
        stream.write_li32(experiments_len as LI32) ? ; for i in 0 ..
        experiments_len
        {
            let mut bin = self.experiments [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_bool(self.experiments_previously_used.to_owned()) ? ;
        stream.write_bool(self.bonus_chest.to_owned()) ? ;
        stream.write_bool(self.map_enabled.to_owned()) ? ;
        stream.write_u8(self.permission_level.to_owned()) ? ;
        stream.write_li32(self.server_chunk_tick_range.to_owned()) ? ;
        stream.write_bool(self.has_locked_behaviour_packs.to_owned()) ? ;
        stream.write_bool(self.has_locked_resource_packs.to_owned()) ? ;
        stream.write_bool(self.from_locked_world_template.to_owned()) ? ;
        stream.write_bool(self.msa_gamertags_only.to_owned()) ? ;
        stream.write_bool(self.from_world_template.to_owned()) ? ;
        stream.write_bool(self.world_template_option_locked.to_owned()) ? ;
        stream.write_bool(self.only_spawn_v1_villagers.to_owned()) ? ;
        stream.write_bool(self.persona_disabled.to_owned()) ? ;
        stream.write_bool(self.custom_skins_disabled.to_owned()) ? ;
        stream.write_bool(self.emote_chat_muted.to_owned()) ? ;
        stream.write_string(self.game_version.to_owned()) ? ;
        stream.write_li32(self.limited_world_width.to_owned()) ? ;
        stream.write_li32(self.limited_world_length.to_owned()) ? ;
        stream.write_bool(self.new_nether.to_owned()) ? ;
        stream.append(& mut self.edu_resource_uri.serialize() ?) ;
        stream.write_bool(self.experimental_gameplay_override.to_owned()) ? ;
        stream.write_u8(self.chat_restriction_level.to_owned()) ? ;
        stream.write_bool(self.disable_player_interactions.to_owned()) ? ;
        stream.write_string(self.level_id.to_owned()) ? ;
        stream.write_string(self.world_name.to_owned()) ? ;
        stream.write_string(self.premium_world_template_id.to_owned()) ? ;
        stream.write_bool(self.is_trial.to_owned()) ? ;
        stream.write_zigzag(self.movement_authority.to_owned()) ? ;
        stream.write_zigzag(self.rewind_history_size.to_owned()) ? ;
        stream.write_bool(self.server_authoritive_block_breaking.to_owned()) ?
        ; stream.write_li64(self.current_tick.to_owned()) ? ;
        stream.write_zigzag(self.enchantment_seed.to_owned()) ? ; let
        block_properties_len = self.block_properties.len() ;
        stream.write_varint(block_properties_len as VarInt) ? ; for i in 0 ..
        block_properties_len
        {
            let mut bin = self.block_properties [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let item_states_len = self.item_states.len() ;
        stream.write_varint(item_states_len as VarInt) ? ; for i in 0 ..
        item_states_len
        {
            let mut bin = self.item_states [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_string(self.multiplayer_correlation_id.to_owned()) ? ;
        stream.write_bool(self.server_authoritative_inventory.to_owned()) ? ;
        stream.write_string(self.engine.to_owned()) ? ;
        stream.append(& mut self.property_data.serialize() ?) ;
        stream.write_lu64(self.block_pallette_checksum.to_owned()) ? ;
        stream.write_uuid(self.world_template_id.to_owned()) ? ;
        stream.write_bool(self.client_side_generation.to_owned()) ? ;
        stream.write_bool(self.block_network_ids_are_hashes.to_owned()) ? ;
        stream.write_bool(self.server_controlled_sound.to_owned()) ? ;
        Ok(stream.data.into())
    } fn deserialize(data : napi :: bindgen_prelude :: Buffer) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let mut stream = crate :: binary :: BinaryStream :: from(data.into())
        ; let _id = stream.read_varint() ? ; let entity_id : ZigZong =
        stream.read_zigzong() ? ; let runtime_entity_id : VarLong =
        stream.read_varlong() ? ; let player_gamemode : ZigZag =
        stream.read_zigzag() ? ; let player_position : Vec3f = Vec3f ::
        deserialize(& mut stream) ? ; let player_rotation : Vec2f = Vec2f ::
        deserialize(& mut stream) ? ; let seed : LU64 = stream.read_lu64() ? ;
        let biome_type : LI16 = stream.read_li16() ? ; let biome_name : String
        = stream.read_string() ? ; let dimension : ZigZag =
        stream.read_zigzag() ? ; let generator : ZigZag = stream.read_zigzag()
        ? ; let world_gamemode : ZigZag = stream.read_zigzag() ? ; let
        difficulty : ZigZag = stream.read_zigzag() ? ; let spawn_position :
        BlockCoordinates = BlockCoordinates :: deserialize(& mut stream) ? ;
        let achievements_disabled : bool = stream.read_bool() ? ; let
        editor_world : bool = stream.read_bool() ? ; let created_in_editor :
        bool = stream.read_bool() ? ; let day_cycle_stop_time : ZigZag =
        stream.read_zigzag() ? ; let edu_offer : ZigZag = stream.read_zigzag()
        ? ; let edu_features_enabled : bool = stream.read_bool() ? ; let
        edu_product_id : String = stream.read_string() ? ; let rain_level :
        LF32 = stream.read_lf32() ? ; let lightning_level : LF32 =
        stream.read_lf32() ? ; let confirmed_platform_locked_content : bool =
        stream.read_bool() ? ; let multiplayer_game : bool =
        stream.read_bool() ? ; let broadcast_to_lan : bool =
        stream.read_bool() ? ; let xbl_broadcast_mode : VarInt =
        stream.read_varint() ? ; let platform_broadcast_mode : VarInt =
        stream.read_varint() ? ; let enable_commands : bool =
        stream.read_bool() ? ; let texture_packs_required : bool =
        stream.read_bool() ? ; let gamerules_len = stream.read_varint() ? ;
        let mut gamerules : Vec < GameRule > = Vec :: new() ; for _ in 0 ..
        gamerules_len
        { gamerules.push(GameRule :: deserialize(& mut stream) ?) ; } let
        experiments_len = stream.read_li32() ? ; let mut experiments : Vec <
        Experiment > = Vec :: new() ; for _ in 0 .. experiments_len
        { experiments.push(Experiment :: deserialize(& mut stream) ?) ; } let
        experiments_previously_used : bool = stream.read_bool() ? ; let
        bonus_chest : bool = stream.read_bool() ? ; let map_enabled : bool =
        stream.read_bool() ? ; let permission_level : u8 = stream.read_u8() ?
        ; let server_chunk_tick_range : LI32 = stream.read_li32() ? ; let
        has_locked_behaviour_packs : bool = stream.read_bool() ? ; let
        has_locked_resource_packs : bool = stream.read_bool() ? ; let
        from_locked_world_template : bool = stream.read_bool() ? ; let
        msa_gamertags_only : bool = stream.read_bool() ? ; let
        from_world_template : bool = stream.read_bool() ? ; let
        world_template_option_locked : bool = stream.read_bool() ? ; let
        only_spawn_v1_villagers : bool = stream.read_bool() ? ; let
        persona_disabled : bool = stream.read_bool() ? ; let
        custom_skins_disabled : bool = stream.read_bool() ? ; let
        emote_chat_muted : bool = stream.read_bool() ? ; let game_version :
        String = stream.read_string() ? ; let limited_world_width : LI32 =
        stream.read_li32() ? ; let limited_world_length : LI32 =
        stream.read_li32() ? ; let new_nether : bool = stream.read_bool() ? ;
        let edu_resource_uri : EducationSharedResource =
        EducationSharedResource :: deserialize(& mut stream) ? ; let
        experimental_gameplay_override : bool = stream.read_bool() ? ; let
        chat_restriction_level : u8 = stream.read_u8() ? ; let
        disable_player_interactions : bool = stream.read_bool() ? ; let
        level_id : String = stream.read_string() ? ; let world_name : String =
        stream.read_string() ? ; let premium_world_template_id : String =
        stream.read_string() ? ; let is_trial : bool = stream.read_bool() ? ;
        let movement_authority : ZigZag = stream.read_zigzag() ? ; let
        rewind_history_size : ZigZag = stream.read_zigzag() ? ; let
        server_authoritive_block_breaking : bool = stream.read_bool() ? ; let
        current_tick : LI64 = stream.read_li64() ? ; let enchantment_seed :
        ZigZag = stream.read_zigzag() ? ; let block_properties_len =
        stream.read_varint() ? ; let mut block_properties : Vec <
        BlockProperty > = Vec :: new() ; for _ in 0 .. block_properties_len
        {
            block_properties.push(BlockProperty :: deserialize(& mut stream)
            ?) ;
        } let item_states_len = stream.read_varint() ? ; let mut item_states :
        Vec < ItemState > = Vec :: new() ; for _ in 0 .. item_states_len
        { item_states.push(ItemState :: deserialize(& mut stream) ?) ; } let
        multiplayer_correlation_id : String = stream.read_string() ? ; let
        server_authoritative_inventory : bool = stream.read_bool() ? ; let
        engine : String = stream.read_string() ? ; let property_data : NBT =
        NBT :: deserialize(& mut stream) ? ; let block_pallette_checksum :
        LU64 = stream.read_lu64() ? ; let world_template_id : UUID =
        stream.read_uuid() ? ; let client_side_generation : bool =
        stream.read_bool() ? ; let block_network_ids_are_hashes : bool =
        stream.read_bool() ? ; let server_controlled_sound : bool =
        stream.read_bool() ? ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
} impl crate :: packets :: prelude :: PacketConversion for StartGamePacket
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("entityId", self.entity_id.to_owned()) ? ;
        object.set_named_property("runtimeEntityId",
        self.runtime_entity_id.to_owned()) ? ;
        object.set_named_property("playerGamemode",
        self.player_gamemode.to_owned()) ? ;
        object.set_named_property("playerPosition",
        self.player_position.to_object(env) ?) ? ;
        object.set_named_property("playerRotation",
        self.player_rotation.to_object(env) ?) ? ;
        object.set_named_property("seed", self.seed.to_owned()) ? ;
        object.set_named_property("biomeType", self.biome_type.to_owned()) ? ;
        object.set_named_property("biomeName", self.biome_name.to_owned()) ? ;
        object.set_named_property("dimension", self.dimension.to_owned()) ? ;
        object.set_named_property("generator", self.generator.to_owned()) ? ;
        object.set_named_property("worldGamemode",
        self.world_gamemode.to_owned()) ? ;
        object.set_named_property("difficulty", self.difficulty.to_owned()) ?
        ;
        object.set_named_property("spawnPosition",
        self.spawn_position.to_object(env) ?) ? ;
        object.set_named_property("achievementsDisabled",
        self.achievements_disabled.to_owned()) ? ;
        object.set_named_property("editorWorld", self.editor_world.to_owned())
        ? ;
        object.set_named_property("createdInEditor",
        self.created_in_editor.to_owned()) ? ;
        object.set_named_property("dayCycleStopTime",
        self.day_cycle_stop_time.to_owned()) ? ;
        object.set_named_property("eduOffer", self.edu_offer.to_owned()) ? ;
        object.set_named_property("eduFeaturesEnabled",
        self.edu_features_enabled.to_owned()) ? ;
        object.set_named_property("eduProductId",
        self.edu_product_id.to_owned()) ? ;
        object.set_named_property("rainLevel", self.rain_level.to_owned()) ? ;
        object.set_named_property("lightningLevel",
        self.lightning_level.to_owned()) ? ;
        object.set_named_property("confirmedPlatformLockedContent",
        self.confirmed_platform_locked_content.to_owned()) ? ;
        object.set_named_property("multiplayerGame",
        self.multiplayer_game.to_owned()) ? ;
        object.set_named_property("broadcastToLan",
        self.broadcast_to_lan.to_owned()) ? ;
        object.set_named_property("xblBroadcastMode",
        self.xbl_broadcast_mode.to_owned()) ? ;
        object.set_named_property("platformBroadcastMode",
        self.platform_broadcast_mode.to_owned()) ? ;
        object.set_named_property("enableCommands",
        self.enable_commands.to_owned()) ? ;
        object.set_named_property("texturePacksRequired",
        self.texture_packs_required.to_owned()) ? ; let mut gamerules =
        env.create_array_with_length(self.gamerules.len()) ? ; for(i, item) in
        self.gamerules.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            gamerules.set_element(i as u32, obj) ? ;
        } object.set_named_property("gamerules", gamerules) ? ; let mut
        experiments = env.create_array_with_length(self.experiments.len()) ? ;
        for(i, item) in self.experiments.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            experiments.set_element(i as u32, obj) ? ;
        } object.set_named_property("experiments", experiments) ? ;
        object.set_named_property("experimentsPreviouslyUsed",
        self.experiments_previously_used.to_owned()) ? ;
        object.set_named_property("bonusChest", self.bonus_chest.to_owned()) ?
        ; object.set_named_property("mapEnabled", self.map_enabled.to_owned())
        ? ;
        object.set_named_property("permissionLevel",
        self.permission_level.to_owned()) ? ;
        object.set_named_property("serverChunkTickRange",
        self.server_chunk_tick_range.to_owned()) ? ;
        object.set_named_property("hasLockedBehaviourPacks",
        self.has_locked_behaviour_packs.to_owned()) ? ;
        object.set_named_property("hasLockedResourcePacks",
        self.has_locked_resource_packs.to_owned()) ? ;
        object.set_named_property("fromLockedWorldTemplate",
        self.from_locked_world_template.to_owned()) ? ;
        object.set_named_property("msaGamertagsOnly",
        self.msa_gamertags_only.to_owned()) ? ;
        object.set_named_property("fromWorldTemplate",
        self.from_world_template.to_owned()) ? ;
        object.set_named_property("worldTemplateOptionLocked",
        self.world_template_option_locked.to_owned()) ? ;
        object.set_named_property("onlySpawnV1Villagers",
        self.only_spawn_v1_villagers.to_owned()) ? ;
        object.set_named_property("personaDisabled",
        self.persona_disabled.to_owned()) ? ;
        object.set_named_property("customSkinsDisabled",
        self.custom_skins_disabled.to_owned()) ? ;
        object.set_named_property("emoteChatMuted",
        self.emote_chat_muted.to_owned()) ? ;
        object.set_named_property("gameVersion", self.game_version.to_owned())
        ? ;
        object.set_named_property("limitedWorldWidth",
        self.limited_world_width.to_owned()) ? ;
        object.set_named_property("limitedWorldLength",
        self.limited_world_length.to_owned()) ? ;
        object.set_named_property("newNether", self.new_nether.to_owned()) ? ;
        object.set_named_property("eduResourceUri",
        self.edu_resource_uri.to_object(env) ?) ? ;
        object.set_named_property("experimentalGameplayOverride",
        self.experimental_gameplay_override.to_owned()) ? ;
        object.set_named_property("chatRestrictionLevel",
        self.chat_restriction_level.to_owned()) ? ;
        object.set_named_property("disablePlayerInteractions",
        self.disable_player_interactions.to_owned()) ? ;
        object.set_named_property("levelId", self.level_id.to_owned()) ? ;
        object.set_named_property("worldName", self.world_name.to_owned()) ? ;
        object.set_named_property("premiumWorldTemplateId",
        self.premium_world_template_id.to_owned()) ? ;
        object.set_named_property("isTrial", self.is_trial.to_owned()) ? ;
        object.set_named_property("movementAuthority",
        self.movement_authority.to_owned()) ? ;
        object.set_named_property("rewindHistorySize",
        self.rewind_history_size.to_owned()) ? ;
        object.set_named_property("serverAuthoritiveBlockBreaking",
        self.server_authoritive_block_breaking.to_owned()) ? ;
        object.set_named_property("currentTick", self.current_tick.to_owned())
        ? ;
        object.set_named_property("enchantmentSeed",
        self.enchantment_seed.to_owned()) ? ; let mut block_properties =
        env.create_array_with_length(self.block_properties.len()) ? ;
        for(i, item) in self.block_properties.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            block_properties.set_element(i as u32, obj) ? ;
        } object.set_named_property("blockProperties", block_properties) ? ;
        let mut item_states =
        env.create_array_with_length(self.item_states.len()) ? ; for(i, item)
        in self.item_states.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            item_states.set_element(i as u32, obj) ? ;
        } object.set_named_property("itemStates", item_states) ? ;
        object.set_named_property("multiplayerCorrelationId",
        self.multiplayer_correlation_id.to_owned()) ? ;
        object.set_named_property("serverAuthoritativeInventory",
        self.server_authoritative_inventory.to_owned()) ? ;
        object.set_named_property("engine", self.engine.to_owned()) ? ;
        object.set_named_property("propertyData",
        self.property_data.to_owned()) ? ;
        object.set_named_property("blockPalletteChecksum",
        self.block_pallette_checksum.to_owned()) ? ;
        object.set_named_property("worldTemplateId",
        self.world_template_id.to_owned()) ? ;
        object.set_named_property("clientSideGeneration",
        self.client_side_generation.to_owned()) ? ;
        object.set_named_property("blockNetworkIdsAreHashes",
        self.block_network_ids_are_hashes.to_owned()) ? ;
        object.set_named_property("serverControlledSound",
        self.server_controlled_sound.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let entity_id : ZigZong = match data.get_named_property("entityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "entityId",
            stringify! (ZigZong), err),))
        } ; let runtime_entity_id : VarLong = match
        data.get_named_property("runtimeEntityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "runtimeEntityId", stringify! (VarLong), err),))
        } ; let player_gamemode : ZigZag = match
        data.get_named_property("playerGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerGamemode", stringify! (ZigZag), err),))
        } ; let player_position : Vec3f = match Vec3f ::
        from_object(data.get_named_property("playerPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerPosition", stringify! (Vec3f), err),))
        } ; let player_rotation : Vec2f = match Vec2f ::
        from_object(data.get_named_property("playerRotation") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerRotation", stringify! (Vec2f), err),))
        } ; let seed : LU64 = match data.get_named_property("seed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "seed",
            stringify! (LU64), err),))
        } ; let biome_type : LI16 = match data.get_named_property("biomeType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeType",
            stringify! (LI16), err),))
        } ; let biome_name : String = match
        data.get_named_property("biomeName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeName",
            stringify! (String), err),))
        } ; let dimension : ZigZag = match
        data.get_named_property("dimension")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "dimension",
            stringify! (ZigZag), err),))
        } ; let generator : ZigZag = match
        data.get_named_property("generator")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "generator",
            stringify! (ZigZag), err),))
        } ; let world_gamemode : ZigZag = match
        data.get_named_property("worldGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldGamemode",
            stringify! (ZigZag), err),))
        } ; let difficulty : ZigZag = match
        data.get_named_property("difficulty")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "difficulty",
            stringify! (ZigZag), err),))
        } ; let spawn_position : BlockCoordinates = match BlockCoordinates ::
        from_object(data.get_named_property("spawnPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "spawnPosition",
            stringify! (BlockCoordinates), err),))
        } ; let achievements_disabled : bool = match
        data.get_named_property("achievementsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "achievementsDisabled", stringify! (bool), err),))
        } ; let editor_world : bool = match
        data.get_named_property("editorWorld")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editorWorld",
            stringify! (bool), err),))
        } ; let created_in_editor : bool = match
        data.get_named_property("createdInEditor")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "createdInEditor", stringify! (bool), err),))
        } ; let day_cycle_stop_time : ZigZag = match
        data.get_named_property("dayCycleStopTime")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "dayCycleStopTime", stringify! (ZigZag), err),))
        } ; let edu_offer : ZigZag = match data.get_named_property("eduOffer")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduOffer",
            stringify! (ZigZag), err),))
        } ; let edu_features_enabled : bool = match
        data.get_named_property("eduFeaturesEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduFeaturesEnabled", stringify! (bool), err),))
        } ; let edu_product_id : String = match
        data.get_named_property("eduProductId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduProductId",
            stringify! (String), err),))
        } ; let rain_level : LF32 = match data.get_named_property("rainLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "rainLevel",
            stringify! (LF32), err),))
        } ; let lightning_level : LF32 = match
        data.get_named_property("lightningLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "lightningLevel", stringify! (LF32), err),))
        } ; let confirmed_platform_locked_content : bool = match
        data.get_named_property("confirmedPlatformLockedContent")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "confirmedPlatformLockedContent", stringify! (bool), err),))
        } ; let multiplayer_game : bool = match
        data.get_named_property("multiplayerGame")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerGame", stringify! (bool), err),))
        } ; let broadcast_to_lan : bool = match
        data.get_named_property("broadcastToLan")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "broadcastToLan", stringify! (bool), err),))
        } ; let xbl_broadcast_mode : VarInt = match
        data.get_named_property("xblBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "xblBroadcastMode", stringify! (VarInt), err),))
        } ; let platform_broadcast_mode : VarInt = match
        data.get_named_property("platformBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "platformBroadcastMode", stringify! (VarInt), err),))
        } ; let enable_commands : bool = match
        data.get_named_property("enableCommands")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enableCommands", stringify! (bool), err),))
        } ; let texture_packs_required : bool = match
        data.get_named_property("texturePacksRequired")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "texturePacksRequired", stringify! (bool), err),))
        } ; let gamerules_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("gamerules")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gamerules",
            stringify! (Vec < GameRule >), err),))
        } ; let mut gamerules : Vec < GameRule > = Vec :: new() ; for item in
        gamerules_napi { gamerules.push(GameRule :: from_object(item) ?) ; }
        let experiments_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("experiments")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "experiments",
            stringify! (Vec < Experiment >), err),))
        } ; let mut experiments : Vec < Experiment > = Vec :: new() ; for item
        in experiments_napi
        { experiments.push(Experiment :: from_object(item) ?) ; } let
        experiments_previously_used : bool = match
        data.get_named_property("experimentsPreviouslyUsed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentsPreviouslyUsed", stringify! (bool), err),))
        } ; let bonus_chest : bool = match
        data.get_named_property("bonusChest")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "bonusChest",
            stringify! (bool), err),))
        } ; let map_enabled : bool = match
        data.get_named_property("mapEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "mapEnabled",
            stringify! (bool), err),))
        } ; let permission_level : u8 = match
        data.get_named_property("permissionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "permissionLevel", stringify! (u8), err),))
        } ; let server_chunk_tick_range : LI32 = match
        data.get_named_property("serverChunkTickRange")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverChunkTickRange", stringify! (LI32), err),))
        } ; let has_locked_behaviour_packs : bool = match
        data.get_named_property("hasLockedBehaviourPacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedBehaviourPacks", stringify! (bool), err),))
        } ; let has_locked_resource_packs : bool = match
        data.get_named_property("hasLockedResourcePacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedResourcePacks", stringify! (bool), err),))
        } ; let from_locked_world_template : bool = match
        data.get_named_property("fromLockedWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromLockedWorldTemplate", stringify! (bool), err),))
        } ; let msa_gamertags_only : bool = match
        data.get_named_property("msaGamertagsOnly")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "msaGamertagsOnly", stringify! (bool), err),))
        } ; let from_world_template : bool = match
        data.get_named_property("fromWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromWorldTemplate", stringify! (bool), err),))
        } ; let world_template_option_locked : bool = match
        data.get_named_property("worldTemplateOptionLocked")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateOptionLocked", stringify! (bool), err),))
        } ; let only_spawn_v1_villagers : bool = match
        data.get_named_property("onlySpawnV1Villagers")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "onlySpawnV1Villagers", stringify! (bool), err),))
        } ; let persona_disabled : bool = match
        data.get_named_property("personaDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "personaDisabled", stringify! (bool), err),))
        } ; let custom_skins_disabled : bool = match
        data.get_named_property("customSkinsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "customSkinsDisabled", stringify! (bool), err),))
        } ; let emote_chat_muted : bool = match
        data.get_named_property("emoteChatMuted")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "emoteChatMuted", stringify! (bool), err),))
        } ; let game_version : String = match
        data.get_named_property("gameVersion")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gameVersion",
            stringify! (String), err),))
        } ; let limited_world_width : LI32 = match
        data.get_named_property("limitedWorldWidth")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldWidth", stringify! (LI32), err),))
        } ; let limited_world_length : LI32 = match
        data.get_named_property("limitedWorldLength")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldLength", stringify! (LI32), err),))
        } ; let new_nether : bool = match data.get_named_property("newNether")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "newNether",
            stringify! (bool), err),))
        } ; let edu_resource_uri : EducationSharedResource = match
        EducationSharedResource ::
        from_object(data.get_named_property("eduResourceUri") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduResourceUri", stringify! (EducationSharedResource), err),))
        } ; let experimental_gameplay_override : bool = match
        data.get_named_property("experimentalGameplayOverride")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentalGameplayOverride", stringify! (bool), err),))
        } ; let chat_restriction_level : u8 = match
        data.get_named_property("chatRestrictionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "chatRestrictionLevel", stringify! (u8), err),))
        } ; let disable_player_interactions : bool = match
        data.get_named_property("disablePlayerInteractions")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "disablePlayerInteractions", stringify! (bool), err),))
        } ; let level_id : String = match data.get_named_property("levelId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "levelId",
            stringify! (String), err),))
        } ; let world_name : String = match
        data.get_named_property("worldName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldName",
            stringify! (String), err),))
        } ; let premium_world_template_id : String = match
        data.get_named_property("premiumWorldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "premiumWorldTemplateId", stringify! (String), err),))
        } ; let is_trial : bool = match data.get_named_property("isTrial")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "isTrial",
            stringify! (bool), err),))
        } ; let movement_authority : ZigZag = match
        data.get_named_property("movementAuthority")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "movementAuthority", stringify! (ZigZag), err),))
        } ; let rewind_history_size : ZigZag = match
        data.get_named_property("rewindHistorySize")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "rewindHistorySize", stringify! (ZigZag), err),))
        } ; let server_authoritive_block_breaking : bool = match
        data.get_named_property("serverAuthoritiveBlockBreaking")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritiveBlockBreaking", stringify! (bool), err),))
        } ; let current_tick : LI64 = match
        data.get_named_property("currentTick")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "currentTick",
            stringify! (LI64), err),))
        } ; let enchantment_seed : ZigZag = match
        data.get_named_property("enchantmentSeed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enchantmentSeed", stringify! (ZigZag), err),))
        } ; let block_properties_napi : Vec < napi :: bindgen_prelude ::
        Object > = match data.get_named_property("blockProperties")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockProperties", stringify! (Vec < BlockProperty >), err),))
        } ; let mut block_properties : Vec < BlockProperty > = Vec :: new() ;
        for item in block_properties_napi
        { block_properties.push(BlockProperty :: from_object(item) ?) ; } let
        item_states_napi : Vec < napi :: bindgen_prelude :: Object > = match
        data.get_named_property("itemStates")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "itemStates",
            stringify! (Vec < ItemState >), err),))
        } ; let mut item_states : Vec < ItemState > = Vec :: new() ; for item
        in item_states_napi
        { item_states.push(ItemState :: from_object(item) ?) ; } let
        multiplayer_correlation_id : String = match
        data.get_named_property("multiplayerCorrelationId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerCorrelationId", stringify! (String), err),))
        } ; let server_authoritative_inventory : bool = match
        data.get_named_property("serverAuthoritativeInventory")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritativeInventory", stringify! (bool), err),))
        } ; let engine : String = match data.get_named_property("engine")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "engine",
            stringify! (String), err),))
        } ; let property_data : NBT = match
        data.get_named_property("propertyData")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "propertyData",
            stringify! (NBT), err),))
        } ; let block_pallette_checksum : LU64 = match
        data.get_named_property("blockPalletteChecksum")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockPalletteChecksum", stringify! (LU64), err),))
        } ; let world_template_id : UUID = match
        data.get_named_property("worldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateId", stringify! (UUID), err),))
        } ; let client_side_generation : bool = match
        data.get_named_property("clientSideGeneration")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "clientSideGeneration", stringify! (bool), err),))
        } ; let block_network_ids_are_hashes : bool = match
        data.get_named_property("blockNetworkIdsAreHashes")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockNetworkIdsAreHashes", stringify! (bool), err),))
        } ; let server_controlled_sound : bool = match
        data.get_named_property("serverControlledSound")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverControlledSound", stringify! (bool), err),))
        } ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockCoordinates { pub x : ZigZag, pub y : VarInt, pub z : ZigZag, } impl
crate :: packets :: prelude :: PacketChildSerialization for BlockCoordinates
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_zigzag(self.x.to_owned()) ? ;
        stream.write_varint(self.y.to_owned()) ? ;
        stream.write_zigzag(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : ZigZag = stream.read_zigzag() ? ;
        let y : VarInt = stream.read_varint() ? ; let z : ZigZag =
        stream.read_zigzag() ? ; Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockCoordinates
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : ZigZag = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (ZigZag), err),))
        } ; let y : VarInt = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (VarInt), err),))
        } ; let z : ZigZag = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (ZigZag), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockCoordinates { pub x : ZigZag, pub y : VarInt, pub z : ZigZag, } impl
crate :: packets :: prelude :: PacketChildSerialization for BlockCoordinates
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_zigzag(self.x.to_owned()) ? ;
        stream.write_varint(self.y.to_owned()) ? ;
        stream.write_zigzag(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : ZigZag = stream.read_zigzag() ? ;
        let y : VarInt = stream.read_varint() ? ; let z : ZigZag =
        stream.read_zigzag() ? ; Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockCoordinates
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : ZigZag = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (ZigZag), err),))
        } ; let y : VarInt = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (VarInt), err),))
        } ; let z : ZigZag = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (ZigZag), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
GameRule
{
    pub name : String, pub editable : bool, #[napi(ts_type = "GameRuleType")]
    pub field_type : VarInt, #[napi(ts_type = "boolean | number")] pub value :
    Value,
} impl crate :: packets :: prelude :: PacketConversion for GameRule
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("editable", self.editable.to_owned()) ? ;
        object.set_named_property("fieldType", self.field_type.to_owned()) ? ;
        object.set_named_property("value", self.value.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let editable : bool = match data.get_named_property("editable")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editable",
            stringify! (bool), err),))
        } ; let field_type : VarInt = match
        data.get_named_property("fieldType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "fieldType",
            stringify! (VarInt), err),))
        } ; let value : Value = match data.get_named_property("value")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "value",
            stringify! (Value), err),))
        } ; Ok(Self { name, editable, field_type, value })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
GameRule
{
    pub name : String, pub editable : bool, #[napi(ts_type = "GameRuleType")]
    pub field_type : VarInt, #[napi(ts_type = "boolean | number")] pub value :
    Value,
} impl crate :: packets :: prelude :: PacketConversion for GameRule
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("editable", self.editable.to_owned()) ? ;
        object.set_named_property("fieldType", self.field_type.to_owned()) ? ;
        object.set_named_property("value", self.value.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let editable : bool = match data.get_named_property("editable")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editable",
            stringify! (bool), err),))
        } ; let field_type : VarInt = match
        data.get_named_property("fieldType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "fieldType",
            stringify! (VarInt), err),))
        } ; let value : Value = match data.get_named_property("value")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "value",
            stringify! (Value), err),))
        } ; Ok(Self { name, editable, field_type, value })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
EducationSharedResource { pub button_name : String, pub link_uri : String, }
impl crate :: packets :: prelude :: PacketChildSerialization for
EducationSharedResource
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.button_name.to_owned()) ? ;
        stream.write_string(self.link_uri.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let button_name : String =
        stream.read_string() ? ; let link_uri : String = stream.read_string()
        ? ; Ok(Self { button_name, link_uri })
    }
} impl crate :: packets :: prelude :: PacketConversion for
EducationSharedResource
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("buttonName", self.button_name.to_owned()) ?
        ; object.set_named_property("linkUri", self.link_uri.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let button_name : String = match data.get_named_property("buttonName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "buttonName",
            stringify! (String), err),))
        } ; let link_uri : String = match data.get_named_property("linkUri")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "linkUri",
            stringify! (String), err),))
        } ; Ok(Self { button_name, link_uri })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
EducationSharedResource { pub button_name : String, pub link_uri : String, }
impl crate :: packets :: prelude :: PacketChildSerialization for
EducationSharedResource
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.button_name.to_owned()) ? ;
        stream.write_string(self.link_uri.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let button_name : String =
        stream.read_string() ? ; let link_uri : String = stream.read_string()
        ? ; Ok(Self { button_name, link_uri })
    }
} impl crate :: packets :: prelude :: PacketConversion for
EducationSharedResource
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("buttonName", self.button_name.to_owned()) ?
        ; object.set_named_property("linkUri", self.link_uri.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let button_name : String = match data.get_named_property("buttonName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "buttonName",
            stringify! (String), err),))
        } ; let link_uri : String = match data.get_named_property("linkUri")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "linkUri",
            stringify! (String), err),))
        } ; Ok(Self { button_name, link_uri })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockProperty { pub name : String, pub state : NBT, } impl crate :: packets ::
prelude :: PacketChildSerialization for BlockProperty
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.append(& mut self.state.serialize() ?) ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let state : NBT = NBT :: deserialize(& mut stream) ? ;
        Ok(Self { name, state })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockProperty
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("state", self.state.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let state : NBT = match data.get_named_property("state")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "state",
            stringify! (NBT), err),))
        } ; Ok(Self { name, state })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
ItemState
{ pub name : String, pub runtime_id : LI16, pub component_based : bool, } impl
crate :: packets :: prelude :: PacketChildSerialization for ItemState
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_li16(self.runtime_id.to_owned()) ? ;
        stream.write_bool(self.component_based.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let runtime_id : LI16 = stream.read_li16() ? ; let component_based :
        bool = stream.read_bool() ? ;
        Ok(Self { name, runtime_id, component_based })
    }
} impl crate :: packets :: prelude :: PacketConversion for ItemState
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("runtimeId", self.runtime_id.to_owned()) ? ;
        object.set_named_property("componentBased",
        self.component_based.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let runtime_id : LI16 = match data.get_named_property("runtimeId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "runtimeId",
            stringify! (LI16), err),))
        } ; let component_based : bool = match
        data.get_named_property("componentBased")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "componentBased", stringify! (bool), err),))
        } ; Ok(Self { name, runtime_id, component_based })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockProperty { pub name : String, pub state : NBT, } impl crate :: packets ::
prelude :: PacketChildSerialization for BlockProperty
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.append(& mut self.state.serialize() ?) ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let state : NBT = NBT :: deserialize(& mut stream) ? ;
        Ok(Self { name, state })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockProperty
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("state", self.state.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let state : NBT = match data.get_named_property("state")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "state",
            stringify! (NBT), err),))
        } ; Ok(Self { name, state })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
ItemState
{ pub name : String, pub runtime_id : LI16, pub component_based : bool, } impl
crate :: packets :: prelude :: PacketChildSerialization for ItemState
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_li16(self.runtime_id.to_owned()) ? ;
        stream.write_bool(self.component_based.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let runtime_id : LI16 = stream.read_li16() ? ; let component_based :
        bool = stream.read_bool() ? ;
        Ok(Self { name, runtime_id, component_based })
    }
} impl crate :: packets :: prelude :: PacketConversion for ItemState
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("runtimeId", self.runtime_id.to_owned()) ? ;
        object.set_named_property("componentBased",
        self.component_based.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let runtime_id : LI16 = match data.get_named_property("runtimeId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "runtimeId",
            stringify! (LI16), err),))
        } ; let component_based : bool = match
        data.get_named_property("componentBased")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "componentBased", stringify! (bool), err),))
        } ; Ok(Self { name, runtime_id, component_based })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec3f { pub x : LF32, pub y : LF32, pub z : LF32, } impl crate :: packets ::
prelude :: PacketChildSerialization for Vec3f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.y.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let y
        : LF32 = stream.read_lf32() ? ; let z : LF32 = stream.read_lf32() ? ;
        Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec3f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let y : LF32 = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Vec2f { pub x : LF32, pub z : LF32, } impl crate :: packets :: prelude ::
PacketChildSerialization for Vec2f
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_lf32(self.x.to_owned()) ? ;
        stream.write_lf32(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : LF32 = stream.read_lf32() ? ; let z
        : LF32 = stream.read_lf32() ? ; Ok(Self { x, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for Vec2f
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : LF32 = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (LF32), err),))
        } ; let z : LF32 = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (LF32), err),))
        } ; Ok(Self { x, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
Experiment { pub name : String, pub enabled : bool, } impl crate :: packets ::
prelude :: PacketChildSerialization for Experiment
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_bool(self.enabled.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let enabled : bool = stream.read_bool() ? ; Ok(Self { name, enabled })
    }
} impl crate :: packets :: prelude :: PacketConversion for Experiment
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("enabled", self.enabled.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let enabled : bool = match data.get_named_property("enabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "enabled",
            stringify! (bool), err),))
        } ; Ok(Self { name, enabled })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
StartGamePacket
{
    pub entity_id : ZigZong, pub runtime_entity_id : VarLong,
    #[napi(ts_type = "GameMode")] pub player_gamemode : ZigZag, pub
    player_position : Vec3f, pub player_rotation : Vec2f, pub seed : LU64, pub
    biome_type : LI16, pub biome_name : String, #[napi(ts_type = "Dimension")]
    pub dimension : ZigZag, pub generator : ZigZag,
    #[napi(ts_type = "GameMode")] pub world_gamemode : ZigZag, pub difficulty
    : ZigZag, pub spawn_position : BlockCoordinates, pub achievements_disabled
    : bool, pub editor_world : bool, pub created_in_editor : bool, pub
    day_cycle_stop_time : ZigZag, pub edu_offer : ZigZag, pub
    edu_features_enabled : bool, pub edu_product_id : String, pub rain_level :
    LF32, pub lightning_level : LF32, pub confirmed_platform_locked_content :
    bool, pub multiplayer_game : bool, pub broadcast_to_lan : bool, pub
    xbl_broadcast_mode : VarInt, pub platform_broadcast_mode : VarInt, pub
    enable_commands : bool, pub texture_packs_required : bool, #[VarInt] pub
    gamerules : Vec < GameRule >, #[LI32] pub experiments : Vec < Experiment
    >, pub experiments_previously_used : bool, pub bonus_chest : bool, pub
    map_enabled : bool, #[napi(ts_type = "PermissionLevel")] pub
    permission_level : u8, pub server_chunk_tick_range : LI32, pub
    has_locked_behaviour_packs : bool, pub has_locked_resource_packs : bool,
    pub from_locked_world_template : bool, pub msa_gamertags_only : bool, pub
    from_world_template : bool, pub world_template_option_locked : bool, pub
    only_spawn_v1_villagers : bool, pub persona_disabled : bool, pub
    custom_skins_disabled : bool, pub emote_chat_muted : bool, pub
    game_version : String, pub limited_world_width : LI32, pub
    limited_world_length : LI32, pub new_nether : bool, pub edu_resource_uri :
    EducationSharedResource, pub experimental_gameplay_override : bool,
    #[napi(ts_type = "ChatRestrictionLevel")] pub chat_restriction_level : u8,
    pub disable_player_interactions : bool, pub level_id : String, pub
    world_name : String, pub premium_world_template_id : String, pub is_trial
    : bool, #[napi(ts_type = "MovementAuthority")] pub movement_authority :
    ZigZag, pub rewind_history_size : ZigZag, pub
    server_authoritive_block_breaking : bool, pub current_tick : LI64, pub
    enchantment_seed : ZigZag, #[VarInt] pub block_properties : Vec <
    BlockProperty >, #[VarInt] pub item_states : Vec < ItemState >, pub
    multiplayer_correlation_id : String, pub server_authoritative_inventory :
    bool, pub engine : String, pub property_data : NBT, pub
    block_pallette_checksum : LU64, pub world_template_id : UUID, pub
    client_side_generation : bool, pub block_network_ids_are_hashes : bool,
    pub server_controlled_sound : bool
} impl StartGamePacket
{ pub const ID : crate :: binary :: prelude :: VarInt = 11u32 ; } impl crate
:: packets :: prelude :: PacketSerialization for StartGamePacket
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < napi ::
    bindgen_prelude :: Buffer >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_varint(StartGamePacket :: ID) ? ;
        stream.write_zigzong(self.entity_id.to_owned()) ? ;
        stream.write_varlong(self.runtime_entity_id.to_owned()) ? ;
        stream.write_zigzag(self.player_gamemode.to_owned()) ? ;
        stream.append(& mut self.player_position.serialize() ?) ;
        stream.append(& mut self.player_rotation.serialize() ?) ;
        stream.write_lu64(self.seed.to_owned()) ? ;
        stream.write_li16(self.biome_type.to_owned()) ? ;
        stream.write_string(self.biome_name.to_owned()) ? ;
        stream.write_zigzag(self.dimension.to_owned()) ? ;
        stream.write_zigzag(self.generator.to_owned()) ? ;
        stream.write_zigzag(self.world_gamemode.to_owned()) ? ;
        stream.write_zigzag(self.difficulty.to_owned()) ? ;
        stream.append(& mut self.spawn_position.serialize() ?) ;
        stream.write_bool(self.achievements_disabled.to_owned()) ? ;
        stream.write_bool(self.editor_world.to_owned()) ? ;
        stream.write_bool(self.created_in_editor.to_owned()) ? ;
        stream.write_zigzag(self.day_cycle_stop_time.to_owned()) ? ;
        stream.write_zigzag(self.edu_offer.to_owned()) ? ;
        stream.write_bool(self.edu_features_enabled.to_owned()) ? ;
        stream.write_string(self.edu_product_id.to_owned()) ? ;
        stream.write_lf32(self.rain_level.to_owned()) ? ;
        stream.write_lf32(self.lightning_level.to_owned()) ? ;
        stream.write_bool(self.confirmed_platform_locked_content.to_owned()) ?
        ; stream.write_bool(self.multiplayer_game.to_owned()) ? ;
        stream.write_bool(self.broadcast_to_lan.to_owned()) ? ;
        stream.write_varint(self.xbl_broadcast_mode.to_owned()) ? ;
        stream.write_varint(self.platform_broadcast_mode.to_owned()) ? ;
        stream.write_bool(self.enable_commands.to_owned()) ? ;
        stream.write_bool(self.texture_packs_required.to_owned()) ? ; let
        gamerules_len = self.gamerules.len() ;
        stream.write_varint(gamerules_len as VarInt) ? ; for i in 0 ..
        gamerules_len
        {
            let mut bin = self.gamerules [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let experiments_len = self.experiments.len() ;
        stream.write_li32(experiments_len as LI32) ? ; for i in 0 ..
        experiments_len
        {
            let mut bin = self.experiments [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_bool(self.experiments_previously_used.to_owned()) ? ;
        stream.write_bool(self.bonus_chest.to_owned()) ? ;
        stream.write_bool(self.map_enabled.to_owned()) ? ;
        stream.write_u8(self.permission_level.to_owned()) ? ;
        stream.write_li32(self.server_chunk_tick_range.to_owned()) ? ;
        stream.write_bool(self.has_locked_behaviour_packs.to_owned()) ? ;
        stream.write_bool(self.has_locked_resource_packs.to_owned()) ? ;
        stream.write_bool(self.from_locked_world_template.to_owned()) ? ;
        stream.write_bool(self.msa_gamertags_only.to_owned()) ? ;
        stream.write_bool(self.from_world_template.to_owned()) ? ;
        stream.write_bool(self.world_template_option_locked.to_owned()) ? ;
        stream.write_bool(self.only_spawn_v1_villagers.to_owned()) ? ;
        stream.write_bool(self.persona_disabled.to_owned()) ? ;
        stream.write_bool(self.custom_skins_disabled.to_owned()) ? ;
        stream.write_bool(self.emote_chat_muted.to_owned()) ? ;
        stream.write_string(self.game_version.to_owned()) ? ;
        stream.write_li32(self.limited_world_width.to_owned()) ? ;
        stream.write_li32(self.limited_world_length.to_owned()) ? ;
        stream.write_bool(self.new_nether.to_owned()) ? ;
        stream.append(& mut self.edu_resource_uri.serialize() ?) ;
        stream.write_bool(self.experimental_gameplay_override.to_owned()) ? ;
        stream.write_u8(self.chat_restriction_level.to_owned()) ? ;
        stream.write_bool(self.disable_player_interactions.to_owned()) ? ;
        stream.write_string(self.level_id.to_owned()) ? ;
        stream.write_string(self.world_name.to_owned()) ? ;
        stream.write_string(self.premium_world_template_id.to_owned()) ? ;
        stream.write_bool(self.is_trial.to_owned()) ? ;
        stream.write_zigzag(self.movement_authority.to_owned()) ? ;
        stream.write_zigzag(self.rewind_history_size.to_owned()) ? ;
        stream.write_bool(self.server_authoritive_block_breaking.to_owned()) ?
        ; stream.write_li64(self.current_tick.to_owned()) ? ;
        stream.write_zigzag(self.enchantment_seed.to_owned()) ? ; let
        block_properties_len = self.block_properties.len() ;
        stream.write_varint(block_properties_len as VarInt) ? ; for i in 0 ..
        block_properties_len
        {
            let mut bin = self.block_properties [i].serialize() ? ;
            stream.append(& mut bin) ;
        } let item_states_len = self.item_states.len() ;
        stream.write_varint(item_states_len as VarInt) ? ; for i in 0 ..
        item_states_len
        {
            let mut bin = self.item_states [i].serialize() ? ;
            stream.append(& mut bin) ;
        } stream.write_string(self.multiplayer_correlation_id.to_owned()) ? ;
        stream.write_bool(self.server_authoritative_inventory.to_owned()) ? ;
        stream.write_string(self.engine.to_owned()) ? ;
        stream.append(& mut self.property_data.serialize() ?) ;
        stream.write_lu64(self.block_pallette_checksum.to_owned()) ? ;
        stream.write_uuid(self.world_template_id.to_owned()) ? ;
        stream.write_bool(self.client_side_generation.to_owned()) ? ;
        stream.write_bool(self.block_network_ids_are_hashes.to_owned()) ? ;
        stream.write_bool(self.server_controlled_sound.to_owned()) ? ;
        Ok(stream.data.into())
    } fn deserialize(data : napi :: bindgen_prelude :: Buffer) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let mut stream = crate :: binary :: BinaryStream :: from(data.into())
        ; let _id = stream.read_varint() ? ; let entity_id : ZigZong =
        stream.read_zigzong() ? ; let runtime_entity_id : VarLong =
        stream.read_varlong() ? ; let player_gamemode : ZigZag =
        stream.read_zigzag() ? ; let player_position : Vec3f = Vec3f ::
        deserialize(& mut stream) ? ; let player_rotation : Vec2f = Vec2f ::
        deserialize(& mut stream) ? ; let seed : LU64 = stream.read_lu64() ? ;
        let biome_type : LI16 = stream.read_li16() ? ; let biome_name : String
        = stream.read_string() ? ; let dimension : ZigZag =
        stream.read_zigzag() ? ; let generator : ZigZag = stream.read_zigzag()
        ? ; let world_gamemode : ZigZag = stream.read_zigzag() ? ; let
        difficulty : ZigZag = stream.read_zigzag() ? ; let spawn_position :
        BlockCoordinates = BlockCoordinates :: deserialize(& mut stream) ? ;
        let achievements_disabled : bool = stream.read_bool() ? ; let
        editor_world : bool = stream.read_bool() ? ; let created_in_editor :
        bool = stream.read_bool() ? ; let day_cycle_stop_time : ZigZag =
        stream.read_zigzag() ? ; let edu_offer : ZigZag = stream.read_zigzag()
        ? ; let edu_features_enabled : bool = stream.read_bool() ? ; let
        edu_product_id : String = stream.read_string() ? ; let rain_level :
        LF32 = stream.read_lf32() ? ; let lightning_level : LF32 =
        stream.read_lf32() ? ; let confirmed_platform_locked_content : bool =
        stream.read_bool() ? ; let multiplayer_game : bool =
        stream.read_bool() ? ; let broadcast_to_lan : bool =
        stream.read_bool() ? ; let xbl_broadcast_mode : VarInt =
        stream.read_varint() ? ; let platform_broadcast_mode : VarInt =
        stream.read_varint() ? ; let enable_commands : bool =
        stream.read_bool() ? ; let texture_packs_required : bool =
        stream.read_bool() ? ; let gamerules_len = stream.read_varint() ? ;
        let mut gamerules : Vec < GameRule > = Vec :: new() ; for _ in 0 ..
        gamerules_len
        { gamerules.push(GameRule :: deserialize(& mut stream) ?) ; } let
        experiments_len = stream.read_li32() ? ; let mut experiments : Vec <
        Experiment > = Vec :: new() ; for _ in 0 .. experiments_len
        { experiments.push(Experiment :: deserialize(& mut stream) ?) ; } let
        experiments_previously_used : bool = stream.read_bool() ? ; let
        bonus_chest : bool = stream.read_bool() ? ; let map_enabled : bool =
        stream.read_bool() ? ; let permission_level : u8 = stream.read_u8() ?
        ; let server_chunk_tick_range : LI32 = stream.read_li32() ? ; let
        has_locked_behaviour_packs : bool = stream.read_bool() ? ; let
        has_locked_resource_packs : bool = stream.read_bool() ? ; let
        from_locked_world_template : bool = stream.read_bool() ? ; let
        msa_gamertags_only : bool = stream.read_bool() ? ; let
        from_world_template : bool = stream.read_bool() ? ; let
        world_template_option_locked : bool = stream.read_bool() ? ; let
        only_spawn_v1_villagers : bool = stream.read_bool() ? ; let
        persona_disabled : bool = stream.read_bool() ? ; let
        custom_skins_disabled : bool = stream.read_bool() ? ; let
        emote_chat_muted : bool = stream.read_bool() ? ; let game_version :
        String = stream.read_string() ? ; let limited_world_width : LI32 =
        stream.read_li32() ? ; let limited_world_length : LI32 =
        stream.read_li32() ? ; let new_nether : bool = stream.read_bool() ? ;
        let edu_resource_uri : EducationSharedResource =
        EducationSharedResource :: deserialize(& mut stream) ? ; let
        experimental_gameplay_override : bool = stream.read_bool() ? ; let
        chat_restriction_level : u8 = stream.read_u8() ? ; let
        disable_player_interactions : bool = stream.read_bool() ? ; let
        level_id : String = stream.read_string() ? ; let world_name : String =
        stream.read_string() ? ; let premium_world_template_id : String =
        stream.read_string() ? ; let is_trial : bool = stream.read_bool() ? ;
        let movement_authority : ZigZag = stream.read_zigzag() ? ; let
        rewind_history_size : ZigZag = stream.read_zigzag() ? ; let
        server_authoritive_block_breaking : bool = stream.read_bool() ? ; let
        current_tick : LI64 = stream.read_li64() ? ; let enchantment_seed :
        ZigZag = stream.read_zigzag() ? ; let block_properties_len =
        stream.read_varint() ? ; let mut block_properties : Vec <
        BlockProperty > = Vec :: new() ; for _ in 0 .. block_properties_len
        {
            block_properties.push(BlockProperty :: deserialize(& mut stream)
            ?) ;
        } let item_states_len = stream.read_varint() ? ; let mut item_states :
        Vec < ItemState > = Vec :: new() ; for _ in 0 .. item_states_len
        { item_states.push(ItemState :: deserialize(& mut stream) ?) ; } let
        multiplayer_correlation_id : String = stream.read_string() ? ; let
        server_authoritative_inventory : bool = stream.read_bool() ? ; let
        engine : String = stream.read_string() ? ; let property_data : NBT =
        NBT :: deserialize(& mut stream) ? ; let block_pallette_checksum :
        LU64 = stream.read_lu64() ? ; let world_template_id : UUID =
        stream.read_uuid() ? ; let client_side_generation : bool =
        stream.read_bool() ? ; let block_network_ids_are_hashes : bool =
        stream.read_bool() ? ; let server_controlled_sound : bool =
        stream.read_bool() ? ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
} impl crate :: packets :: prelude :: PacketConversion for StartGamePacket
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("entityId", self.entity_id.to_owned()) ? ;
        object.set_named_property("runtimeEntityId",
        self.runtime_entity_id.to_owned()) ? ;
        object.set_named_property("playerGamemode",
        self.player_gamemode.to_owned()) ? ;
        object.set_named_property("playerPosition",
        self.player_position.to_object(env) ?) ? ;
        object.set_named_property("playerRotation",
        self.player_rotation.to_object(env) ?) ? ;
        object.set_named_property("seed", self.seed.to_owned()) ? ;
        object.set_named_property("biomeType", self.biome_type.to_owned()) ? ;
        object.set_named_property("biomeName", self.biome_name.to_owned()) ? ;
        object.set_named_property("dimension", self.dimension.to_owned()) ? ;
        object.set_named_property("generator", self.generator.to_owned()) ? ;
        object.set_named_property("worldGamemode",
        self.world_gamemode.to_owned()) ? ;
        object.set_named_property("difficulty", self.difficulty.to_owned()) ?
        ;
        object.set_named_property("spawnPosition",
        self.spawn_position.to_object(env) ?) ? ;
        object.set_named_property("achievementsDisabled",
        self.achievements_disabled.to_owned()) ? ;
        object.set_named_property("editorWorld", self.editor_world.to_owned())
        ? ;
        object.set_named_property("createdInEditor",
        self.created_in_editor.to_owned()) ? ;
        object.set_named_property("dayCycleStopTime",
        self.day_cycle_stop_time.to_owned()) ? ;
        object.set_named_property("eduOffer", self.edu_offer.to_owned()) ? ;
        object.set_named_property("eduFeaturesEnabled",
        self.edu_features_enabled.to_owned()) ? ;
        object.set_named_property("eduProductId",
        self.edu_product_id.to_owned()) ? ;
        object.set_named_property("rainLevel", self.rain_level.to_owned()) ? ;
        object.set_named_property("lightningLevel",
        self.lightning_level.to_owned()) ? ;
        object.set_named_property("confirmedPlatformLockedContent",
        self.confirmed_platform_locked_content.to_owned()) ? ;
        object.set_named_property("multiplayerGame",
        self.multiplayer_game.to_owned()) ? ;
        object.set_named_property("broadcastToLan",
        self.broadcast_to_lan.to_owned()) ? ;
        object.set_named_property("xblBroadcastMode",
        self.xbl_broadcast_mode.to_owned()) ? ;
        object.set_named_property("platformBroadcastMode",
        self.platform_broadcast_mode.to_owned()) ? ;
        object.set_named_property("enableCommands",
        self.enable_commands.to_owned()) ? ;
        object.set_named_property("texturePacksRequired",
        self.texture_packs_required.to_owned()) ? ; let mut gamerules =
        env.create_array_with_length(self.gamerules.len()) ? ; for(i, item) in
        self.gamerules.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            gamerules.set_element(i as u32, obj) ? ;
        } object.set_named_property("gamerules", gamerules) ? ; let mut
        experiments = env.create_array_with_length(self.experiments.len()) ? ;
        for(i, item) in self.experiments.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            experiments.set_element(i as u32, obj) ? ;
        } object.set_named_property("experiments", experiments) ? ;
        object.set_named_property("experimentsPreviouslyUsed",
        self.experiments_previously_used.to_owned()) ? ;
        object.set_named_property("bonusChest", self.bonus_chest.to_owned()) ?
        ; object.set_named_property("mapEnabled", self.map_enabled.to_owned())
        ? ;
        object.set_named_property("permissionLevel",
        self.permission_level.to_owned()) ? ;
        object.set_named_property("serverChunkTickRange",
        self.server_chunk_tick_range.to_owned()) ? ;
        object.set_named_property("hasLockedBehaviourPacks",
        self.has_locked_behaviour_packs.to_owned()) ? ;
        object.set_named_property("hasLockedResourcePacks",
        self.has_locked_resource_packs.to_owned()) ? ;
        object.set_named_property("fromLockedWorldTemplate",
        self.from_locked_world_template.to_owned()) ? ;
        object.set_named_property("msaGamertagsOnly",
        self.msa_gamertags_only.to_owned()) ? ;
        object.set_named_property("fromWorldTemplate",
        self.from_world_template.to_owned()) ? ;
        object.set_named_property("worldTemplateOptionLocked",
        self.world_template_option_locked.to_owned()) ? ;
        object.set_named_property("onlySpawnV1Villagers",
        self.only_spawn_v1_villagers.to_owned()) ? ;
        object.set_named_property("personaDisabled",
        self.persona_disabled.to_owned()) ? ;
        object.set_named_property("customSkinsDisabled",
        self.custom_skins_disabled.to_owned()) ? ;
        object.set_named_property("emoteChatMuted",
        self.emote_chat_muted.to_owned()) ? ;
        object.set_named_property("gameVersion", self.game_version.to_owned())
        ? ;
        object.set_named_property("limitedWorldWidth",
        self.limited_world_width.to_owned()) ? ;
        object.set_named_property("limitedWorldLength",
        self.limited_world_length.to_owned()) ? ;
        object.set_named_property("newNether", self.new_nether.to_owned()) ? ;
        object.set_named_property("eduResourceUri",
        self.edu_resource_uri.to_object(env) ?) ? ;
        object.set_named_property("experimentalGameplayOverride",
        self.experimental_gameplay_override.to_owned()) ? ;
        object.set_named_property("chatRestrictionLevel",
        self.chat_restriction_level.to_owned()) ? ;
        object.set_named_property("disablePlayerInteractions",
        self.disable_player_interactions.to_owned()) ? ;
        object.set_named_property("levelId", self.level_id.to_owned()) ? ;
        object.set_named_property("worldName", self.world_name.to_owned()) ? ;
        object.set_named_property("premiumWorldTemplateId",
        self.premium_world_template_id.to_owned()) ? ;
        object.set_named_property("isTrial", self.is_trial.to_owned()) ? ;
        object.set_named_property("movementAuthority",
        self.movement_authority.to_owned()) ? ;
        object.set_named_property("rewindHistorySize",
        self.rewind_history_size.to_owned()) ? ;
        object.set_named_property("serverAuthoritiveBlockBreaking",
        self.server_authoritive_block_breaking.to_owned()) ? ;
        object.set_named_property("currentTick", self.current_tick.to_owned())
        ? ;
        object.set_named_property("enchantmentSeed",
        self.enchantment_seed.to_owned()) ? ; let mut block_properties =
        env.create_array_with_length(self.block_properties.len()) ? ;
        for(i, item) in self.block_properties.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            block_properties.set_element(i as u32, obj) ? ;
        } object.set_named_property("blockProperties", block_properties) ? ;
        let mut item_states =
        env.create_array_with_length(self.item_states.len()) ? ; for(i, item)
        in self.item_states.iter().enumerate()
        {
            let obj = item.to_object(env) ? ;
            item_states.set_element(i as u32, obj) ? ;
        } object.set_named_property("itemStates", item_states) ? ;
        object.set_named_property("multiplayerCorrelationId",
        self.multiplayer_correlation_id.to_owned()) ? ;
        object.set_named_property("serverAuthoritativeInventory",
        self.server_authoritative_inventory.to_owned()) ? ;
        object.set_named_property("engine", self.engine.to_owned()) ? ;
        object.set_named_property("propertyData",
        self.property_data.to_owned()) ? ;
        object.set_named_property("blockPalletteChecksum",
        self.block_pallette_checksum.to_owned()) ? ;
        object.set_named_property("worldTemplateId",
        self.world_template_id.to_owned()) ? ;
        object.set_named_property("clientSideGeneration",
        self.client_side_generation.to_owned()) ? ;
        object.set_named_property("blockNetworkIdsAreHashes",
        self.block_network_ids_are_hashes.to_owned()) ? ;
        object.set_named_property("serverControlledSound",
        self.server_controlled_sound.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let entity_id : ZigZong = match data.get_named_property("entityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "entityId",
            stringify! (ZigZong), err),))
        } ; let runtime_entity_id : VarLong = match
        data.get_named_property("runtimeEntityId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "runtimeEntityId", stringify! (VarLong), err),))
        } ; let player_gamemode : ZigZag = match
        data.get_named_property("playerGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerGamemode", stringify! (ZigZag), err),))
        } ; let player_position : Vec3f = match Vec3f ::
        from_object(data.get_named_property("playerPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerPosition", stringify! (Vec3f), err),))
        } ; let player_rotation : Vec2f = match Vec2f ::
        from_object(data.get_named_property("playerRotation") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "playerRotation", stringify! (Vec2f), err),))
        } ; let seed : LU64 = match data.get_named_property("seed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "seed",
            stringify! (LU64), err),))
        } ; let biome_type : LI16 = match data.get_named_property("biomeType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeType",
            stringify! (LI16), err),))
        } ; let biome_name : String = match
        data.get_named_property("biomeName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "biomeName",
            stringify! (String), err),))
        } ; let dimension : ZigZag = match
        data.get_named_property("dimension")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "dimension",
            stringify! (ZigZag), err),))
        } ; let generator : ZigZag = match
        data.get_named_property("generator")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "generator",
            stringify! (ZigZag), err),))
        } ; let world_gamemode : ZigZag = match
        data.get_named_property("worldGamemode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldGamemode",
            stringify! (ZigZag), err),))
        } ; let difficulty : ZigZag = match
        data.get_named_property("difficulty")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "difficulty",
            stringify! (ZigZag), err),))
        } ; let spawn_position : BlockCoordinates = match BlockCoordinates ::
        from_object(data.get_named_property("spawnPosition") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "spawnPosition",
            stringify! (BlockCoordinates), err),))
        } ; let achievements_disabled : bool = match
        data.get_named_property("achievementsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "achievementsDisabled", stringify! (bool), err),))
        } ; let editor_world : bool = match
        data.get_named_property("editorWorld")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editorWorld",
            stringify! (bool), err),))
        } ; let created_in_editor : bool = match
        data.get_named_property("createdInEditor")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "createdInEditor", stringify! (bool), err),))
        } ; let day_cycle_stop_time : ZigZag = match
        data.get_named_property("dayCycleStopTime")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "dayCycleStopTime", stringify! (ZigZag), err),))
        } ; let edu_offer : ZigZag = match data.get_named_property("eduOffer")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduOffer",
            stringify! (ZigZag), err),))
        } ; let edu_features_enabled : bool = match
        data.get_named_property("eduFeaturesEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduFeaturesEnabled", stringify! (bool), err),))
        } ; let edu_product_id : String = match
        data.get_named_property("eduProductId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "eduProductId",
            stringify! (String), err),))
        } ; let rain_level : LF32 = match data.get_named_property("rainLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "rainLevel",
            stringify! (LF32), err),))
        } ; let lightning_level : LF32 = match
        data.get_named_property("lightningLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "lightningLevel", stringify! (LF32), err),))
        } ; let confirmed_platform_locked_content : bool = match
        data.get_named_property("confirmedPlatformLockedContent")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "confirmedPlatformLockedContent", stringify! (bool), err),))
        } ; let multiplayer_game : bool = match
        data.get_named_property("multiplayerGame")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerGame", stringify! (bool), err),))
        } ; let broadcast_to_lan : bool = match
        data.get_named_property("broadcastToLan")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "broadcastToLan", stringify! (bool), err),))
        } ; let xbl_broadcast_mode : VarInt = match
        data.get_named_property("xblBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "xblBroadcastMode", stringify! (VarInt), err),))
        } ; let platform_broadcast_mode : VarInt = match
        data.get_named_property("platformBroadcastMode")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "platformBroadcastMode", stringify! (VarInt), err),))
        } ; let enable_commands : bool = match
        data.get_named_property("enableCommands")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enableCommands", stringify! (bool), err),))
        } ; let texture_packs_required : bool = match
        data.get_named_property("texturePacksRequired")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "texturePacksRequired", stringify! (bool), err),))
        } ; let gamerules_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("gamerules")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gamerules",
            stringify! (Vec < GameRule >), err),))
        } ; let mut gamerules : Vec < GameRule > = Vec :: new() ; for item in
        gamerules_napi { gamerules.push(GameRule :: from_object(item) ?) ; }
        let experiments_napi : Vec < napi :: bindgen_prelude :: Object > =
        match data.get_named_property("experiments")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "experiments",
            stringify! (Vec < Experiment >), err),))
        } ; let mut experiments : Vec < Experiment > = Vec :: new() ; for item
        in experiments_napi
        { experiments.push(Experiment :: from_object(item) ?) ; } let
        experiments_previously_used : bool = match
        data.get_named_property("experimentsPreviouslyUsed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentsPreviouslyUsed", stringify! (bool), err),))
        } ; let bonus_chest : bool = match
        data.get_named_property("bonusChest")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "bonusChest",
            stringify! (bool), err),))
        } ; let map_enabled : bool = match
        data.get_named_property("mapEnabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "mapEnabled",
            stringify! (bool), err),))
        } ; let permission_level : u8 = match
        data.get_named_property("permissionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "permissionLevel", stringify! (u8), err),))
        } ; let server_chunk_tick_range : LI32 = match
        data.get_named_property("serverChunkTickRange")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverChunkTickRange", stringify! (LI32), err),))
        } ; let has_locked_behaviour_packs : bool = match
        data.get_named_property("hasLockedBehaviourPacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedBehaviourPacks", stringify! (bool), err),))
        } ; let has_locked_resource_packs : bool = match
        data.get_named_property("hasLockedResourcePacks")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "hasLockedResourcePacks", stringify! (bool), err),))
        } ; let from_locked_world_template : bool = match
        data.get_named_property("fromLockedWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromLockedWorldTemplate", stringify! (bool), err),))
        } ; let msa_gamertags_only : bool = match
        data.get_named_property("msaGamertagsOnly")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "msaGamertagsOnly", stringify! (bool), err),))
        } ; let from_world_template : bool = match
        data.get_named_property("fromWorldTemplate")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "fromWorldTemplate", stringify! (bool), err),))
        } ; let world_template_option_locked : bool = match
        data.get_named_property("worldTemplateOptionLocked")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateOptionLocked", stringify! (bool), err),))
        } ; let only_spawn_v1_villagers : bool = match
        data.get_named_property("onlySpawnV1Villagers")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "onlySpawnV1Villagers", stringify! (bool), err),))
        } ; let persona_disabled : bool = match
        data.get_named_property("personaDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "personaDisabled", stringify! (bool), err),))
        } ; let custom_skins_disabled : bool = match
        data.get_named_property("customSkinsDisabled")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "customSkinsDisabled", stringify! (bool), err),))
        } ; let emote_chat_muted : bool = match
        data.get_named_property("emoteChatMuted")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "emoteChatMuted", stringify! (bool), err),))
        } ; let game_version : String = match
        data.get_named_property("gameVersion")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "gameVersion",
            stringify! (String), err),))
        } ; let limited_world_width : LI32 = match
        data.get_named_property("limitedWorldWidth")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldWidth", stringify! (LI32), err),))
        } ; let limited_world_length : LI32 = match
        data.get_named_property("limitedWorldLength")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "limitedWorldLength", stringify! (LI32), err),))
        } ; let new_nether : bool = match data.get_named_property("newNether")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "newNether",
            stringify! (bool), err),))
        } ; let edu_resource_uri : EducationSharedResource = match
        EducationSharedResource ::
        from_object(data.get_named_property("eduResourceUri") ?)
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "eduResourceUri", stringify! (EducationSharedResource), err),))
        } ; let experimental_gameplay_override : bool = match
        data.get_named_property("experimentalGameplayOverride")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "experimentalGameplayOverride", stringify! (bool), err),))
        } ; let chat_restriction_level : u8 = match
        data.get_named_property("chatRestrictionLevel")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "chatRestrictionLevel", stringify! (u8), err),))
        } ; let disable_player_interactions : bool = match
        data.get_named_property("disablePlayerInteractions")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "disablePlayerInteractions", stringify! (bool), err),))
        } ; let level_id : String = match data.get_named_property("levelId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "levelId",
            stringify! (String), err),))
        } ; let world_name : String = match
        data.get_named_property("worldName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "worldName",
            stringify! (String), err),))
        } ; let premium_world_template_id : String = match
        data.get_named_property("premiumWorldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "premiumWorldTemplateId", stringify! (String), err),))
        } ; let is_trial : bool = match data.get_named_property("isTrial")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "isTrial",
            stringify! (bool), err),))
        } ; let movement_authority : ZigZag = match
        data.get_named_property("movementAuthority")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "movementAuthority", stringify! (ZigZag), err),))
        } ; let rewind_history_size : ZigZag = match
        data.get_named_property("rewindHistorySize")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "rewindHistorySize", stringify! (ZigZag), err),))
        } ; let server_authoritive_block_breaking : bool = match
        data.get_named_property("serverAuthoritiveBlockBreaking")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritiveBlockBreaking", stringify! (bool), err),))
        } ; let current_tick : LI64 = match
        data.get_named_property("currentTick")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "currentTick",
            stringify! (LI64), err),))
        } ; let enchantment_seed : ZigZag = match
        data.get_named_property("enchantmentSeed")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "enchantmentSeed", stringify! (ZigZag), err),))
        } ; let block_properties_napi : Vec < napi :: bindgen_prelude ::
        Object > = match data.get_named_property("blockProperties")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockProperties", stringify! (Vec < BlockProperty >), err),))
        } ; let mut block_properties : Vec < BlockProperty > = Vec :: new() ;
        for item in block_properties_napi
        { block_properties.push(BlockProperty :: from_object(item) ?) ; } let
        item_states_napi : Vec < napi :: bindgen_prelude :: Object > = match
        data.get_named_property("itemStates")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "itemStates",
            stringify! (Vec < ItemState >), err),))
        } ; let mut item_states : Vec < ItemState > = Vec :: new() ; for item
        in item_states_napi
        { item_states.push(ItemState :: from_object(item) ?) ; } let
        multiplayer_correlation_id : String = match
        data.get_named_property("multiplayerCorrelationId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "multiplayerCorrelationId", stringify! (String), err),))
        } ; let server_authoritative_inventory : bool = match
        data.get_named_property("serverAuthoritativeInventory")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverAuthoritativeInventory", stringify! (bool), err),))
        } ; let engine : String = match data.get_named_property("engine")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "engine",
            stringify! (String), err),))
        } ; let property_data : NBT = match
        data.get_named_property("propertyData")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "propertyData",
            stringify! (NBT), err),))
        } ; let block_pallette_checksum : LU64 = match
        data.get_named_property("blockPalletteChecksum")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockPalletteChecksum", stringify! (LU64), err),))
        } ; let world_template_id : UUID = match
        data.get_named_property("worldTemplateId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "worldTemplateId", stringify! (UUID), err),))
        } ; let client_side_generation : bool = match
        data.get_named_property("clientSideGeneration")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "clientSideGeneration", stringify! (bool), err),))
        } ; let block_network_ids_are_hashes : bool = match
        data.get_named_property("blockNetworkIdsAreHashes")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "blockNetworkIdsAreHashes", stringify! (bool), err),))
        } ; let server_controlled_sound : bool = match
        data.get_named_property("serverControlledSound")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "serverControlledSound", stringify! (bool), err),))
        } ;
        Ok(Self
        {
            entity_id, runtime_entity_id, player_gamemode, player_position,
            player_rotation, seed, biome_type, biome_name, dimension,
            generator, world_gamemode, difficulty, spawn_position,
            achievements_disabled, editor_world, created_in_editor,
            day_cycle_stop_time, edu_offer, edu_features_enabled,
            edu_product_id, rain_level, lightning_level,
            confirmed_platform_locked_content, multiplayer_game,
            broadcast_to_lan, xbl_broadcast_mode, platform_broadcast_mode,
            enable_commands, texture_packs_required, gamerules, experiments,
            experiments_previously_used, bonus_chest, map_enabled,
            permission_level, server_chunk_tick_range,
            has_locked_behaviour_packs, has_locked_resource_packs,
            from_locked_world_template, msa_gamertags_only,
            from_world_template, world_template_option_locked,
            only_spawn_v1_villagers, persona_disabled, custom_skins_disabled,
            emote_chat_muted, game_version, limited_world_width,
            limited_world_length, new_nether, edu_resource_uri,
            experimental_gameplay_override, chat_restriction_level,
            disable_player_interactions, level_id, world_name,
            premium_world_template_id, is_trial, movement_authority,
            rewind_history_size, server_authoritive_block_breaking,
            current_tick, enchantment_seed, block_properties, item_states,
            multiplayer_correlation_id, server_authoritative_inventory,
            engine, property_data, block_pallette_checksum, world_template_id,
            client_side_generation, block_network_ids_are_hashes,
            server_controlled_sound
        })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockCoordinates { pub x : ZigZag, pub y : VarInt, pub z : ZigZag, } impl
crate :: packets :: prelude :: PacketChildSerialization for BlockCoordinates
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_zigzag(self.x.to_owned()) ? ;
        stream.write_varint(self.y.to_owned()) ? ;
        stream.write_zigzag(self.z.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let x : ZigZag = stream.read_zigzag() ? ;
        let y : VarInt = stream.read_varint() ? ; let z : ZigZag =
        stream.read_zigzag() ? ; Ok(Self { x, y, z })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockCoordinates
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("x", self.x.to_owned()) ? ;
        object.set_named_property("y", self.y.to_owned()) ? ;
        object.set_named_property("z", self.z.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let x : ZigZag = match data.get_named_property("x")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "x", stringify!
            (ZigZag), err),))
        } ; let y : VarInt = match data.get_named_property("y")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "y", stringify!
            (VarInt), err),))
        } ; let z : ZigZag = match data.get_named_property("z")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "z", stringify!
            (ZigZag), err),))
        } ; Ok(Self { x, y, z })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
GameRule
{
    pub name : String, pub editable : bool, #[napi(ts_type = "GameRuleType")]
    pub field_type : VarInt, #[napi(ts_type = "boolean | number")] pub value :
    Value,
} impl crate :: packets :: prelude :: PacketConversion for GameRule
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("editable", self.editable.to_owned()) ? ;
        object.set_named_property("fieldType", self.field_type.to_owned()) ? ;
        object.set_named_property("value", self.value.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let editable : bool = match data.get_named_property("editable")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "editable",
            stringify! (bool), err),))
        } ; let field_type : VarInt = match
        data.get_named_property("fieldType")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "fieldType",
            stringify! (VarInt), err),))
        } ; let value : Value = match data.get_named_property("value")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "value",
            stringify! (Value), err),))
        } ; Ok(Self { name, editable, field_type, value })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
EducationSharedResource { pub button_name : String, pub link_uri : String, }
impl crate :: packets :: prelude :: PacketChildSerialization for
EducationSharedResource
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.button_name.to_owned()) ? ;
        stream.write_string(self.link_uri.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let button_name : String =
        stream.read_string() ? ; let link_uri : String = stream.read_string()
        ? ; Ok(Self { button_name, link_uri })
    }
} impl crate :: packets :: prelude :: PacketConversion for
EducationSharedResource
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("buttonName", self.button_name.to_owned()) ?
        ; object.set_named_property("linkUri", self.link_uri.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let button_name : String = match data.get_named_property("buttonName")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "buttonName",
            stringify! (String), err),))
        } ; let link_uri : String = match data.get_named_property("linkUri")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "linkUri",
            stringify! (String), err),))
        } ; Ok(Self { button_name, link_uri })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
BlockProperty { pub name : String, pub state : NBT, } impl crate :: packets ::
prelude :: PacketChildSerialization for BlockProperty
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.append(& mut self.state.serialize() ?) ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let state : NBT = NBT :: deserialize(& mut stream) ? ;
        Ok(Self { name, state })
    }
} impl crate :: packets :: prelude :: PacketConversion for BlockProperty
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("state", self.state.to_owned()) ? ;
        Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let state : NBT = match data.get_named_property("state")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "state",
            stringify! (NBT), err),))
        } ; Ok(Self { name, state })
    }
}#[napi(object)] #[derive(protocol_derive :: PacketFieldAttributes)] pub struct
ItemState
{ pub name : String, pub runtime_id : LI16, pub component_based : bool, } impl
crate :: packets :: prelude :: PacketChildSerialization for ItemState
{
    fn serialize(& self) -> napi :: bindgen_prelude :: Result < crate ::
    binary :: BinaryStream >
    {
        let mut stream = crate :: binary :: BinaryStream :: new() ;
        stream.write_string(self.name.to_owned()) ? ;
        stream.write_li16(self.runtime_id.to_owned()) ? ;
        stream.write_bool(self.component_based.to_owned()) ? ; Ok(stream)
    } fn deserialize(stream : & mut crate :: binary :: BinaryStream) -> napi
    :: bindgen_prelude :: Result < Self >
    {
        let mut stream = stream ; let name : String = stream.read_string() ? ;
        let runtime_id : LI16 = stream.read_li16() ? ; let component_based :
        bool = stream.read_bool() ? ;
        Ok(Self { name, runtime_id, component_based })
    }
} impl crate :: packets :: prelude :: PacketConversion for ItemState
{
    fn to_object(& self, env : napi :: bindgen_prelude :: Env) -> napi ::
    bindgen_prelude :: Result < napi :: bindgen_prelude :: Object >
    {
        let mut object = env.create_object() ? ;
        object.set_named_property("name", self.name.to_owned()) ? ;
        object.set_named_property("runtimeId", self.runtime_id.to_owned()) ? ;
        object.set_named_property("componentBased",
        self.component_based.to_owned()) ? ; Ok(object)
    } fn from_object(data : napi :: bindgen_prelude :: Object) -> napi ::
    bindgen_prelude :: Result < Self >
    {
        let name : String = match data.get_named_property("name")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "name",
            stringify! (String), err),))
        } ; let runtime_id : LI16 = match data.get_named_property("runtimeId")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}", "runtimeId",
            stringify! (LI16), err),))
        } ; let component_based : bool = match
        data.get_named_property("componentBased")
        {
            Ok(value) => value, Err(err) => return
            Err(napi :: Error ::
            new(err.status, format!
            ("Expected property '{}' to be of type '{}'\n{}",
            "componentBased", stringify! (bool), err),))
        } ; Ok(Self { name, runtime_id, component_based })
    }
}