pub mod navy;
pub mod legion;
pub mod industry;
pub mod event;

use std::borrow::Cow;
use tracing as trc;

use serenity::{all::{CommandInteraction, CommandOptionType, CommandType, ResolvedOption, ResolvedValue}, builder::{CreateCommand, CreateCommandOption}, model::Permissions};
use strum::{EnumCount, EnumIter};

use crate::discord::ExecutionContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum RequestKind {
    Ping,

    EventParticipantRecord,
    EventParticipantRemove,
    EventParticipantCheck,

    IndustryMiningRockRecord,

    IndustryProfitRecord,
    IndustryProfitDelete,
    IndustryProfitBoast,
    IndustryProfitCheck,
    IndustryProfitScoreboard,
    IndustryProfitClearUnknown,

    // User
    NavyVictoryRecordOneUser,
    NavyVictoryCheckUser,
    // ChatInput
    NavyVictoryRecord,
    NavyVictoryDelete,
    NavyVictoryBoast,
    NavyVictoryCheck,
    NavyVictoryScoreboard,
    NavyVictoryClearUnknown,

    NavyTackleAssistRecord,
    NavyTackleAssistDelete,
    NavyTackleAssistBoast,
    NavyTackleAssistCheck,
    NavyTackleAssistScoreboard,
    NavyTackleAssistClearUnknown,

    LegionKillRecord,
    LegionKillDelete,
    LegionKillBoast,
    LegionKillCheck,
    LegionKillScoreboard,
    LegionKillClearUnknown,

    MonthlyGoalCheck,
    MonthlyGoalSet,
}

