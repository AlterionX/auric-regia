pub mod navy;
pub mod legion;
pub mod industry;

use std::borrow::Cow;
use tracing as trc;

use serenity::{all::{CommandInteraction, CommandOptionType, CommandType, ResolvedOption, ResolvedValue}, builder::{CreateCommand, CreateCommandOption}, model::Permissions};
use strum::{EnumCount, EnumIter};

use crate::discord::ExecutionContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum RequestKind {
    Ping,

    IndustryMiningRockRecord,

    // User
    NavyVictoryRecordOneUser,
    NavyVictoryCheckUser,
    // ChatInput
    NavyVictoryRecord,
    NavyVictoryDelete,
    NavyVictoryBoast,
    NavyVictoryCheck,
    NavyVictoryScoreboard,

    LegionKillRecord,
    LegionKillDelete,
    LegionKillBoast,
    LegionKillCheck,
    LegionKillScoreboard,

    IndustryProfitRecord,
    IndustryProfitDelete,
    IndustryProfitBoast,
    IndustryProfitCheck,
    IndustryProfitScoreboard,
}

impl RequestKind {
    pub fn name(&self) -> &'static str {
        match self {
            RequestKind::Ping => {
                "ping"
            },

            RequestKind::IndustryMiningRockRecord => {
                "record"
            },

            RequestKind::IndustryProfitRecord => {
                "record"
            },
            RequestKind::IndustryProfitDelete => {
                "delete"
            },
            RequestKind::IndustryProfitBoast => {
                "boast"
            },
            RequestKind::IndustryProfitCheck => {
                "check"
            },
            RequestKind::IndustryProfitScoreboard => {
                "scoreboard"
            },

            RequestKind::NavyVictoryRecordOneUser => {
                "Record One Naval Victory"
            },
            RequestKind::NavyVictoryCheckUser => {
                "Check Naval Victories"
            },

            RequestKind::NavyVictoryRecord => {
                "record"
            },
            RequestKind::NavyVictoryDelete => {
                "delete"
            },
            RequestKind::NavyVictoryBoast => {
                "boast"
            },
            RequestKind::NavyVictoryCheck => {
                "check"
            },
            RequestKind::NavyVictoryScoreboard => {
                "scoreboard"
            },

            RequestKind::LegionKillRecord => {
                "record"
            },
            RequestKind::LegionKillDelete => {
                "delete"
            },
            RequestKind::LegionKillBoast => {
                "boast"
            },
            RequestKind::LegionKillCheck => {
                "check"
            },
            RequestKind::LegionKillScoreboard => {
                "scoreboard"
            },
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RequestKind::Ping => {
                "Ping!"
            },

            RequestKind::IndustryMiningRockRecord => {
                "Records rocks"
            },

            RequestKind::IndustryProfitRecord => {
                "Record profits"
            },
            RequestKind::IndustryProfitDelete => {
                "Delete profits"
            },
            RequestKind::IndustryProfitBoast => {
                "Boast about your profits"
            },
            RequestKind::IndustryProfitCheck => {
                "Checks someone's (or your own) profits"
            },
            RequestKind::IndustryProfitScoreboard => {
                "Creates the scoreboard of profits across Auric."
            },

            RequestKind::NavyVictoryRecordOneUser => {
                "Record one victory for this user."
            },
            RequestKind::NavyVictoryCheckUser => {
                "Checks one victory for user."
            },

            RequestKind::NavyVictoryRecord => {
                "Records a certain number of naval victories for a user."
            },
            RequestKind::NavyVictoryDelete => {
                "Removes a certain number of naval victories for a user. Only goes down to 0!"
            },
            RequestKind::NavyVictoryBoast => {
                "Boast about your naval victories."
            },
            RequestKind::NavyVictoryCheck => {
                "Checks the number of naval victories for a specific user (or yourself)."
            },
            RequestKind::NavyVictoryScoreboard => {
                "Creates the scoreboard of naval victories across Auric."
            },

            RequestKind::LegionKillRecord => {
                "Records a certain number of kills for a user."
            },
            RequestKind::LegionKillDelete => {
                "Deletes a certain number of kills for a user."
            },
            RequestKind::LegionKillBoast => {
                "Boast about your legion kills."
            },
            RequestKind::LegionKillCheck => {
                "Checks the number of legion kills for a specific user (or yourself)."
            },
            RequestKind::LegionKillScoreboard => {
                "Creates the scoreboard of legion kills across Auric."
            },
        }
    }

    pub fn options(&self) -> Vec<RawCommandOptionEntry> {
        match self {
            RequestKind::Ping => {
                vec![]
            },

            RequestKind::IndustryMiningRockRecord => {
                vec![]
            },

            RequestKind::IndustryProfitRecord => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "aUEC",
                        description: "Number of alpha UEC. Defaults to 1000.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own profits."
                    },
                ]
            },
            RequestKind::IndustryProfitDelete => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "aUEC",
                        description: "Number of alpha UEC.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own profits.",
                    },
                ]
            },
            RequestKind::IndustryProfitBoast => {
                vec![]
            },
            RequestKind::IndustryProfitCheck => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person to get victories for. Defaults to self. Quieter than boasting.",
                    },
                ]
            },
            RequestKind::IndustryProfitScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        choices: vec![
                            ("Me", "me"),
                            ("Bottom", "bottom"),
                            ("Top (default)", "top"),
                            ("Someone", "someone"),
                            ("Rank", "rank"),
                        ],
                    },
                    RawCommandOptionEntry::User {
                        name: "someone",
                        description: "Should only be provided if \"at\" is set to \"someone\"."
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\""
                    },
                ]
            },

            RequestKind::NavyVictoryRecordOneUser => {
                vec![]
            },
            RequestKind::NavyVictoryCheckUser => {
                vec![]
            },

            RequestKind::NavyVictoryRecord => {
                vec![
                    RawCommandOptionEntry::Number {
                        name: "victories",
                        description: "Number of victories. Only accepts values in intervals of 0.25. Defaults to 1.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own victories."
                    },
                ]
            },
            RequestKind::NavyVictoryDelete => {
                vec![
                    RawCommandOptionEntry::Number {
                        name: "victories",
                        description: "Number of victories. Only accepts values in intervals of 0.25. Defaults to 1.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own victories.",
                    },
                ]
            },
            RequestKind::NavyVictoryBoast => {
                vec![]
            },
            RequestKind::NavyVictoryCheck => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person to get victories for. Defaults to self. Quieter than boasting.",
                    },
                ]
            },
            RequestKind::NavyVictoryScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        choices: vec![
                            ("Me", "me"),
                            ("Bottom", "bottom"),
                            ("Top (default)", "top"),
                            ("Someone", "someone"),
                            ("Rank", "rank"),
                        ],
                    },
                    RawCommandOptionEntry::User {
                        name: "someone",
                        description: "Should only be provided if \"at\" is set to \"someone\"."
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\""
                    },
                ]
            },

            RequestKind::LegionKillRecord => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "kills",
                        description: "Number of kills. Only accepts values in intervals of 0.25. Defaults to 1.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own kills."
                    },
                ]
            },
            RequestKind::LegionKillDelete => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "kills",
                        description: "Number of kills. Only accepts values in intervals of 0.25. Defaults to 1.",
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own kills.",
                    },
                ]
            },
            RequestKind::LegionKillBoast => {
                vec![]
            },
            RequestKind::LegionKillCheck => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person to get kills for. Defaults to self. Quieter than boasting.",
                    },
                ]
            },
            RequestKind::LegionKillScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        choices: vec![
                            ("Me", "me"),
                            ("Bottom", "bottom"),
                            ("Top (default)", "top"),
                            ("Someone", "someone"),
                            ("Rank", "rank"),
                        ],
                    },
                    RawCommandOptionEntry::User {
                        name: "someone",
                        description: "Should only be provided if \"at\" is set to \"someone\"."
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\""
                    },
                ]
            },
        }
    }
}

