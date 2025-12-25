use diesel::IntoSql;
use serenity::all::{CommandInteraction, ResolvedOption, ResolvedValue};
use tracing as trc;

use azel::discord::ExecutionContext;

use crate::{cmd::RequestError, db};

#[derive(Debug)]
pub struct Request<'a> {
    branch: &'a str,
}

impl<'a> Request<'a> {
    pub fn parse(_cmd: &'a CommandInteraction, options: &'_ [ResolvedOption<'a>]) -> Result<Self, RequestError> {
        let mut branch = "main";
        for opt in options {
            match opt.name {
                "branch" => {
                    let ResolvedValue::String(u) = opt.value else {
                        trc::error!("Bad value for `branch` in `monthly_goal set` {:?}", opt);
                        return Err(RequestError::Internal("Bad value for `branch` in `monthly_goal set`.".into()));
                    };
                    branch = u;
                }
                _ => {
                    trc::error!("Unknown option `{}` for `monthly_goal set`", opt.name);
                    return Err(RequestError::Internal("Unknown option in `monthly_goal set`".into()));
                }
            }
        }

        Ok(Self {
            branch,
        })
    }

    pub async fn execute(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        if self.branch != "main" {
            self.execute_main_summary(ctx).await
        } else {
            self.execute_branch_summary(ctx).await
        }
    }

    pub async fn execute_main_summary(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(main_data) = db::MonthlyGoal::load_detailed_summary(&ctx.db_cfg, "main").await else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };

        let Ok(branch_data) = db::MonthlyGoal::load_primary_summary(&ctx.db_cfg).await else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };

        let all_progress = main_data.iter().map(|goal| i64::from(goal.progress))
            .chain(branch_data.iter().map(|(_branch, (progress, _total_progress))| *progress))
            .map(|progress| usize::try_from(progress).unwrap_or(0))
            .sum();
        let total_possible_progress = branch_data.iter().map(|(_branch, (_progress, total_progress))| *total_progress)
            .map(|progress| usize::try_from(progress).unwrap_or(0))
            .chain(std::iter::once(100 * main_data.len()))
            .sum();

        let msg: String = std::iter::once(format!(
            "\
                # Goal Progress: Main\n\
                \n\
                Progress: ```{}```\n\
                \n\
            ",
            progrs_bar::Bar::new(all_progress, total_possible_progress).generate_string(25, fetch_branch_color("main"))
        ))
            .chain(branch_data.into_iter().map(|(branch_name, (branch_progress, branch_total_progress))| {
                format!(
                    "\
                        ## {}\n\
                        \n\
                        Progress: ```{}```\n\
                        \n\
                    ",
                    branch_name,
                    progrs_bar::Bar::new(
                        usize::try_from(branch_progress).unwrap_or(0),
                        usize::try_from(branch_total_progress).unwrap_or(1),
                    ).generate_string(25, fetch_branch_color(branch_name.as_str())),
                )
            }))
            .chain(main_data.into_iter().map(|goal| {
                format!(
                    "\
                        ## {}\n\
                        {}\n\
                        \n\
                        Progress: ```{}```\n\
                        \n\
                    ",
                    goal.header,
                    goal.body,
                    progrs_bar::Bar::new(usize::try_from(goal.progress).unwrap_or(0), 100).generate_string(25, fetch_branch_color(goal.tag.as_str())),
                )
            }))
            .collect();

        ctx.reply_restricted(msg).await?;

        Ok(())
    }

    pub async fn execute_branch_summary(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(data) = db::MonthlyGoal::load_detailed_summary(&ctx.db_cfg, self.branch).await else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };

        let branch_color = fetch_branch_color(self.branch);

        let all_progress = data.iter().map(|goal| usize::try_from(goal.progress).unwrap_or(0)).sum();
        let total_possible_progress = 100 * data.len();

        let msg: String = std::iter::once(progrs_bar::Bar::new(all_progress, total_possible_progress).generate_string(25, branch_color))
            .chain(data.into_iter().map(|goal| {
                format!(
                    "\
                        ## {}\n\
                        {}\n\
                        \n\
                        Progress: ```{}```\n\
                        \n\
                    ",
                    goal.header,
                    goal.body,
                    progrs_bar::Bar::new(usize::try_from(goal.progress).unwrap_or(0), 100).generate_string(25, branch_color),
                )
            }))
            .collect();

        ctx.reply_restricted(msg).await?;

        Ok(())
    }
}

pub fn fetch_branch_color(branch: &str) -> crossterm::style::Color {
    match branch {
        "navy" => Some(crossterm::style::Color::AnsiValue(27)),
        "legion" => Some(crossterm::style::Color::AnsiValue(41)),
        "industry" => Some(crossterm::style::Color::AnsiValue(55)),
        "main" => Some(crossterm::style::Color::AnsiValue(221)),
        _ => Some(crossterm::style::Color::White),
    }.expect(branch)
}

#[cfg(test)]
mod test {
    use crate::cmd::monthly_goal::check::fetch_branch_color;

    #[test]
    fn test_ansi_parse() {
        for branch in ["navy", "legion", "industry"] {
            fetch_branch_color(branch);
        }
    }
}
