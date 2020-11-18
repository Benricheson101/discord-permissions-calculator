use FLAGS::*;
use std::{
    convert::From,
    slice::Iter,
    collections::HashMap,
    io::{Write, stdin, stdout},
    fmt,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FLAGS {
    CREATE_INSTANT_INVITE = 0x00000001,
    KICK_MEMBERS = 0x00000002,
    BAN_MEMBERS = 0x00000004,
    ADMINISTRATOR = 0x00000008,
    MANAGE_CHANNELS = 0x00000010,
    MANAGE_GUILD = 0x00000020,
    ADD_REACTIONS = 0x00000040,
    VIEW_AUDIT_LOG = 0x00000080,
    PRIORITY_SPEAKER = 0x00000100,
    STREAM = 0x00000200,
    VIEW_CHANNEL = 0x00000400,
    SEND_MESSAGES = 0x00000800,
    SEND_TTS_MESSAGES = 0x00001000,
    MANAGE_MESSAGES = 0x00002000,
    EMBED_LINKS = 0x00004000,
    ATTACH_FILES = 0x00008000,
    READ_MESSAGE_HISTORY = 0x00010000,
    MENTION_EVERYONE = 0x00020000,
    USE_EXTERNAL_EMOJIS = 0x00040000,
    VIEW_GUILD_INSIGHTS = 0x00080000,
    CONNECT = 0x00100000,
    SPEAK = 0x00200000,
    MUTE_MEMBERS = 0x00400000,
    DEAFEN_MEMBERS = 0x00800000,
    MOVE_MEMBERS = 0x01000000,
    USE_VAD = 0x02000000,
    CHANGE_NICKNAME = 0x04000000,
    MANAGE_NICKNAMES = 0x08000000,
    MANAGE_ROLES = 0x10000000,
    MANAGE_WEBHOOKS = 0x20000000,
    MANAGE_EMOJIS = 0x40000000,
}

impl FLAGS {
    pub fn iterator() -> Iter<'static, Self> {
        static FS: [FLAGS; 31] = [
            FLAGS::CREATE_INSTANT_INVITE,
            FLAGS::KICK_MEMBERS,
            FLAGS::BAN_MEMBERS,
            FLAGS::ADMINISTRATOR,
            FLAGS::MANAGE_CHANNELS,
            FLAGS::MANAGE_GUILD,
            FLAGS::ADD_REACTIONS,
            FLAGS::VIEW_AUDIT_LOG,
            FLAGS::PRIORITY_SPEAKER,
            FLAGS::STREAM,
            FLAGS::VIEW_CHANNEL,
            FLAGS::SEND_MESSAGES,
            FLAGS::SEND_TTS_MESSAGES,
            FLAGS::MANAGE_MESSAGES,
            FLAGS::EMBED_LINKS,
            FLAGS::ATTACH_FILES,
            FLAGS::READ_MESSAGE_HISTORY,
            FLAGS::MENTION_EVERYONE,
            FLAGS::USE_EXTERNAL_EMOJIS,
            FLAGS::VIEW_GUILD_INSIGHTS,
            FLAGS::CONNECT,
            FLAGS::SPEAK,
            FLAGS::MUTE_MEMBERS,
            FLAGS::DEAFEN_MEMBERS,
            FLAGS::MOVE_MEMBERS,
            FLAGS::USE_VAD,
            FLAGS::CHANGE_NICKNAME,
            FLAGS::MANAGE_NICKNAMES,
            FLAGS::MANAGE_ROLES,
            FLAGS::MANAGE_WEBHOOKS,
            FLAGS::MANAGE_EMOJIS,
        ];

        FS.iter()
    }
}

pub struct Perms(pub usize);

impl From<&Vec<FLAGS>> for Perms {
    fn from(flags: &Vec<FLAGS>) -> Self {
        let bits = flags
            .iter()
            .fold(0, |t, c| t | *c as usize);

        Self(bits)
    }
}

impl Perms {
    pub fn new(bits: usize) -> Self {
        Self(bits)
    }