impl RequestKind {
    pub fn name(&self) -> &'static str {
        match self {
            RequestKind::Ping => {
                "ping"
            },

            RequestKind::EventParticipantRecord => {
                "record"
            },
            RequestKind::EventParticipantRemove => {
                "remove"
            },
            RequestKind::EventParticipantCheck => {
                "check"
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
            RequestKind::IndustryProfitClearUnknown => {
                "clear_unknown"
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
            RequestKind::NavyVictoryClearUnknown => {
                "clear_unknown"
            },

            RequestKind::NavyTackleAssistRecord => {
                "record"
            },
            RequestKind::NavyTackleAssistDelete => {
                "delete"
            },
            RequestKind::NavyTackleAssistBoast => {
                "boast"
            },
            RequestKind::NavyTackleAssistCheck => {
                "check"
            },
            RequestKind::NavyTackleAssistScoreboard => {
                "scoreboard"
            },
            RequestKind::NavyTackleAssistClearUnknown => {
                "clear_unknown"
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
            RequestKind::LegionKillClearUnknown => {
                "clear_unknown"
            },

            RequestKind::MonthlyGoalCheck => {
                "check"
            },
            RequestKind::MonthlyGoalSet => {
                "set"
            },
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            RequestKind::Ping => {
                "Ping!"
            },

            RequestKind::EventParticipantRecord => {
                "Record a participant for an event"
            },
            RequestKind::EventParticipantRemove => {
                "Remove a participant from an event"
            },
            RequestKind::EventParticipantCheck => {
                "Check how many events a participant has been part of"
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
            RequestKind::IndustryProfitClearUnknown => {
                "Removes old unknown users from the scoreboard"
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
            RequestKind::NavyVictoryClearUnknown => {
                "Removes old unknown users from the scoreboard"
            },

            RequestKind::NavyTackleAssistRecord => {
                "Records a certain number of naval tackle assists for a user."
            },
            RequestKind::NavyTackleAssistDelete => {
                "Removes a certain number of naval tackle assists for a user. Only goes down to 0!"
            },
            RequestKind::NavyTackleAssistBoast => {
                "Boast about your naval tackle assists."
            },
            RequestKind::NavyTackleAssistCheck => {
                "Checks the number of naval tackle assists for a specific user (or yourself)."
            },
            RequestKind::NavyTackleAssistScoreboard => {
                "Creates the scoreboard of naval tackle assists across Auric."
            },
            RequestKind::NavyTackleAssistClearUnknown => {
                "Removes old unknown users from the scoreboard"
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
            RequestKind::LegionKillClearUnknown => {
                "Removes old unknown users from the scoreboard"
            },

            RequestKind::MonthlyGoalCheck => {
                "Check the monthly goal for the org or a branch"
            },
            RequestKind::MonthlyGoalSet => {
                "Check the monthly goal for the org or a branch"
            },
        }
    }

    pub fn options(&self) -> Vec<RawCommandOptionEntry> {
        match self {
            RequestKind::Ping => {
                vec![]
            },

            RequestKind::EventParticipantRecord => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own participation.",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "count",
                        description: "Number of events being recorded, defaults to 1",
                        required: false,
                    },
                    RawCommandOptionEntry::String {
                        name: "note",
                        description: "Notes. This is not accessible via commands.",
                        required: false,
                    },
                ]
            },
            RequestKind::EventParticipantRemove => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own participation.",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "count",
                        description: "Number of events being recorded, defaults to 1",
                        required: false,
                    },
                    RawCommandOptionEntry::String {
                        name: "note",
                        description: "notes",
                        required: false,
                    },
                ]
            },
            RequestKind::EventParticipantCheck => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being checked. Leaving this out means that you're checking your own participation.",
                        required: false,
                    },
                ]
            },

            RequestKind::IndustryMiningRockRecord => {
                vec![]
            },

            RequestKind::IndustryProfitRecord => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "auec",
                        description: "Number of alpha UEC. Defaults to 1000.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own profits.",
                        required: false,
                    },
                ]
            },
            RequestKind::IndustryProfitDelete => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "auec",
                        description: "Number of alpha UEC.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own profits.",
                        required: false,
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
                        required: false,
                    },
                ]
            },
            RequestKind::IndustryProfitScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                        required: false,
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        required: false,
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
                        description: "Should only be provided if \"at\" is set to \"someone\".",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\"",
                        required: false,
                    },
                ]
            },
            RequestKind::IndustryProfitClearUnknown => {
                vec![]
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
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own victories.",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyVictoryDelete => {
                vec![
                    RawCommandOptionEntry::Number {
                        name: "victories",
                        description: "Number of victories. Only accepts values in intervals of 0.25. Defaults to 1.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own victories.",
                        required: false,
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
                        required: false,
                    },
                ]
            },
            RequestKind::NavyVictoryScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                        required: false,
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        required: false,
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
                        description: "Should only be provided if \"at\" is set to \"someone\".",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\"",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyVictoryClearUnknown => {
                vec![]
            },

            RequestKind::NavyTackleAssistRecord => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "tackle_assists",
                        description: "Number of tackle assists. Defaults to 1.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own tackle assists.",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyTackleAssistDelete => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "tackle_assists",
                        description: "Number of tackle assists. Defaults to 1.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own tackle assists.",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyTackleAssistBoast => {
                vec![]
            },
            RequestKind::NavyTackleAssistCheck => {
                vec![
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person to get tackle assists for. Defaults to self. Quieter than boasting.",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyTackleAssistScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                        required: false,
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        required: false,
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
                        description: "Should only be provided if \"at\" is set to \"someone\".",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\"",
                        required: false,
                    },
                ]
            },
            RequestKind::NavyTackleAssistClearUnknown => {
                vec![]
            },

            RequestKind::LegionKillRecord => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "kills",
                        description: "Number of kills. Only accepts values in intervals of 0.25. Defaults to 1.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own kills.",
                        required: false,
                    },
                ]
            },
            RequestKind::LegionKillDelete => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "kills",
                        description: "Number of kills. Only accepts values in intervals of 0.25. Defaults to 1.",
                        required: false,
                    },
                    RawCommandOptionEntry::User {
                        name: "user",
                        description: "Person being recorded for. Leaving this out means that you're recording your own kills.",
                        required: false,
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
                        required: false,
                    },
                ]
            },
            RequestKind::LegionKillScoreboard => {
                vec![
                    RawCommandOptionEntry::Integer {
                        name: "limit",
                        description: "Maximum entries to return. Max of 20. Defaults to 10.",
                        required: false,
                    },
                    RawCommandOptionEntry::StringSelect {
                        name: "at",
                        description: "What to orient the scoreboard on.",
                        required: false,
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
                        description: "Should only be provided if \"at\" is set to \"someone\".",
                        required: false,
                    },
                    RawCommandOptionEntry::Integer {
                        name: "rank",
                        description: "The integer rank to start the scoreboard at. Mutually exclusive with \"someone\"",
                        required: false,
                    },
                ]
            },
            RequestKind::LegionKillClearUnknown => {
                vec![]
            },

            RequestKind::MonthlyGoalCheck => {
                vec![
                    RawCommandOptionEntry::StringSelect {
                        name: "branch",
                        description: "Which branch to set the goal for, or the org",
                        required: false,
                        choices: vec![
                            ("Main (default)", "main"),
                            ("Navy", "navy"),
                            ("Legion", "legion"),
                            ("Industry", "industry"),
                        ],
                    }
                ]
            },
            RequestKind::MonthlyGoalSet => {
                vec![
                    RawCommandOptionEntry::StringSelect {
                        name: "branch",
                        description: "Which branch to set the goal for, or the org",
                        required: false,
                        choices: vec![
                            ("Main (default)", "main"),
                            ("Navy", "navy"),
                            ("Legion", "legion"),
                            ("Industry", "industry"),
                        ],
                    },
                    RawCommandOptionEntry::String {
                        name: "header",
                        description: "Header of the message, as of 2025/12/16 max 100 chars",
                        required: true,
                    },
                    RawCommandOptionEntry::String {
                        name: "body",
                        description: "Body of the message, as of 2025/12/16 max 5000 chars",
                        required: true,
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
        description: &'static str,
        required: bool,
    },
    Number {
        name: &'static str,
        description: &'static str,
        required: bool,
    },
    Boolean {
        name: &'static str,
        description: &'static str,
        required: bool,
    },
    String {
        name: &'static str,
        description: &'static str,
        required: bool,
    },
    User {
        name: &'static str,
        description: &'static str,
        required: bool,
    },
    StringSelect {
        name: &'static str,
        description: &'static str,
        // (name, value)
        choices: Vec<(&'static str, &'static str)>,
        required: bool,
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

    fn required(&self) -> bool {
        *match self {
            Self::Integer { required, .. } => required,
            Self::Number { required, .. } => required,
            Self::Boolean { required, .. } => required,
            Self::String { required, .. } => required,
            Self::User { required, .. } => required,
            Self::StringSelect { required, .. } => required,
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
        builder = builder.required(self.required());
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
    Complex {
        name: &'static str,
        description: &'static str,
        kind: CommandType,
        subcommand_groups: Vec<CommandTreeIntermediate>,
        subcommands: Vec<RequestKind>,
        opt_default_perm: Option<Permissions>,
    },
    NakedChatInput(RequestKind, Option<Permissions>),
    NakedUser(RequestKind, Option<Permissions>),
}

impl CommandTreeTop {
    pub fn into_discord_command(self) -> CreateCommand {
        match self {
            Self::Complex { name, description, kind, subcommands, subcommand_groups, opt_default_perm } => {
                let mut top_level = CreateCommand::new(name).description(description).kind(kind);
                if let Some(perm) = opt_default_perm {
                    top_level = top_level.default_member_permissions(perm);
                }

                let subcommand_iter = subcommands.into_iter().map(|rk| {
                    let mut subcommand = CreateCommandOption::new(CommandOptionType::SubCommand, rk.name(), rk.description());
                    let options = rk.options();
                    for option in options {
                        subcommand = subcommand.add_sub_option(option.to_option());
                    }
                    subcommand
                });
                let subcommand_group_iter = subcommand_groups.into_iter().map(|cti| {
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
                });

                let child_members: Vec<_> = subcommand_iter.chain(subcommand_group_iter).collect();
                if !child_members.is_empty() {
                    top_level = top_level.set_options(child_members);
                }

                top_level
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
        CommandTreeTop::Complex {
            name: "event",
            description: "Event commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            subcommands: vec![],
            subcommand_groups: vec![
                CommandTreeIntermediate {
                    name: "participation",
                    description: "Commands for tracking event participation",
                    children: vec![
                        RequestKind::EventParticipantRecord,
                        RequestKind::EventParticipantRemove,
                        RequestKind::EventParticipantCheck,
                    ],
                },
            ],
        },
        CommandTreeTop::Complex {
            name: "industry",
            description: "Industry commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            subcommands: vec![],
            subcommand_groups: vec![
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
                        RequestKind::IndustryProfitClearUnknown,
                    ],
                },
            ],
        },

        CommandTreeTop::NakedUser(RequestKind::NavyVictoryRecordOneUser, None),
        CommandTreeTop::NakedUser(RequestKind::NavyVictoryCheckUser, None),
        CommandTreeTop::Complex {
            name: "navy",
            description: "Navy commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            subcommands: vec![],
            subcommand_groups: vec![
                CommandTreeIntermediate {
                    name: "victory",
                    description: "Commands for managing victory counts",
                    children: vec![
                        RequestKind::NavyVictoryRecord,
                        RequestKind::NavyVictoryDelete,
                        RequestKind::NavyVictoryBoast,
                        RequestKind::NavyVictoryCheck,
                        RequestKind::NavyVictoryScoreboard,
                        RequestKind::NavyVictoryClearUnknown,
                    ],
                },
                CommandTreeIntermediate {
                    name: "tackle_assist",
                    description: "Commands for managing tackle assist counts",
                    children: vec![
                        RequestKind::NavyTackleAssistRecord,
                        RequestKind::NavyTackleAssistDelete,
                        RequestKind::NavyTackleAssistBoast,
                        RequestKind::NavyTackleAssistCheck,
                        RequestKind::NavyTackleAssistScoreboard,
                        RequestKind::NavyTackleAssistClearUnknown,
                    ],
                },
            ],
        },
        CommandTreeTop::Complex {
            name: "legion",
            description: "Legion commands",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            subcommands: vec![],
            subcommand_groups: vec![
                CommandTreeIntermediate {
                    name: "kill",
                    description: "Commands for managing kill counts",
                    children: vec![
                        RequestKind::LegionKillRecord,
                        RequestKind::LegionKillDelete,
                        RequestKind::LegionKillBoast,
                        RequestKind::LegionKillCheck,
                        RequestKind::LegionKillScoreboard,
                        RequestKind::LegionKillClearUnknown,
                    ],
                },
            ],
        },
    ]
}

#[derive(Debug)]
pub enum RequestArgs<'a> {
    Ping,

    EventParticipantRecord(event::participation::record::Request<'a>),
    EventParticipantRemove(event::participation::remove::Request<'a>),
    EventParticipantCheck(event::participation::check::Request),

    IndustryMiningRockRecord,

    IndustryProfitRecord(industry::profit::record::Request),
    IndustryProfitDelete(industry::profit::delete::Request),
    IndustryProfitBoast(industry::profit::boast::Request),
    IndustryProfitCheck(industry::profit::check::Request),
    IndustryProfitScoreboard(industry::profit::scoreboard::Request<'a>),
    IndustryProfitClearUnknown(industry::profit::clear::Request),

    NavyVictoryRecord(navy::victory::record::Request),
    NavyVictoryDelete(navy::victory::delete::Request),
    NavyVictoryBoast(navy::victory::boast::Request),
    NavyVictoryCheck(navy::victory::check::Request),
    NavyVictoryScoreboard(navy::victory::scoreboard::Request<'a>),
    NavyVictoryClearUnknown(navy::victory::clear::Request),

    NavyTackleAssistRecord(navy::tackle_assist::record::Request),
    NavyTackleAssistDelete(navy::tackle_assist::delete::Request),
    NavyTackleAssistBoast(navy::tackle_assist::boast::Request),
    NavyTackleAssistCheck(navy::tackle_assist::check::Request),
    NavyTackleAssistScoreboard(navy::tackle_assist::scoreboard::Request<'a>),
    NavyTackleAssistClearUnknown(navy::tackle_assist::clear::Request),

    LegionKillRecord(legion::kill::record::Request),
    LegionKillDelete(legion::kill::delete::Request),
    LegionKillBoast(legion::kill::boast::Request),
    LegionKillCheck(legion::kill::check::Request),
    LegionKillScoreboard(legion::kill::scoreboard::Request<'a>),
    LegionKillClearUnknown(legion::kill::clear::Request),
}

impl <'a> RequestArgs<'a> {
    pub fn parse(cmd: &'a CommandInteraction) -> Result<Self, RequestError> {
        match cmd.data.name.as_str() {
            "ping" => {
                Ok(RequestArgs::Ping)
            },
            "event" => {
                let tier0_options = cmd.data.options();
                let Some(tier1) = tier0_options.first() else {
                    return Err(RequestError::Internal("Missing options for `event`.".into()));
                };
                match tier1.name {
                    "participation" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `participation`.".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
                            return Err(RequestError::Internal("Missing options for `event participation`.".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `event participation`.".into()));
                        };
                        match tier2.name {
                            "record" => {
                                Ok(RequestArgs::EventParticipantRecord(event::participation::record::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "remove" => {
                                Ok(RequestArgs::EventParticipantRemove(event::participation::remove::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "check" => {
                                Ok(RequestArgs::EventParticipantCheck(event::participation::check::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                Err(RequestError::Internal("Unknown subcommand for `event participation`".into()))
                            },
                        }
                    },
                    _ => {
                        Err(RequestError::Internal("Bad subcommand for `event`.".into()))
                    },
                }
            },
            "industry" => {
                let tier0_options = cmd.data.options();
                let Some(tier1) = tier0_options.first() else {
                    return Err(RequestError::Internal("Missing options for `industry`.".into()));
                };
                match tier1.name {
                    "profit" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `industry`.".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
                            return Err(RequestError::Internal("Missing options for `industry profit`.".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `industry profit`.".into()));
                        };
                        match tier2.name {
                            "record" => {
                                Ok(RequestArgs::IndustryProfitRecord(industry::profit::record::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "delete" => {
                                Ok(RequestArgs::IndustryProfitDelete(industry::profit::delete::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "boast" => {
                                Ok(RequestArgs::IndustryProfitBoast(industry::profit::boast::Request::parse(cmd, &[])?))
                            },
                            "check" => {
                                Ok(RequestArgs::IndustryProfitCheck(industry::profit::check::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "scoreboard" => {
                                Ok(RequestArgs::IndustryProfitScoreboard(industry::profit::scoreboard::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "clear_unknown" => {
                                Ok(RequestArgs::IndustryProfitClearUnknown(industry::profit::clear::Request::parse(cmd, &[])?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                Err(RequestError::Internal("Unknown subcommand for `industry profit`".into()))
                            },
                        }
                    },
                    "mining" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `industry`.".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
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
                        Err(RequestError::Internal("Bad subcommand for `industry`.".into()))
                    },
                }
            },
            "Record One Naval Victory" => {
                Ok(RequestArgs::NavyVictoryRecord(navy::victory::record::Request::parse(cmd, &[])?))
            },
            "Check Naval Victories" => {
                Ok(Self::NavyVictoryCheck(navy::victory::check::Request::parse(cmd, &[])?))
            },
            "navy" => {
                let tier0_options: Vec<ResolvedOption<'a>> = cmd.data.options();
                let Some(tier1) = tier0_options.first() else {
                    return Err(RequestError::Internal("Missing options for `navy`.".into()));
                };
                match tier1.name {
                    "victory" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `navy`".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
                            return Err(RequestError::Internal("Missing options for `navy victory`".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `navy victory`".into()));
                        };
                        match tier2.name {
                            "record" => {
                                Ok(RequestArgs::NavyVictoryRecord(navy::victory::record::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "delete" => {
                                Ok(RequestArgs::NavyVictoryDelete(navy::victory::delete::Request::parse(cmd, tier2_options.as_slice())?))
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
                            "clear_unknown" => {
                                Ok(RequestArgs::NavyVictoryClearUnknown(navy::victory::clear::Request::parse(cmd, &[])?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                Err(RequestError::Internal("Unknown subcommand for `navy victory`".into()))
                            },
                        }
                    },
                    "tackle_assist" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `navy`".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
                            return Err(RequestError::Internal("Missing options for `navy tackle_assist`".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `navy tackle_assist`".into()));
                        };
                        match tier2.name {
                            "record" => {
                                Ok(RequestArgs::NavyTackleAssistRecord(navy::tackle_assist::record::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "delete" => {
                                Ok(RequestArgs::NavyTackleAssistDelete(navy::tackle_assist::delete::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "boast" => {
                                Ok(RequestArgs::NavyTackleAssistBoast(navy::tackle_assist::boast::Request::parse(cmd, &[])?))
                            },
                            "check" => {
                                Ok(RequestArgs::NavyTackleAssistCheck(navy::tackle_assist::check::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "scoreboard" => {
                                Ok(RequestArgs::NavyTackleAssistScoreboard(navy::tackle_assist::scoreboard::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "clear_unknown" => {
                                Ok(RequestArgs::NavyTackleAssistClearUnknown(navy::tackle_assist::clear::Request::parse(cmd, &[])?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                Err(RequestError::Internal("Unknown subcommand for `navy tackle_assist`".into()))
                            },
                        }
                    },
                    _ => {
                        trc::warn!("Unknown subcommand {:?}", tier1);
                        Err(RequestError::Internal("Unknown subcommand for `navy`".into()))
                    },
                }
            },
            "legion" => {
                let tier0_options: Vec<ResolvedOption<'a>> = cmd.data.options();
                let Some(tier1) = tier0_options.first() else {
                    return Err(RequestError::Internal("Missing options for `legion`.".into()));
                };
                match tier1.name {
                    "kill" => {
                        let ResolvedValue::SubCommandGroup(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing subcommand group for `legion`".into()));
                        };
                        let Some(tier2) = tier1_options.first() else {
                            return Err(RequestError::Internal("Missing options for `legion kill`".into()));
                        };
                        let ResolvedValue::SubCommand(ref tier2_options) = tier2.value else {
                            return Err(RequestError::Internal("Missing subcommand for `legion kill`".into()));
                        };
                        match tier2.name {
                            "record" => {
                                Ok(RequestArgs::LegionKillRecord(legion::kill::record::Request::parse(cmd, tier2_options.as_slice())?))
                            },
                            "delete" => {
                                Ok(RequestArgs::LegionKillDelete(legion::kill::delete::Request::parse(cmd, tier2_options.as_slice())?))
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
                            "clear_unknown" => {
                                Ok(RequestArgs::LegionKillClearUnknown(legion::kill::clear::Request::parse(cmd, &[])?))
                            },
                            _ => {
                                trc::warn!("Unknown subcommand {:?}", tier1);
                                Err(RequestError::Internal("Unknown subcommand for `legion kill`".into()))
                            },
                        }
                    },
                    _ => {
                        trc::warn!("Unknown subcommand {:?}", tier1);
                        Err(RequestError::Internal("Unknown subcommand for `legion`".into()))
                    },
                }
            },
            _ => {
                trc::error!("Unknown command {:?} received", cmd);
                Err(RequestError::Internal("Unknown command.".into()))
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

            RequestArgs::EventParticipantRemove(req) => {
                req.execute(ctx).await
            },
            RequestArgs::EventParticipantRecord(req) => {
                req.execute(ctx).await
            },
            RequestArgs::EventParticipantCheck(req) => {
                req.execute(ctx).await
            },

            RequestArgs::IndustryMiningRockRecord => {
                ctx.reply("Industry mining number crunching not yet implemented.".to_owned()).await
            },

            RequestArgs::IndustryProfitRecord(req) => {
                req.execute(ctx).await
            },
            RequestArgs::IndustryProfitDelete(req) => {
                req.execute(ctx).await
            },
            RequestArgs::IndustryProfitBoast(req) => {
                req.execute(ctx).await
            },
            RequestArgs::IndustryProfitCheck(req) => {
                req.execute(ctx).await
            },
            RequestArgs::IndustryProfitScoreboard(req) => {
                req.execute(ctx).await
            },
            RequestArgs::IndustryProfitClearUnknown(req) => {
                req.execute(ctx).await
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
            RequestArgs::NavyVictoryClearUnknown(req) => {
                req.execute(ctx).await
            },

            RequestArgs::NavyTackleAssistRecord(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyTackleAssistDelete(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyTackleAssistBoast(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyTackleAssistCheck(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyTackleAssistScoreboard(req) => {
                req.execute(ctx).await
            },
            RequestArgs::NavyTackleAssistClearUnknown(req) => {
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
            RequestArgs::LegionKillClearUnknown(req) => {
                req.execute(ctx).await
            },
        }
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use strum::{EnumCount, IntoEnumIterator};

    use super::{generate_command_descriptions, RequestKind, CommandTreeTop};

    fn iter_tree(tree: &CommandTreeTop, set: &mut HashSet<RequestKind>) {
        match tree {
            CommandTreeTop::Complex { ref subcommand_groups, ref subcommands, .. } => {
                for c in subcommand_groups.iter().flat_map(|g| g.children.iter()).chain(subcommands.iter()) {
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
