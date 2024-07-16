#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ETankPacketType {
    NetGamePacketState,
    NetGamePacketCallFunction,
    NetGamePacketUpdateStatus,
    NetGamePacketTileChangeRequest,
    NetGamePacketSendMapData,
    NetGamePacketSendTileUpdateData,
    NetGamePacketSendTileUpdateDataMultiple,
    NetGamePacketTileActivateRequest,
    NetGamePacketTileApplyDamage,
    NetGamePacketSendInventoryState,
    NetGamePacketItemActivateRequest,
    NetGamePacketItemActivateObjectRequest,
    NetGamePacketSendTileTreeState,
    NetGamePacketModifyItemInventory,
    NetGamePacketItemChangeObject,
    NetGamePacketSendLock,
    NetGamePacketSendItemDatabaseData,
    NetGamePacketSendParticleEffect,
    NetGamePacketSetIconState,
    NetGamePacketItemEffect,
    NetGamePacketSetCharacterState,
    NetGamePacketPingReply,
    NetGamePacketPingRequest,
    NetGamePacketGotPunched,
    NetGamePacketAppCheckResponse,
    NetGamePacketAppIntegrityFail,
    NetGamePacketDisconnect,
    NetGamePacketBattleJoin,
    NetGamePacketBattleEvent,
    NetGamePacketUseDoor,
    NetGamePacketSendParental,
    NetGamePacketGoneFishin,
    NetGamePacketSteam,
    NetGamePacketPetBattle,
    NetGamePacketNpc,
    NetGamePacketSpecial,
    NetGamePacketSendParticleEffectV2,
    NetGamePacketActiveArrowToItem,
    NetGamePacketSelectTileIndex,
    NetGamePacketSendPlayerTributeData,
}

impl From<u8> for ETankPacketType {
    fn from(value: u8) -> Self {
        match value {
            0 => ETankPacketType::NetGamePacketState,
            1 => ETankPacketType::NetGamePacketCallFunction,
            2 => ETankPacketType::NetGamePacketUpdateStatus,
            3 => ETankPacketType::NetGamePacketTileChangeRequest,
            4 => ETankPacketType::NetGamePacketSendMapData,
            5 => ETankPacketType::NetGamePacketSendTileUpdateData,
            6 => ETankPacketType::NetGamePacketSendTileUpdateDataMultiple,
            7 => ETankPacketType::NetGamePacketTileActivateRequest,
            8 => ETankPacketType::NetGamePacketTileApplyDamage,
            9 => ETankPacketType::NetGamePacketSendInventoryState,
            10 => ETankPacketType::NetGamePacketItemActivateRequest,
            11 => ETankPacketType::NetGamePacketItemActivateObjectRequest,
            12 => ETankPacketType::NetGamePacketSendTileTreeState,
            13 => ETankPacketType::NetGamePacketModifyItemInventory,
            14 => ETankPacketType::NetGamePacketItemChangeObject,
            15 => ETankPacketType::NetGamePacketSendLock,
            16 => ETankPacketType::NetGamePacketSendItemDatabaseData,
            17 => ETankPacketType::NetGamePacketSendParticleEffect,
            18 => ETankPacketType::NetGamePacketSetIconState,
            19 => ETankPacketType::NetGamePacketItemEffect,
            20 => ETankPacketType::NetGamePacketSetCharacterState,
            21 => ETankPacketType::NetGamePacketPingReply,
            22 => ETankPacketType::NetGamePacketPingRequest,
            23 => ETankPacketType::NetGamePacketGotPunched,
            24 => ETankPacketType::NetGamePacketAppCheckResponse,
            25 => ETankPacketType::NetGamePacketAppIntegrityFail,
            26 => ETankPacketType::NetGamePacketDisconnect,
            27 => ETankPacketType::NetGamePacketBattleJoin,
            28 => ETankPacketType::NetGamePacketBattleEvent,
            29 => ETankPacketType::NetGamePacketUseDoor,
            30 => ETankPacketType::NetGamePacketSendParental,
            31 => ETankPacketType::NetGamePacketGoneFishin,
            32 => ETankPacketType::NetGamePacketSteam,
            33 => ETankPacketType::NetGamePacketPetBattle,
            34 => ETankPacketType::NetGamePacketNpc,
            35 => ETankPacketType::NetGamePacketSpecial,
            36 => ETankPacketType::NetGamePacketSendParticleEffectV2,
            37 => ETankPacketType::NetGamePacketActiveArrowToItem,
            38 => ETankPacketType::NetGamePacketSelectTileIndex,
            39 => ETankPacketType::NetGamePacketSendPlayerTributeData,
            _ => ETankPacketType::NetGamePacketState,
        }
    }
}