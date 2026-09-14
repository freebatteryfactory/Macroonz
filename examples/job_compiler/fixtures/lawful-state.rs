use neutral_job_adopter::job::{Event, Stage, baked};
fn main() {
    assert_eq!(baked::apply(Stage::Draft, Event::Queue), Ok(Stage::Queued));
}
