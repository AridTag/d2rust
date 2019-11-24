use std::io::{Cursor, SeekFrom, Seek};
use byteorder::{LittleEndian, ReadBytesExt};
use std::fmt::{Debug, Formatter};
use crate::errors::*;
use crate::read_string::*;

fn is_reader_at_end(reader: &mut Cursor<&[u8]>) -> bool {
    let current_pos = reader.position();
    if let Err(_) = reader.seek(SeekFrom::End(0)) {
        return true;
    }
    let end_pos = reader.position();
    reader.seek(SeekFrom::Start(current_pos)).expect("this shouldn't fail (famous last words)");

    return current_pos < end_pos;
}

#[derive(Clone)]
pub struct D2s {
    pub version: u32,
    pub file_size: u32,
    pub checksum: u32,
    pub active_weapon_set: u32,
    pub character_name: String,
    pub character_status: u8,
    pub num_acts_beaten: u8,
    pub unknown1: u16,
    pub character_class: u8,
    pub unknown2: u16,
    pub character_menu_level: u8,
    pub unknown3: u32,
    pub timestamp: u32,
    pub unknown4: u32,

    pub hotkeys: [u32;16],
    pub skill_left: u32,
    pub skill_right: u32,
    pub alternate_skill_left: u32,
    pub alternate_skill_right: u32,
    pub character_menu_appearance: CharacterMenuAppearance,
    pub difficulty_normal: u8,
    pub difficulty_nightmare: u8,
    pub difficulty_hell: u8,
    pub map_seed: u32,
    pub unknown6: u16,
    pub mercenary_dead: u16,
    pub mercenary_seed: u32,
    pub mercenary_name_index: u16,
    pub mercenary_type: u16,
    pub mercenary_exp: u32,
    pub unknown8: [u8;144],
    pub quests: QuestStatus,
}

impl Default for D2s {
    fn default() -> Self {
        D2s {
            version: 96,
            file_size: 0,
            checksum: 0,
            active_weapon_set: 0,
            character_name: String::new(),
            character_status: 0,
            num_acts_beaten: 0,
            unknown1: 0,
            character_class: 0,
            unknown2: 0,
            character_menu_level: 1,
            unknown3: 0,
            timestamp: 0,
            unknown4: 0,
            hotkeys: [0u32;16],
            skill_left: 0,
            skill_right: 0,
            alternate_skill_left: 0,
            alternate_skill_right: 0,
            character_menu_appearance: Default::default(),
            difficulty_normal: 0,
            difficulty_nightmare: 0,
            difficulty_hell: 0,
            map_seed: 0,
            unknown6: 0,
            mercenary_dead: 0,
            mercenary_seed: 0,
            mercenary_name_index: 0,
            mercenary_type: 0,
            mercenary_exp: 0,
            unknown8: [0u8; 144],
            quests: Default::default(),
        }
    }
}