    pub fn from_input() -> Self {
        let mut input = String::new();
        let mut flags = Vec::new();

        println!("Type the permissions you would like to calculate, comma separated.");
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();

        let split = input.as_str()
            .split(',')
            .map(|x| x.trim().to_ascii_uppercase().replace(" ", "_"))
            .filter(|s| !s.is_empty())
            .collect::<Vec<String>>();

        for flag in &split {
            match flag.as_str() {
                "CREATE_INSTANT_INVITE" | "CREATE_INVITE" => flags.push(CREATE_INSTANT_INVITE),
                "KICK" | "KICK_MEMBERS" => flags.push(KICK_MEMBERS),
                "BAN" | "BAN_MEMBERS" => flags.push(BAN_MEMBERS),
                "ADMIN" | "ADMINISTRATOR" => flags.push(ADMINISTRATOR),
                "MANAGE_CHANNELS" => flags.push(MANAGE_CHANNELS),
                "MANAGE_SERVER" | "MANAGE_GUILD" => flags.push(MANAGE_GUILD),
                "REACT" | "ADD_REACTIONS" => flags.push(ADD_REACTIONS),
                "AUDIT_LOG" | "VIEW_AUDIT_LOG" => flags.push(VIEW_AUDIT_LOG),
                "PRIORITY_SPEAKER" => flags.push(PRIORITY_SPEAKER),
                "STREAM" | "GOLIVE" | "GO_LIVE" => flags.push(STREAM),
                "READ" | "READ_MESSAGES" | "VIEW" | "VIEW_CHANNEL" => flags.push(VIEW_CHANNEL),
                "SEND" | "SEND_MESSAGES" => flags.push(SEND_MESSAGES),
                "TTS" | "SEND_TTS" | "SEND_TTS_MESSAGES" => flags.push(SEND_TTS_MESSAGES),
                "MANAGE_MESSAGES" => flags.push(MANAGE_MESSAGES),
                "EMBEDS" | "EMBED" | "EMBED_LINKS" => flags.push(EMBED_LINKS),
                "FILES" | "ATTACH" | "ATTACH_FILES" => flags.push(ATTACH_FILES),
                "MESSAGE_HISTORY" | "READ_MESSAGE_HISTORY" => flags.push(READ_MESSAGE_HISTORY),
                "PING_EVERYONE" | "MENTION_EVERYONE" => flags.push(MENTION_EVERYONE),
                "EXTERNAL_EMOJIS" | "EXTERNAL_EMOTES" | "USE_EXTERNAL_EMOJIS" | "USE_EXTERNAL_EMOTES" => flags.push(USE_EXTERNAL_EMOJIS),
                "INSIGHTS" | "VIEW_GUILD_INSIGHTS" => flags.push(VIEW_GUILD_INSIGHTS),
                "CONNECT" => flags.push(CONNECT),
                "SPEAK" => flags.push(SPEAK),
                "MUTE" | "MUTE_MEMBERS" => flags.push(MUTE_MEMBERS),
                "DEAFEN" | "DEAFEN_MEMBERS" => flags.push(DEAFEN_MEMBERS),
                "MOVE" | "MOVE_MEMBERS" => flags.push(MOVE_MEMBERS),
                "VOICE_ACTIVITY" | "USE_VOICE_ACTIVITY" | "USE_VAD" => flags.push(USE_VAD),
                "NICK" | "NICKNAME" | "CHANGE_NICKNAME" | "CHANGE_NICK" => flags.push(CHANGE_NICKNAME),
                "MANAGE_NICKNAMES" | "MANAGE_NICKS" => flags.push(MANAGE_NICKNAMES),
                "MANAGE_ROLES" => flags.push(MANAGE_ROLES),
                "WEBHOOKS" | "MANAGE_WEBHOOKS" => flags.push(MANAGE_WEBHOOKS),
                "MANAGE_EMOJIS" => flags.push(MANAGE_EMOJIS),
                &_ => (),
            }
        }

        Self::from(&flags)
    }

    pub fn add(&mut self, flag: &FLAGS) {
        self.0 |= *flag as usize;
    }

    pub fn remove(&mut self, flag: &FLAGS) {
        self.0 &= !(*flag as usize);
    }

    pub fn has(&self, flag: &FLAGS) -> bool {
        (self.0 & *flag as usize) == *flag as usize
    }

    pub fn tokens(&self) -> Vec<FLAGS> {
        let mut tokens = Vec::new();

        for flag in FLAGS::iterator() {
            if self.has(flag) {
                tokens.push(*flag)
            }
        }
        tokens
    }

    pub fn serialize(&self) -> HashMap<FLAGS, bool> {
        let mut map = HashMap::new();

        for flag in FLAGS::iterator() {
            map.insert(*flag, self.has(flag));
        }
        map
    }
}

impl fmt::Display for Perms {
    fn fmt(&self, f: & mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