#[derive(Debug)]
pub enum RawCommandOptionEntry {
    Integer {
        name: &'static str,
        description: &'static str
    },
    Number {
        name: &'static str,
        description: &'static str
    },
    Boolean {
        name: &'static str,
        description: &'static str
    },
    String {
        name: &'static str,
        description: &'static str
    },
    User {
        name: &'static str,
        description: &'static str
    },
    StringSelect {
        name: &'static str,
        description: &'static str,
        // (name, value)
        choices: Vec<(&'static str, &'static str)>,
    },
}

impl RawCommandOptionEntry {
    fn kind(&self) -> CommandOptionType {
        match self {
            Self::Integer { .. } => CommandOptionType::Integer,
            Self::Number { .. } => CommandOptionType::Number,
            Self::Boolean { .. } => CommandOptionType::Boolean,
            Self::String { .. } => CommandOptionType::String,
            Self::User { .. } => CommandOptionType::User,
            Self::StringSelect { .. } => CommandOptionType::String,
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Self::Integer { name, .. } => name,
            Self::Number { name, .. } => name,
            Self::Boolean { name, .. } => name,
            Self::String { name, .. } => name,
            Self::User { name, .. } => name,
            Self::StringSelect { name, .. } => name,
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Self::Integer { description, .. } => description,
            Self::Number { description, .. } => description,
            Self::Boolean { description, .. } => description,
            Self::String { description, .. } => description,
            Self::User { description, .. } => description,
            Self::StringSelect { description, .. } => description,
        }
    }