impl D2s {
    pub fn from(file_bytes: &[u8]) -> Result<D2s> {
        let mut reader = Cursor::new(file_bytes);
        let magic_header = reader.read_u32::<LittleEndian>()?;
        if magic_header != 0xAA55AA55 {
            bail!("file is not a Diablo II save file");
        }

        let version = reader.read_u32::<LittleEndian>()?;
        // TODO: handle different versions

        let file_size = reader.read_u32::<LittleEndian>()?;
        let checksum = reader.read_u32::<LittleEndian>()?;
        let active_weapon_set = reader.read_u32::<LittleEndian>()?;
        let character_name = std::str::from_utf8(&reader.read_bytes(16)?)?.to_string();
        let character_status = reader.read_u8()?;
        let num_acts_beaten = reader.read_u8()?;
        let unknown1 = reader.read_u16::<LittleEndian>()?;
        let character_class = reader.read_u8()?;
        let unknown2 = reader.read_u16::<LittleEndian>()?;
        let character_menu_level = reader.read_u8()?;
        let unknown3 = reader.read_u32::<LittleEndian>()?;
        let timestamp = reader.read_u32::<LittleEndian>()?;
        let unknown4 = reader.read_u32::<LittleEndian>()?;
        let mut hotkeys = [0u32; 16];
        for i in 0..hotkeys.len() {
            hotkeys[i] = reader.read_u32::<LittleEndian>()?;
        }
        let skill_left = reader.read_u32::<LittleEndian>()?;
        let skill_right = reader.read_u32::<LittleEndian>()?;
        let alternate_skill_left = reader.read_u32::<LittleEndian>()?;
        let alternate_skill_right = reader.read_u32::<LittleEndian>()?;
        let character_menu_appearance = CharacterMenuAppearance::from(&mut reader)?;
        let difficulty_normal = reader.read_u8()?;
        let difficulty_nightmare = reader.read_u8()?;
        let difficulty_hell = reader.read_u8()?;
        let map_seed = reader.read_u32::<LittleEndian>()?;
        let unknown6 = reader.read_u16::<LittleEndian>()?;
        let mercenary_dead = reader.read_u16::<LittleEndian>()?;
        let mercenary_seed = reader.read_u32::<LittleEndian>()?;
        let mercenary_name_index = reader.read_u16::<LittleEndian>()?;
        let mercenary_type = reader.read_u16::<LittleEndian>()?;
        let mercenary_exp = reader.read_u32::<LittleEndian>()?;
        let mut unknown8 = [0u8;144];
        for i in 0..unknown8.len() {
            unknown8[i] = reader.read_u8()?;
        }

        if is_reader_at_end(&mut reader) {
            let result = D2s
                {
                    version,
                    file_size,
                    checksum,
                    active_weapon_set,
                    character_name,
                    character_status,
                    num_acts_beaten,
                    unknown1,
                    character_class,
                    unknown2,
                    character_menu_level,
                    unknown3,
                    timestamp,
                    unknown4,
                    hotkeys,
                    skill_left,
                    skill_right,
                    alternate_skill_left,
                    alternate_skill_right,
                    character_menu_appearance,
                    difficulty_normal,
                    difficulty_nightmare,
                    difficulty_hell,
                    map_seed,
                    unknown6,
                    mercenary_dead,
                    mercenary_seed,
                    mercenary_name_index,
                    mercenary_type,
                    mercenary_exp,
                    unknown8,
                    quests: Default::default()
                };

            //println!("{:?}", result);

            return Ok(result);
        }

         // TODO: more reading
        let quests = QuestStatus::from(&mut reader)?;

        let result = D2s
            {
                version,
                file_size,
                checksum,
                active_weapon_set,
                character_name,
                character_status,
                num_acts_beaten,
                unknown1,
                character_class,
                unknown2,
                character_menu_level,
                unknown3,
                timestamp,
                unknown4,
                hotkeys,
                skill_left,
                skill_right,
                alternate_skill_left,
                alternate_skill_right,
                character_menu_appearance,
                difficulty_normal,
                difficulty_nightmare,
                difficulty_hell,
                map_seed,
                unknown6,
                mercenary_dead,
                mercenary_seed,
                mercenary_name_index,
                mercenary_type,
                mercenary_exp,
                unknown8,
                quests,
            };

        println!("{:?}", result);

        Ok(result)
    }
}

