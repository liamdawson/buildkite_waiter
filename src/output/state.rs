use buildkite_waiter::{Build, BuildStateGroup};
use console::StyledObject;
use heck::ToTitleCase;

pub enum FormatBuildStateCase {
    Title,
    Lower,
}

pub trait FormatBuildState {
    fn state_case(&self, case: FormatBuildStateCase) -> String;
    fn colored_state_case(&self, case: FormatBuildStateCase) -> StyledObject<String>;
}

impl FormatBuildState for Build {
    fn state_case(&self, case: FormatBuildStateCase) -> String {
        match case {
            FormatBuildStateCase::Lower => self.state.to_title_case().to_ascii_lowercase(),
            FormatBuildStateCase::Title => self.state.to_title_case(),
        }
    }

    fn colored_state_case(&self, case: FormatBuildStateCase) -> StyledObject<String> {
        let styled = console::style(self.state_case(case));

        match self.state_group() {
            BuildStateGroup::Blocked => styled.yellow(),
            BuildStateGroup::Unfinished => styled.magenta(),
            BuildStateGroup::Successful => styled.green(),
            BuildStateGroup::Unsuccessful => styled.red(),
        }
    }
}