    fn to_option(&self) -> CreateCommandOption {
        let mut builder = CreateCommandOption::new(self.kind(), self.name(), self.description());
        match self {
            Self::Integer { .. } => {},
            Self::Number { .. } => {},
            Self::Boolean { .. } => {},
            Self::String { .. } => {},
            Self::User { .. } => {},
            Self::StringSelect { choices, .. } => {
                for (name, value) in choices {
                    builder = builder.add_string_choice(*name, *value);
                }
            },
        }
        builder
    }
}

pub struct CommandTreeIntermediate {
    pub name: &'static str,
    pub description: &'static str,
    pub children: Vec<RequestKind>,
}

pub enum CommandTreeTop {
    Secondary {
        name: &'static str,
        description: &'static str,
        kind: CommandType,
        children: Vec<CommandTreeIntermediate>,
        opt_default_perm: Option<Permissions>,
    },
    Primary {
        name: &'static str,
        description: &'static str,
        kind: CommandType,
        children: Vec<RequestKind>,
        opt_default_perm: Option<Permissions>,
    },
    NakedChatInput(RequestKind, Option<Permissions>),
    NakedUser(RequestKind, Option<Permissions>),
}

impl CommandTreeTop {
    pub fn into_discord_command(self) -> CreateCommand {
        match self {
            Self::Secondary { name, description, kind, children, opt_default_perm } => {
                let mut top_level = CreateCommand::new(name).description(description).kind(kind);
                if let Some(perm) = opt_default_perm {
                    top_level = top_level.default_member_permissions(perm);
                }

                let subcommand_groups: Vec<_> = children.into_iter().map(|cti| {
                    let mut subcommand_group = CreateCommandOption::new(CommandOptionType::SubCommandGroup, cti.name, cti.description);
                    for child in cti.children {
                        let mut subcommand = CreateCommandOption::new(CommandOptionType::SubCommand, child.name(), child.description());
                        let options = child.options();
                        for option in options {
                            subcommand = subcommand.add_sub_option(option.to_option());
                        }
                        subcommand_group = subcommand_group.add_sub_option(subcommand);
                    }

                    subcommand_group
                }).collect();
                if !subcommand_groups.is_empty() {
                    top_level = top_level.set_options(subcommand_groups);
                }

                top_level
            },
            Self::Primary { name, description, kind, children, opt_default_perm } => {
                let mut builder = CreateCommand::new(name).description(description).kind(kind);
                if let Some(perm) = opt_default_perm {
                    builder = builder.default_member_permissions(perm);
                }
                for child in children {
                    let mut subcommand = CreateCommandOption::new(CommandOptionType::SubCommand, child.name(), child.description());
                    let options = child.options();
                    for option in options {
                        subcommand = subcommand.add_sub_option(option.to_option());
                    }
                    builder = builder.add_option(subcommand);
                }

                builder
            },
            Self::NakedChatInput(cmd, opt_default_perm) => {
                let mut builder = CreateCommand::new(cmd.name()).description(cmd.description()).kind(CommandType::ChatInput);
                if let Some(perm) = opt_default_perm {
                    builder = builder.default_member_permissions(perm);
                }
                let options = cmd.options();
                if !options.is_empty() {
                    builder = builder.set_options(options.into_iter().map(|rcoe| {
                        rcoe.to_option()
                    }).collect());
                }
                builder
            },
            Self::NakedUser(cmd, opt_default_perm) => {
                let mut builder = CreateCommand::new(cmd.name()).kind(CommandType::User);
                if let Some(perm) = opt_default_perm {
                    builder = builder.default_member_permissions(perm);
                }
                let options = cmd.options();
                if !options.is_empty() {
                    builder = builder.set_options(options.into_iter().map(|rcoe| {
                        rcoe.to_option()
                    }).collect());
                }
                builder
            },
        }
    }
}