impl Debug for D2s {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "version          : {}", self.version)?;
        writeln!(f, "file size        : {}", self.file_size)?;
        writeln!(f, "checksum         : {}", self.checksum)?;
        writeln!(f, "active weapon set: {}", self.active_weapon_set)?;
        writeln!(f, "name             : {}", self.character_name)?;
        writeln!(f, "status           : {:#b}", self.character_status)?;
        writeln!(f, "acts beaten      : {}", self.num_acts_beaten)?;
        writeln!(f, "unknown1         : {:#06X}", self.unknown1)?;
        writeln!(f, "class            : {:#04X}", self.character_class)?;
        writeln!(f, "unknown2         : {:#06X}", self.unknown2)?;
        writeln!(f, "level (in menu)  : {}", self.character_menu_level)?;
        writeln!(f, "unknown3         : {:#10X}", self.unknown3)?;
        writeln!(f, "timestamp        : {}", self.timestamp)?;
        writeln!(f, "unknown4         : {:#10X}", self.unknown4)?;
        write!  (f, "hotkeys          : [")?;
        for i in 0..self.hotkeys.len() {
            write!(f, "{:#10X}", self.hotkeys[i])?;
            if i < self.hotkeys.len() - 1 {
                write!(f, ", ")?;
            }
        }
        writeln!(f, "]")?;
        writeln!(f, "skill_left       : {:#10X}", self.skill_left)?;
        writeln!(f, "skill_right      : {:#10X}", self.skill_right)?;
        writeln!(f, "alt_skill_left   : {:#10X}", self.alternate_skill_left)?;
        writeln!(f, "alt_skill_right  : {:#10X}", self.alternate_skill_right)?;
        write!(f, "{:?}", self.character_menu_appearance)?;
        writeln!(f, "difficulty_norm  : {:#b}", self.difficulty_normal)?;
        writeln!(f, "difficulty_nm    : {:#b}", self.difficulty_nightmare)?;
        writeln!(f, "difficulty_hell  : {:#b}", self.difficulty_hell)?;
        writeln!(f, "map_seed         : {:#10X}", self.map_seed)?;
        writeln!(f, "mercenary_dead   : {:#06X}", self.mercenary_dead)?;
        writeln!(f, "mercenary_seed   : {:#10X}", self.mercenary_seed)?;
        writeln!(f, "mercenary_name   : {}", self.mercenary_name_index)?;
        writeln!(f, "mercenary_type   : {:#06X}", self.mercenary_type)?;
        writeln!(f, "mercenary_exp    : {}", self.mercenary_exp)?;
        write!(f, "{:?}", self.quests)?;

        Ok(())
    }
}

#[derive(Clone)]
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

impl Default for CharacterMenuAppearance {
    fn default() -> Self {
        CharacterMenuAppearance {
            head: 0xFF,
            torso: 0xFF,
            legs: 0xFF,
            arm_right: 0xFF,
            arm_left: 0xFF,
            hand_right: 0xFF,
            hand_left: 0xFF,
            shield: 0xFF,
            shoulder_right: 0xFF,
            shoulder_left: 0xFF,
            reserved1: [0xFFu8;6],

            head_tint: 0xFF,
            torso_tint: 0xFF,
            legs_tint: 0xFF,
            arm_right_tint: 0xFF,
            arm_left_tint: 0xFF,
            hand_right_tint: 0xFF,
            hand_left_tint: 0xFF,
            shield_tint: 0xFF,
            shoulder_right_tint: 0xFF,
            shoulder_left_tint: 0xFF,
            reserved2: [0xFFu8;6]
        }
    }
}

impl Debug for CharacterMenuAppearance {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "menu appearance")?;
        writeln!(f, "  head           : {:#4X}  tint : {:#4X}", self.head, self.head_tint)?;
        writeln!(f, "  torso          : {:#4X}  tint : {:#4X}", self.torso, self.torso_tint)?;
        writeln!(f, "  legs           : {:#4X}  tint : {:#4X}", self.legs, self.legs_tint)?;
        writeln!(f, "  arm_right      : {:#4X}  tint : {:#4X}", self.arm_right, self.arm_right_tint)?;
        writeln!(f, "  arm_left       : {:#4X}  tint : {:#4X}", self.arm_left, self.arm_left_tint)?;
        writeln!(f, "  hand_right     : {:#4X}  tint : {:#4X}", self.hand_right, self.hand_right_tint)?;
        writeln!(f, "  hand_left      : {:#4X}  tint : {:#4X}", self.hand_left, self.hand_left_tint)?;
        writeln!(f, "  shield         : {:#4X}  tint : {:#4X}", self.shield, self.shield_tint)?;
        writeln!(f, "  shoulder_right : {:#4X}  tint : {:#4X}", self.shoulder_right, self.shoulder_right_tint)?;
        writeln!(f, "  shoulder_left  : {:#4X}  tint : {:#4X}", self.shoulder_left, self.shoulder_left_tint)?;

        Ok(())
    }
}

