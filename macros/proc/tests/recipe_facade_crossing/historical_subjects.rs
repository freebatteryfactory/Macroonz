//! Historical subjects observed through the final facade and callable compiler surfaces.

use super::support::{observe_subject_journeys, observed_in_scratch};

#[test]
fn historical_subjects_share_one_final_facade_and_callable_compiler_surface() -> Result<(), String>
{
    observed_in_scratch(observe_subject_journeys)
}
