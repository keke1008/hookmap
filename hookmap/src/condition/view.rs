use bitvec::prelude::BitVec;

use super::flag::{FlagIndex, FlagState};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct View {
    enabled_flags: BitVec,
    disabled_flags: BitVec,
}

impl View {
    pub(crate) fn is_enabled(&self, state: &FlagState) -> bool {
        let state = state.as_raw_slice();
        state
            .iter()
            .zip(self.enabled_flags.as_raw_slice())
            .all(|(state, &flag)| state & flag == flag)
            && state
                .iter()
                .zip(self.disabled_flags.as_raw_slice())
                .all(|(state, &flag)| state & flag == 0)
    }

    pub(super) fn iter_enabled_flags(&self) -> impl Iterator<Item = FlagIndex> + '_ {
        self.enabled_flags.iter_ones().map(FlagIndex)
    }

    pub(super) fn iter_disabled_flags(&self) -> impl Iterator<Item = FlagIndex> + '_ {
        self.disabled_flags.iter_ones().map(FlagIndex)
    }
}

fn set_with_extend(target: &mut BitVec, index: usize, state: bool) {
    let len = target.len();

    if len <= index {
        target.resize(index + 1, false);
    }

    target.set(index, state);
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ViewBuilder {
    enabled_flags: BitVec,
    disabled_flags: BitVec,
}

impl ViewBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enabled(mut self, flag: FlagIndex) -> Self {
        set_with_extend(&mut self.enabled_flags, flag.0, true);
        self
    }

    pub fn disabled(mut self, flag: FlagIndex) -> Self {
        set_with_extend(&mut self.disabled_flags, flag.0, true);
        self
    }

    pub fn merge(mut self, other: &View) -> Self {
        for index in other.enabled_flags.iter_ones() {
            set_with_extend(&mut self.enabled_flags, index, true);
        }
        for index in other.disabled_flags.iter_ones() {
            set_with_extend(&mut self.disabled_flags, index, true);
        }
        self
    }

    pub fn build(self) -> View {
        View {
            enabled_flags: self.enabled_flags,
            disabled_flags: self.disabled_flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_view_with_empty_state() {
        let state = FlagState::default();
        let view = View::default();
        assert!(view.is_enabled(&state));
    }

    #[test]
    fn single_enabled_flag() {
        let mut state = FlagState::default();
        let flag = state.create_flag(true);

        let enable_view = ViewBuilder::new().enabled(flag).build();
        assert!(enable_view.is_enabled(&state));

        let disable_view = ViewBuilder::new().disabled(flag).build();
        assert!(!disable_view.is_enabled(&state));

        let empty_view = View::default();
        assert!(empty_view.is_enabled(&state));
    }

    #[test]
    fn single_disable_flag() {
        let mut state = FlagState::default();
        let flag = state.create_flag(false);

        let enable_view = ViewBuilder::new().enabled(flag).build();
        assert!(!enable_view.is_enabled(&state));

        let disable_view = ViewBuilder::new().disabled(flag).build();
        assert!(disable_view.is_enabled(&state));

        let empty_view = View::default();
        assert!(empty_view.is_enabled(&state));
    }

    #[test]
    fn multi_flags() {
        let mut state = FlagState::default();
        let flag1 = state.create_flag(true);
        let flag2 = state.create_flag(false);
        let flag3 = state.create_flag(true);

        let view = ViewBuilder::new()
            .enabled(flag1)
            .disabled(flag2)
            .enabled(flag3)
            .build();
        assert!(view.is_enabled(&state));

        let view = ViewBuilder::new().enabled(flag2).build();
        assert!(!view.is_enabled(&state));

        let view = ViewBuilder::new().disabled(flag1).build();
        assert!(!view.is_enabled(&state));
    }
}