impl CharacterMenuAppearance {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<CharacterMenuAppearance> {
        let head = reader.read_u8()?;
        let torso = reader.read_u8()?;
        let legs = reader.read_u8()?;
        let arm_right = reader.read_u8()?;
        let arm_left = reader.read_u8()?;
        let hand_right = reader.read_u8()?;
        let hand_left = reader.read_u8()?;
        let shield = reader.read_u8()?;
        let shoulder_right = reader.read_u8()?;
        let shoulder_left = reader.read_u8()?;
        let mut reserved1 = [0u8;6];
        for b in &mut reserved1 {
            *b = reader.read_u8()?;
        }
        let head_tint = reader.read_u8()?;
        let torso_tint = reader.read_u8()?;
        let legs_tint = reader.read_u8()?;
        let arm_right_tint = reader.read_u8()?;
        let arm_left_tint = reader.read_u8()?;
        let hand_right_tint = reader.read_u8()?;
        let hand_left_tint = reader.read_u8()?;
        let shield_tint = reader.read_u8()?;
        let shoulder_right_tint = reader.read_u8()?;
        let shoulder_left_tint = reader.read_u8()?;
        let mut reserved2 = [0u8;6];
        for b in &mut reserved2 {
            *b = reader.read_u8()?;
        }

        Ok(CharacterMenuAppearance {
            head,
            torso,
            legs,
            arm_right,
            arm_left,
            hand_right,
            hand_left,
            shield,
            shoulder_right,
            shoulder_left,
            reserved1,
            head_tint,
            torso_tint,
            legs_tint,
            arm_right_tint,
            arm_left_tint,
            hand_right_tint,
            hand_left_tint,
            shield_tint,
            shoulder_right_tint,
            shoulder_left_tint,
            reserved2
        })
    }
}

#[derive(Clone)]
pub struct QuestStatus {
    pub identifier: [u8;4],
    pub unknown: [u8;6],
    pub normal: DifficultyQuestGroup,
    pub nightmare: DifficultyQuestGroup,
    pub hell: DifficultyQuestGroup,
}

impl QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<QuestStatus> {
        let mut identifier = [0u8;4];
        for b in &mut identifier {
            *b = reader.read_u8()?;
        }

        let mut unknown = [0u8;6];
        for b in &mut unknown {
            *b = reader.read_u8()?;
        }

        let normal = DifficultyQuestGroup::from(reader)?;
        let nightmare = DifficultyQuestGroup::from(reader)?;
        let hell = DifficultyQuestGroup::from(reader)?;

        Ok(QuestStatus {
            identifier,
            unknown,
            normal,
            nightmare,
            hell
        })
    }
}

impl Default for QuestStatus {
    fn default() -> Self {
        QuestStatus {
            identifier: ['W' as u8, 'o' as u8, 'o' as u8, '!' as u8],
            unknown: [0u8;6],
            normal: Default::default(),
            nightmare: Default::default(),
            hell: Default::default(),
        }
    }
}

impl Debug for QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "quest data")?;
        writeln!(f, "  normal")?;
        write!(f, "{:?}", self.normal)?;
        writeln!(f, "  nightmare")?;
        write!(f, "{:?}", self.nightmare)?;
        writeln!(f, "  hell")?;
        write!(f, "{:?}", self.hell)?;
        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct DifficultyQuestGroup {
    pub act_1: Act1QuestStatus,
    pub act_2: Act2QuestStatus,
    pub act_3: Act3QuestStatus,
    pub act_4: Act4QuestStatus,
    pub act_5: Act5QuestStatus,
    pub reserved: [u16;7],
}