#[tracing::instrument(name = "hello")]
pub fn generate_command_descriptions() -> Vec<CommandTreeTop> {
    vec![
        CommandTreeTop::NakedChatInput(RequestKind::Ping, None),
        CommandTreeTop::Secondary {
            name: "industry",
            description: "Industry commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            children: vec![
                CommandTreeIntermediate {
                    name: "mining",
                    description: "Commands for mining data stashing",
                    children: vec![
                        RequestKind::IndustryMiningRockRecord,
                    ],
                },
                CommandTreeIntermediate {
                    name: "profit",
                    description: "Commands for managing profit records",
                    children: vec![
                        RequestKind::IndustryProfitRecord,
                        RequestKind::IndustryProfitDelete,
                        RequestKind::IndustryProfitBoast,
                        RequestKind::IndustryProfitCheck,
                        RequestKind::IndustryProfitScoreboard,
                    ],
                },
            ],
        },

        CommandTreeTop::NakedUser(RequestKind::NavyVictoryRecordOneUser, None),
        CommandTreeTop::NakedUser(RequestKind::NavyVictoryCheckUser, None),
        CommandTreeTop::Secondary {
            name: "navy",
            description: "Navy commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            children: vec![
                CommandTreeIntermediate {
                    name: "victory",
                    description: "Commands for managing victory counts",
                    children: vec![
                        RequestKind::NavyVictoryRecord,
                        RequestKind::NavyVictoryDelete,
                        RequestKind::NavyVictoryBoast,
                        RequestKind::NavyVictoryCheck,
                        RequestKind::NavyVictoryScoreboard,
                    ],
                },
            ],
        },
        CommandTreeTop::Secondary {
            name: "legion",
            description: "Legion commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            children: vec![
                CommandTreeIntermediate {
                    name: "kill",
                    description: "Commands for managing kill counts",
                    children: vec![
                        RequestKind::LegionKillRecord,
                        RequestKind::LegionKillDelete,
                        RequestKind::LegionKillBoast,
                        RequestKind::LegionKillCheck,
                        RequestKind::LegionKillScoreboard,
                    ],
                },
            ],
        },
    ]
}

