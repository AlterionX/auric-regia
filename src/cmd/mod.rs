pub mod navy;
pub mod legion;
pub mod industry;
pub mod event;
pub mod monthly_goal;

use tracing as trc;

use serenity::all::{CommandInteraction, CommandType, ResolvedOption, ResolvedValue};
use strum::{EnumCount, EnumDiscriminants, EnumIter};

use azel::{cmd::{CommandTreeTop, CommandTreeIntermediate, DiscordCommandArgs, DiscordCommandDescriptor, RawCommandOptionEntry, RequestError}, discord::ExecutionContext};

#[derive(Debug)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(Hash, EnumCount, EnumIter))]
#[strum_discriminants(name(RequestKind))]
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

    // Dummy variants needed for the request kind enum, these are
    // subsumed into the ones below.
    #[allow(dead_code)]
    NavyVictoryRecordOneUser(!),
    #[allow(dead_code)]
    NavyVictoryCheckUser(!),

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

    MonthlyGoalCheck(monthly_goal::check::Request<'a>),
    MonthlyGoalSet(monthly_goal::set::Request<'a>),
    MonthlyGoalClear(monthly_goal::clear::Request),
    MonthlyGoalAdminList(monthly_goal::admin_list::Request<'a>),
}

impl DiscordCommandDescriptor for RequestKind {
    type Args<'a> = RequestArgs<'a>;

    fn name(&self) -> &'static str {
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
            RequestKind::MonthlyGoalClear => {
                "clear"
            },
            RequestKind::MonthlyGoalAdminList => {
                "admin_list"
            },
        }
    }

    fn description(&self) -> &'static str {
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
            RequestKind::MonthlyGoalClear => {
                "Clear all monthly goals"
            },
            RequestKind::MonthlyGoalAdminList => {
                "List out goals including shortnames"
            },
        }
    }

    fn options(&self) -> Vec<RawCommandOptionEntry> {
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
                    RawCommandOptionEntry::String {
                        name: "shortname",
                        description: "Name for the goal. If active doesn't exist, will create",
                        required: true,
                    },
                    RawCommandOptionEntry::LimitedInteger {
                        name: "progress",
                        description: "Progress of the goal, up to 100",
                        required: false,
                        max: 100,
                        min: 0,
                    },
                    RawCommandOptionEntry::String {
                        name: "header",
                        description: "Header of the message, as of 2025/12/16 max 100 chars",
                        required: false,
                    },
                    RawCommandOptionEntry::String {
                        name: "body",
                        description: "Body of the message, as of 2025/12/16 max 5000 chars",
                        required: false,
                    },
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
                ]
            },
            RequestKind::MonthlyGoalClear => {
                vec![]
            },
            RequestKind::MonthlyGoalAdminList => {
                vec![
                    RawCommandOptionEntry::String {
                        name: "branch",
                        description: "Tag of the goal. will get all if not provided",
                        required: false,
                    },
                ]
            },
        }
    }

    fn parse<'a>(cmd: &'a CommandInteraction) -> Result<Self::Args<'a>, RequestError> {
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
                Ok(RequestArgs::NavyVictoryCheck(navy::victory::check::Request::parse(cmd, &[])?))
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
            "monthly_goal" => {
                let tier0_options: Vec<ResolvedOption<'a>> = cmd.data.options();
                let Some(tier1) = tier0_options.first() else {
                    return Err(RequestError::Internal("Missing options for `monthly_goal`.".into()));
                };
                match tier1.name {
                    "set" => {
                        let ResolvedValue::SubCommand(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing options for `monthly_goal set`".into()));
                        };
                        Ok(RequestArgs::MonthlyGoalSet(monthly_goal::set::Request::parse(cmd, tier1_options.as_slice())?))
                    },
                    "check" => {
                        let ResolvedValue::SubCommand(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing options for `monthly_goal check`".into()));
                        };
                        Ok(RequestArgs::MonthlyGoalCheck(monthly_goal::check::Request::parse(cmd, tier1_options.as_slice())?))
                    },
                    "clear" => {
                        let ResolvedValue::SubCommand(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing options for `monthly_goal check`".into()));
                        };
                        Ok(RequestArgs::MonthlyGoalClear(monthly_goal::clear::Request::parse(cmd, tier1_options.as_slice())?))
                    },
                    "admin_list" => {
                        let ResolvedValue::SubCommand(ref tier1_options) = tier1.value else {
                            return Err(RequestError::Internal("Missing options for `monthly_goal check`".into()));
                        };
                        Ok(RequestArgs::MonthlyGoalAdminList(monthly_goal::admin_list::Request::parse(cmd, tier1_options.as_slice())?))
                    },
                    _ => {
                        trc::warn!("Unknown subcommand {:?}", tier1);
                        Err(RequestError::Internal("Unknown subcommand for `legion kill`".into()))
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

impl <'a> DiscordCommandArgs for RequestArgs<'a> {
    async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        match self {
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

            RequestArgs::MonthlyGoalCheck(req) => {
                req.execute(ctx).await
            },
            RequestArgs::MonthlyGoalSet(req) => {
                req.execute(ctx).await
            },
            RequestArgs::MonthlyGoalClear(req) => {
                req.execute(ctx).await
            },
            RequestArgs::MonthlyGoalAdminList(req) => {
                req.execute(ctx).await
            },
        }
    }
}

#[tracing::instrument(name = "hello")]
pub fn generate_command_descriptions() -> Vec<CommandTreeTop<RequestKind>> {
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
        CommandTreeTop::Complex {
            name: "monthly_goal",
            description: "Commands for managing monthly goals",
            kind: CommandType::ChatInput,
            opt_default_perm: None,
            subcommands: vec![
                RequestKind::MonthlyGoalSet,
                RequestKind::MonthlyGoalCheck,
                RequestKind::MonthlyGoalClear,
                RequestKind::MonthlyGoalAdminList,
            ],
            subcommand_groups: vec![],
        },
    ]
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use strum::EnumCount;

    use super::{generate_command_descriptions, RequestKind, CommandTreeTop};

    fn iter_tree(tree: &CommandTreeTop<RequestKind>, set: &mut HashSet<RequestKind>) {
        match tree {
            CommandTreeTop::Complex { ref subcommand_groups, ref subcommands, .. } => {
                for c in subcommand_groups.iter().flat_map(|g| g.children.iter()).chain(subcommands.iter()) {
                    assert!(!set.contains(c));
                    set.insert(*c);
                }
            },
            CommandTreeTop::GlobalMessageContextMenu(cmd, _) | CommandTreeTop::NakedUser(cmd, _) | CommandTreeTop::NakedChatInput(cmd, _) | CommandTreeTop::MessageContextMenu(cmd, _) => {
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
        azel::cmd::test_utils::test_command_description_lengths::<RequestKind>();
    }
}