impl DifficultyQuestGroup {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<DifficultyQuestGroup> {
        let act_1 = Act1QuestStatus::from(reader)?;
        let act_2 = Act2QuestStatus::from(reader)?;
        let act_3 = Act3QuestStatus::from(reader)?;
        let act_4 = Act4QuestStatus::from(reader)?;
        let act_5 = Act5QuestStatus::from(reader)?;

        let mut reserved = [0u16; 7];
        for b in &mut reserved {
            *b = reader.read_u16::<LittleEndian>()?;
        }

        Ok(DifficultyQuestGroup {
            act_1,
            act_2,
            act_3,
            act_4,
            act_5,
            reserved
        })
    }
}

impl Debug for DifficultyQuestGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.act_1)?;
        write!(f, "{:?}", self.act_2)?;
        write!(f, "{:?}", self.act_3)?;
        write!(f, "{:?}", self.act_4)?;
        write!(f, "{:?}", self.act_5)?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Act1QuestStatus {
    pub introduced: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
    pub travelled_act2: u16
}

impl Act1QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Act1QuestStatus> {
        let introduced = reader.read_u16::<LittleEndian>()?;
        let quest_1 = reader.read_u16::<LittleEndian>()?;
        let quest_2 = reader.read_u16::<LittleEndian>()?;
        let quest_3 = reader.read_u16::<LittleEndian>()?;
        let quest_4 = reader.read_u16::<LittleEndian>()?;
        let quest_5 = reader.read_u16::<LittleEndian>()?;
        let quest_6 = reader.read_u16::<LittleEndian>()?;
        let travelled_act2 = reader.read_u16::<LittleEndian>()?;

        Ok(Act1QuestStatus {
            introduced,
            quest_1,
            quest_2,
            quest_3,
            quest_4,
            quest_5,
            quest_6,
            travelled_act2
        })
    }
}

impl Debug for Act1QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    act_1")?;
        writeln!(f, "      q1")?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Act2QuestStatus {
    pub introduced: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
    pub travelled_act3: u16
}

impl Act2QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Act2QuestStatus> {
        let introduced = reader.read_u16::<LittleEndian>()?;
        let quest_1 = reader.read_u16::<LittleEndian>()?;
        let quest_2 = reader.read_u16::<LittleEndian>()?;
        let quest_3 = reader.read_u16::<LittleEndian>()?;
        let quest_4 = reader.read_u16::<LittleEndian>()?;
        let quest_5 = reader.read_u16::<LittleEndian>()?;
        let quest_6 = reader.read_u16::<LittleEndian>()?;
        let travelled_act3 = reader.read_u16::<LittleEndian>()?;

        Ok(Act2QuestStatus {
            introduced,
            quest_1,
            quest_2,
            quest_3,
            quest_4,
            quest_5,
            quest_6,
            travelled_act3
        })
    }
}

impl Debug for Act2QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    act_2")?;
        writeln!(f, "      q1")?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Act3QuestStatus {
    pub introduced: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
    pub travelled_act4: u16
}

impl Act3QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Act3QuestStatus> {
        let introduced = reader.read_u16::<LittleEndian>()?;
        let quest_1 = reader.read_u16::<LittleEndian>()?;
        let quest_2 = reader.read_u16::<LittleEndian>()?;
        let quest_3 = reader.read_u16::<LittleEndian>()?;
        let quest_4 = reader.read_u16::<LittleEndian>()?;
        let quest_5 = reader.read_u16::<LittleEndian>()?;
        let quest_6 = reader.read_u16::<LittleEndian>()?;
        let travelled_act4 = reader.read_u16::<LittleEndian>()?;

        Ok(Act3QuestStatus {
            introduced,
            quest_1,
            quest_2,
            quest_3,
            quest_4,
            quest_5,
            quest_6,
            travelled_act4
        })
    }
}

impl Debug for Act3QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    act_3")?;
        writeln!(f, "      q1")?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Act4QuestStatus {
    pub introduced: u16,
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub travelled_act5: u16
}

