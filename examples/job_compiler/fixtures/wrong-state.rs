use neutral_job_adopter::job::{Event, baked};
fn main() {
    let _ = baked::apply(7u8, Event::Queue);
}
