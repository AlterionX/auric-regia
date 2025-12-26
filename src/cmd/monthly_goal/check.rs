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
            self.execute_branch_summary(ctx).await
        } else {
            self.execute_main_summary(ctx).await
        }
    }

    pub async fn execute_main_summary(self, ctx: &ExecutionContext<'_>) -> Result<(), RequestError> {
        let Ok(main_data) = db::MonthlyGoal::load_detailed_summary(&ctx.db_cfg, "main").await else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };

        let Ok(mut branch_data) = db::MonthlyGoal::load_primary_summary(&ctx.db_cfg).await else {
            return Err(RequestError::Internal("Failed to load monthly goals.".into()));
        };
        // We're calculating this on the side, ignore any values from the db for this.
        branch_data.remove("main");
        let branch_data = branch_data;

        if main_data.len() + branch_data.len() == 0 {
            ctx.reply_restricted("No goals have been set up!".to_owned()).await?;
            return Ok(());
        }

        let all_progress = main_data.iter().map(|goal| goal.progress as f64)
            .chain(branch_data.iter().map(|(_branch, (progress, total_progress))| 100. * (*progress as f64 / *total_progress as f64)))
            .map(|progress| progress.min(100.) as usize)
            .sum();
        let total_possible_progress = 100 * (main_data.len() + branch_data.len());

        let msg: String = std::iter::once(format!(
            "\
                # Goal Progress: Main\n\
                \n\
                Progress ({:.2}%): ```ansi\n{}\n```\n\
            ",
            (all_progress as f64 / total_possible_progress as f64).clamp(0., 1.) * 100.,
            progrs_bar::Bar::new(all_progress, total_possible_progress.max(1)).generate_string(25, fetch_branch_color("main"))
        ))
            .chain(branch_data.into_iter().map(|(branch_name, (branch_progress, branch_goals_count))| {
                format!(
                    "\
                        ## {}\n\
                        \n\
                        Progress ({:.2}%): ```ansi\n{}\n```\n\
                    ",
                    fetch_branch_display_name(branch_name.as_str()),
                    (branch_progress as f64 / (branch_goals_count * 100).max(1) as f64).clamp(0., 1.) * 100.,
                    progrs_bar::Bar::new(
                        usize::try_from(branch_progress).unwrap_or(0),
                        usize::try_from((branch_goals_count * 100).max(1)).unwrap_or(1),
                    ).generate_string(25, fetch_branch_color(branch_name.as_str())),
                )
            }))
            .chain(main_data.into_iter().map(|goal| {
                format!(
                    "\
                        ## {}\n\
                        {}\n\
                        \n\
                        Progress ({:.2}%): ```ansi\n{}\n```\n\
                    ",
                    goal.header,
                    goal.body,
                    goal.progress as f64,
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

        if data.len() == 0 {
            ctx.reply_restricted("No goals have been set up!".to_owned()).await?;
            return Ok(());
        }

        let branch_color = fetch_branch_color(self.branch);

        let all_progress = data.iter().map(|goal| usize::try_from(goal.progress).unwrap_or(0)).sum();
        let total_possible_progress = 100 * data.len();

        let msg: String = std::iter::once(format!(
                "\
                    # Goal Progress: {}\n\
                    Progress ({:.2}%): ```ansi\n{}\n```\n\
                ",
                fetch_branch_display_name(self.branch),
                (all_progress as f64 / total_possible_progress as f64) * 100.,
                progrs_bar::Bar::new(all_progress, total_possible_progress).generate_string(25, branch_color),
            ))
            .chain(data.into_iter().map(|goal| {
                format!(
                    "\
                        ## {}\n\
                        {}\n\
                        \n\
                        Progress ({:.2}%): ```ansi\n{}\n```\n\
                    ",
                    goal.header,
                    goal.body,
                    goal.progress as f64,
                    progrs_bar::Bar::new(usize::try_from(goal.progress).unwrap_or(0), 100).generate_string(25, branch_color),
                )
            }))
            .collect();

        ctx.reply_restricted(msg).await?;

        Ok(())
    }
}

pub fn fetch_branch_display_name(branch: &str) -> &'static str {
    match branch {
        "navy" => "Navy",
        "legion" => "Legion",
        "industry" => "Industry",
        "main" => "Main",
        _ => "Other",
    }
}

pub fn fetch_branch_color(branch: &str) -> crossterm::style::Color {
    match branch {
        "navy" => Some(crossterm::style::Color::AnsiValue(34)),
        "legion" => Some(crossterm::style::Color::AnsiValue(32)),
        "industry" => Some(crossterm::style::Color::AnsiValue(35)),
        "main" => Some(crossterm::style::Color::AnsiValue(33)),
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
