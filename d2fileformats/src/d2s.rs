use crate::errors::*;

#[derive(Clone)]
pub struct D2s {

}

impl D2s {
    pub fn from(file_bytes: &[u8]) -> Result<D2s> {
        Ok(D2s {})
    }
}

pub struct D2sHeader {
    pub version: u32,
    pub character_name: String,
    pub hardcore_flags: u8,
    pub num_acts_beaten: u8,
    pub active_weapon_set: u16,
    pub unknown1: u16,
    pub unknown2: u32,
    pub character_class: u16,
    pub character_menu_level: u16,
    pub character_menu_appearance: CharacterMenuAppearance,
    pub hotkeys: [u8;16],
    pub skill_left: u8,
    pub skill_right: u8,
    pub last_played_act_difficulty: u16,
    pub unknown3: [u8;36],
    pub map_seed: u32,
    pub unknown4: [u8;8],
    pub quest_status_normal: QuestStatus,
    pub quest_status_nightmare: QuestStatus,
    pub quest_status_hell: QuestStatus,
}

pub struct CharacterMenuAppearance {
    pub head: u8,
    pub torso: u8,
    pub legs: u8,
    pub arm_right: u8,
    pub arm_left: u8,
    pub hand_right: u8,
    pub hand_left: u8,
    pub shield: u8,
    pub shoulder_right: u8,
    pub shoulder_left: u8,
    pub reserved1: [u8;6],

    pub head_tint: u8,
    pub torso_tint: u8,
    pub legs_tint: u8,
    pub arm_right_tint: u8,
    pub arm_left_tint: u8,
    pub hand_right_tint: u8,
    pub hand_left_tint: u8,
    pub shield_tint: u8,
    pub shoulder_right_tint: u8,
    pub shoulder_left_tint: u8,
    pub reserved2: [u8;6],
}

pub struct QuestStatus {
    pub act_1: Act1QuestStatus,
    pub act_2: Act2QuestStatus,
    pub act_3: Act3QuestStatus,
    pub act_4: Act4QuestStatus,
    pub act_5: Act5QuestStatus,
    pub reserved: [u8;10],
}

pub struct Act1QuestStatus {
    pub enable_a: u16,
    pub enable_b: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
}

pub struct Act2QuestStatus {
    pub enable_a: u16,
    pub enable_b: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
}

pub struct Act3QuestStatus {
    pub enable_a: u16,
    pub enable_b: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
}

pub struct Act4QuestStatus {
    pub enable_a: u16,
    pub enable_b: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
}

pub struct Act5QuestStatus {
    pub enable_a: u16,
    pub reserved1: [u8;6],
    pub enable_b: u16,
    pub reserved2: [u8;4],
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
}