#[derive(Debug)]
pub enum RequestArgs<'a> {
    Ping,

    IndustryMiningRockRecord,

    NavyVictoryRecord(navy::victory::record::Request),
    NavyVictoryDelete(navy::victory::delete::Request),
    NavyVictoryBoast(navy::victory::boast::Request),
    NavyVictoryCheck(navy::victory::check::Request),
    NavyVictoryScoreboard(navy::victory::scoreboard::Request<'a>),

    LegionKillRecord(legion::kill::record::Request),
    LegionKillDelete(legion::kill::delete::Request),
    LegionKillBoast(legion::kill::boast::Request),
    LegionKillCheck(legion::kill::check::Request),
    LegionKillScoreboard(legion::kill::scoreboard::Request<'a>),
}

impl <'a> RequestArgs<'a> {
    pub fn parse(cmd: &'a CommandInteraction) -> Result<Self, RequestError> {
        match cmd.data.name.as_str() {
            "ping" => {
                Ok(RequestArgs::Ping)
            },
            "industry" => {
                let tier0_options = cmd.data.options();
                let Some(tier1) = tier0_options.get(0) else {
                    return Err(RequestError::Internal("Missing options for `industry`.".into()));
                };
                match tier1.name {
                    "mining" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `industry`.".into()));
                        };
                        let Some(tier2) = tier1_options.get(0) else {
                            return Err(RequestError::Internal("Missing options for `industry mining`.".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `industry mining`.".into()));
                        };
                        if tier2.name != "record" {
                            return Err(RequestError::Internal("Unknown subcommand for `industry mining`.".into()));
                        }
                        if !tier2_options.is_empty() {
                            return Err(RequestError::Internal("Extra options for `industry mining`".into()));
                        }
                        Ok(RequestArgs::IndustryMiningRockRecord)
                    },
                    _ => {
                        return Err(RequestError::Internal("Bad subcommand for `industry`.".into()));
                    },
                }
            },
            "Record One Naval Victory" => {
                return Ok(RequestArgs::NavyVictoryRecord(navy::victory::record::Request::parse(cmd, &[])?));
            },
            "Check Naval Victories" => {
                Ok(Self::NavyVictoryCheck(navy::victory::check::Request::parse(cmd, &[])?))
            },
            "navy" => {
                let tier0_options: Vec<ResolvedOption<'a>> = cmd.data.options();
                let Some(tier1) = tier0_options.get(0) else {
                    return Err(RequestError::Internal("Missing options for `navy`.".into()));
                };
                match tier1.name {
                    "victory" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `navy`".into()));
                        };
                        let Some(tier2) = tier1_options.get(0) else {
                            return Err(RequestError::Internal("Missing options for `navy victory`".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `navy victory`".into()));
                        };
                        match tier2.name {
                            "record" => {
                                return Ok(RequestArgs::NavyVictoryRecord(navy::victory::record::Request::parse(cmd, tier2_options.as_slice())?));
                            },
                            "delete" => {
                                return Ok(RequestArgs::NavyVictoryDelete(navy::victory::delete::Request::parse(cmd, tier2_options.as_slice())?));
                            },
                            "boast" => {
                                Ok(RequestArgs::NavyVictoryBoast(navy::victory::boast::Request::parse(cmd, &[])?))
                            },
                            "check" => {
                                Ok(RequestArgs::NavyVictoryCheck(navy::victory::check::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "scoreboard" => {
                                Ok(RequestArgs::NavyVictoryScoreboard(navy::victory::scoreboard::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                return Err(RequestError::Internal("Unknown subcommand for `navy victory`".into()));
                            },
                        }
                    },
                    _ => {
                        trc::warn!("Unknown subcommand {:?}", tier1);
                        return Err(RequestError::Internal("Unknown subcommand for `navy`".into()));
                    },
                }
            },
            "legion" => {
                let tier0_options: Vec<ResolvedOption<'a>> = cmd.data.options();
                let Some(tier1) = tier0_options.get(0) else {
                    return Err(RequestError::Internal("Missing options for `legion`.".into()));
                };
                match tier1.name {
                    "kill" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `legion`".into()));
                        };
                        let Some(tier2) = tier1_options.get(0) else {
                            return Err(RequestError::Internal("Missing options for `legion kill`".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `legion kill`".into()));
                        };
                        match tier2.name {
                            "record" => {
                                return Ok(RequestArgs::LegionKillRecord(legion::kill::record::Request::parse(cmd, tier2_options.as_slice())?));
                            },
                            "delete" => {
                                return Ok(RequestArgs::LegionKillDelete(legion::kill::delete::Request::parse(cmd, tier2_options.as_slice())?));
                            },
                            "boast" => {
                                Ok(RequestArgs::LegionKillBoast(legion::kill::boast::Request::parse(cmd, &[])?))
                            },
                            "check" => {
                                Ok(RequestArgs::LegionKillCheck(legion::kill::check::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "scoreboard" => {
                                Ok(RequestArgs::LegionKillScoreboard(legion::kill::scoreboard::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                return Err(RequestError::Internal("Unknown subcommand for `legion kill`".into()));
                            },
                        }
                    },
                    _ => {
                        trc::warn!("Unknown subcommand {:?}", tier1);
                        return Err(RequestError::Internal("Unknown subcommand for `legion`".into()));
                    },
                }
            },
            _ => {
                trc::error!("Unknown command {:?} received", cmd);
                return Err(RequestError::Internal("Unknown command.".into()));
            },
        }
    }
}

#[derive(Debug)]
pub struct Request<'a> {
    pub args: RequestArgs<'a>,
}

#[derive(Debug)]
pub enum RequestError {
    User(Cow<'static, str>),
    Internal(Cow<'static, str>),
}

impl RequestError {
    pub async fn report(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        match self {
            Self::User(reason) => {
                trc::warn!("REQ-ERR-USER reason={}", reason);
                ctx.reply_restricted(reason.to_string()).await
            },
            Self::Internal(reason) => {
                trc::error!("REQ-ERR-INTERNAL reason={}", reason);
                ctx.reply_restricted("Something broke! Please contact a mod for help.".to_owned()).await
            },
        }
    }
}

impl <'a> Request<'a> {
    pub fn parse(cmd: &'a CommandInteraction) -> Result<Self, RequestError> {
        Ok(Request {
            args: RequestArgs::parse(&cmd)?,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        match self.args {
            RequestArgs::Ping => {
                // Just try pong.
                ctx.reply("Pong!".to_owned()).await
            },

            RequestArgs::IndustryMiningRockRecord => {
                ctx.reply("Industry mining number crunching not yet implemented.".to_owned()).await
            },

            RequestArgs::NavyVictoryRecord(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyVictoryDelete(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyVictoryBoast(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyVictoryCheck(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyVictoryScoreboard(req) => {
                req.execute(ctx).await
            },

            RequestArgs::LegionKillRecord(req) => {
                req.execute(ctx).await
            },
            RequestArgs::LegionKillDelete(req) => {
                req.execute(ctx).await
            },
            RequestArgs::LegionKillBoast(req) => {
                req.execute(ctx).await
            },
            RequestArgs::LegionKillCheck(req) => {
                req.execute(ctx).await
            },
            RequestArgs::LegionKillScoreboard(req) => {
                req.execute(ctx).await
            },
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use strum::{EnumCount, IntoEnumIterator};

    use super::{generate_command_descriptions, RequestKind, CommandTreeIntermediate, CommandTreeTop};

    fn iter_tree_intermediate(inter: &CommandTreeIntermediate, set: &mut HashSet<RequestKind>) {
        for c in inter.children.iter() {
            assert!(!set.contains(c));
            set.insert(*c);
        }
    }

    fn iter_tree(tree: &CommandTreeTop, set: &mut HashSet<RequestKind>) {
        match tree {
            CommandTreeTop::Secondary { ref children, .. } => {
                for c in children {
                    iter_tree_intermediate(c, set);
                }
            },
            CommandTreeTop::Primary { ref children, .. } => {
                for c in children {
                    assert!(!set.contains(c));
                    set.insert(*c);
                }
            },
            CommandTreeTop::NakedUser(ref cmd, _) | CommandTreeTop::NakedChatInput(ref cmd, _) => {
                assert!(!set.contains(cmd));
                set.insert(*cmd);
            },
        }
    }

    #[test]
    fn all_commands_accounted_for() {
        let mut found_commands = HashSet::new();
        let command_tree = generate_command_descriptions();
        command_tree.iter().for_each(|ctt| iter_tree(ctt, &mut found_commands));
        assert_eq!(found_commands.len(), RequestKind::COUNT);
    }

    #[test]
    fn description_not_too_long() {
        for c in RequestKind::iter() {
            assert!(c.description().len() < 100, "{c:?}");
            for o in c.options() {
                assert!(o.description().len() < 100, "{c:?}, {o:?}")
            }
        }
    }
}