impl Act4QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Act4QuestStatus> {
        let introduced = reader.read_u16::<LittleEndian>()?;
        let quest_1 = reader.read_u16::<LittleEndian>()?;
        let quest_2 = reader.read_u16::<LittleEndian>()?;
        let quest_3 = reader.read_u16::<LittleEndian>()?;
        let travelled_act5 = reader.read_u16::<LittleEndian>()?;

        Ok(Act4QuestStatus {
            introduced,
            quest_1,
            quest_2,
            quest_3,
            travelled_act5
        })
    }
}

impl Debug for Act4QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    act_4")?;
        writeln!(f, "      q1")?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Act5QuestStatus {
    pub introduced: u16,
    pub unknown1: [u16;3],
    pub unknown2: u16,
    pub unknown3: [u16;2],
    pub quest_1: u16,
    pub quest_2: u16,
    pub quest_3: u16,
    pub quest_4: u16,
    pub quest_5: u16,
    pub quest_6: u16,
}

impl Act5QuestStatus {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Act5QuestStatus> {
        let introduced = reader.read_u16::<LittleEndian>()?;
        let mut unknown1 = [0u16;3];
        for b in &mut unknown1 {
            *b = reader.read_u16::<LittleEndian>()?;
        }
        let unknown2 = reader.read_u16::<LittleEndian>()?;
        let mut unknown3 = [0u16;2];
        for b in &mut unknown3 {
            *b = reader.read_u16::<LittleEndian>()?;
        }
        let quest_1 = reader.read_u16::<LittleEndian>()?;
        let quest_2 = reader.read_u16::<LittleEndian>()?;
        let quest_3 = reader.read_u16::<LittleEndian>()?;
        let quest_4 = reader.read_u16::<LittleEndian>()?;
        let quest_5 = reader.read_u16::<LittleEndian>()?;
        let quest_6 = reader.read_u16::<LittleEndian>()?;


        Ok(Act5QuestStatus {
            introduced,
            unknown1,
            unknown2,
            unknown3,
            quest_1,
            quest_2,
            quest_3,
            quest_4,
            quest_5,
            quest_6,
        })
    }
}

impl Debug for Act5QuestStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    act_5")?;
        writeln!(f, "      q1")?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct Waypoints {
    pub identifier: [u8;2],
    pub unknown: u8,
    pub normal: ActWaypoints,
    pub nightmare: ActWaypoints,
    pub hell: ActWaypoints,
}

impl Waypoints {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<Waypoints> {
        let mut identifier = [0u8;2];
        for b in &mut identifier {
            *b = reader.read_u8()?;
        }

        let unknown = reader.read_u8()?;
        let normal = ActWaypoints::from(reader)?;
        let nightmare = ActWaypoints::from(reader)?;
        let hell = ActWaypoints::from(reader)?;

        Ok(Waypoints {
            identifier,
            unknown,
            normal,
            nightmare,
            hell
        })
    }
}

impl Debug for Waypoints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "  normal")?;
        write!(f, "{:?}", self.normal)?;
        writeln!(f, "  nightmare")?;
        write!(f, "{:?}", self.nightmare)?;
        writeln!(f, "  hell")?;
        write!(f, "{:?}", self.hell)?;

        Ok(())
    }
}

#[derive(Clone,Default)]
pub struct ActWaypoints {
    pub unknown: [u8;2],
    pub waypoints: [u8;5],
    pub unknown2: [u8;17]
}

impl ActWaypoints {
    pub fn from(reader: &mut Cursor<&[u8]>) -> Result<ActWaypoints> {
        let mut unknown = [0u8;2];
        for b in &mut unknown {
            *b = reader.read_u8()?;
        }

        let mut waypoints = [0u8;5];
        for b in &mut waypoints {
            *b = reader.read_u8()?;
        }

        let mut unknown2 = [0u8;17];
        for b in &mut unknown2 {
            *b = reader.read_u8()?;
        }

        Ok(ActWaypoints {
            unknown,
            waypoints,
            unknown2
        })
    }
}

impl Debug for ActWaypoints {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "    waypoints here")?;

        Ok(())
    }
}

