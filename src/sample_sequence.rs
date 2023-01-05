/* schala's theme lol */
use crate::timed_event::{build_timed_note, TimedEvent};
use midi_types::Note;

pub fn build_sequence(step: usize) -> Vec<TimedEvent> {
    vec![
        /* bells only */
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        /* strings */
        build_timed_note(0, 2, Note::As4, 64),
        /* second bell */
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        /* strings */
        build_timed_note(0, 2, Note::As4, 0),
        build_timed_note(0, 2, Note::Gs4, 64),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        /* strings */
        build_timed_note(0, 2, Note::Gs4, 0),
        build_timed_note(0, 2, Note::F4, 64),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
        /* strings */
        build_timed_note(0, 2, Note::F4, 0),
        build_timed_note(0, 2, Note::Ds4, 64),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(step, 0, Note::F5, 0),
        /* strings */
        build_timed_note(0, 2, Note::Ds4, 0),
        build_timed_note(0, 2, Note::Cs4, 64),
        build_timed_note(0, 0, Note::As4, 127),
        build_timed_note(step, 0, Note::As5, 127),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::As4, 96),
        build_timed_note(step, 0, Note::As5, 96),
        build_timed_note(0, 0, Note::As4, 0),
        build_timed_note(step, 0, Note::As5, 0),
        build_timed_note(0, 0, Note::Cs5, 127),
        build_timed_note(step, 0, Note::F5, 127),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        build_timed_note(0, 0, Note::Cs5, 96),
        build_timed_note(step, 0, Note::F5, 96),
        build_timed_note(0, 0, Note::Cs5, 0),
        build_timed_note(step, 0, Note::F5, 0),
        /* strings */
        build_timed_note(0, 2, Note::Cs4, 0),
        build_timed_note(0, 2, Note::C4, 64),
        build_timed_note(0, 0, Note::Ds5, 127),
        build_timed_note(step, 0, Note::C6, 127),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(0, 0, Note::Ds5, 96),
        build_timed_note(step, 0, Note::C6, 96),
        build_timed_note(0, 0, Note::Ds5, 0),
        build_timed_note(step, 0, Note::C6, 0),
        build_timed_note(step, 0, Note::Gs5, 127),
        build_timed_note(step, 0, Note::Gs5, 0),
        build_timed_note(step, 0, Note::Gs5, 96),
        build_timed_note(step, 0, Note::Gs5, 0),
    ]
}
