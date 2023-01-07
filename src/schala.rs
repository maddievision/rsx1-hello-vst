use crate::cmd_sequence::{Cmd, CmdSequence, Note, NoteEvent, Playlist, TimedCmdEvent};
use crate::timed_event::EventSequence;

pub fn build_timed_note(frames: usize, channel: u8, note: Note, velocity: u8) -> TimedCmdEvent {
    TimedCmdEvent::new(
        frames,
        Cmd::NoteCmd(NoteEvent::new(channel, note, velocity)),
    )
}

pub fn build_schala(l32: usize) -> EventSequence {
    let l16 = l32 * 2;
    let l8 = l16 * 2;
    let l4 = l8 * 2;
    let l2 = l4 * 2;
    let l1 = l2 * 2;
    Playlist::new(vec![
        CmdSequence::new(
            0,
            vec![
                /* bells only */
                TimedCmdEvent::new(0, Cmd::RepeatCmd(6)),
                build_timed_note(l32, 0, Note::As5, 127),
                build_timed_note(l32, 0, Note::As5, 0),
                build_timed_note(l32, 0, Note::As5, 96),
                build_timed_note(l32, 0, Note::As5, 0),
                build_timed_note(l32, 0, Note::F5, 127),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::F5, 96),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::Gs5, 127),
                build_timed_note(l32, 0, Note::Gs5, 0),
                build_timed_note(l32, 0, Note::Gs5, 96),
                build_timed_note(l32, 0, Note::Gs5, 0),
                build_timed_note(l32, 0, Note::F5, 127),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::F5, 96),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::As5, 127),
                build_timed_note(l32, 0, Note::As5, 0),
                build_timed_note(l32, 0, Note::As5, 96),
                build_timed_note(l32, 0, Note::As5, 0),
                build_timed_note(l32, 0, Note::F5, 127),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::F5, 96),
                build_timed_note(l32, 0, Note::F5, 0),
                build_timed_note(l32, 0, Note::C6, 127),
                build_timed_note(l32, 0, Note::C6, 0),
                build_timed_note(l32, 0, Note::C6, 96),
                build_timed_note(l32, 0, Note::C6, 0),
                build_timed_note(l32, 0, Note::Gs5, 127),
                build_timed_note(l32, 0, Note::Gs5, 0),
                build_timed_note(l32, 0, Note::Gs5, 96),
                build_timed_note(l32, 0, Note::Gs5, 0),
                TimedCmdEvent::new(0, Cmd::RepeatEndCmd),
            ],
        ),
        CmdSequence::new(
            l1 * 2,
            vec![
                /* bells only */
                TimedCmdEvent::new(0, Cmd::RepeatCmd(8)),
                build_timed_note(l32, 0, Note::As4, 127),
                build_timed_note(l32, 0, Note::As4, 0),
                build_timed_note(l32, 0, Note::As4, 96),
                build_timed_note(l32, 0, Note::As4, 0),
                build_timed_note(l32, 0, Note::Cs5, 127),
                build_timed_note(l32, 0, Note::Cs5, 0),
                build_timed_note(l32, 0, Note::Cs5, 96),
                build_timed_note(l32, 0, Note::Cs5, 0),
                build_timed_note(l32, 0, Note::Ds5, 127),
                build_timed_note(l32, 0, Note::Ds5, 0),
                build_timed_note(l32, 0, Note::Ds5, 96),
                build_timed_note(l32, 0, Note::Ds5, 0),
                TimedCmdEvent::new(l8, Cmd::WaitCmd),
                TimedCmdEvent::new(0, Cmd::RepeatEndCmd),
            ],
        ),
        CmdSequence::new(
            l1 * 2,
            vec![
                /* strings only */
                build_timed_note(l1, 2, Note::As4, 64),
                build_timed_note(0, 2, Note::As4, 0),
                build_timed_note(l1, 2, Note::Gs4, 64),
                build_timed_note(0, 2, Note::Gs4, 0),
                build_timed_note(l1, 2, Note::F4, 64),
                build_timed_note(0, 2, Note::F4, 0),
                build_timed_note(l2, 2, Note::Ds4, 64),
                build_timed_note(0, 2, Note::Ds4, 0),
                build_timed_note(l4, 2, Note::Cs4, 64),
                build_timed_note(0, 2, Note::Cs4, 0),
                build_timed_note(l4, 2, Note::C4, 64),
                build_timed_note(0, 2, Note::C4, 0),
            ],
        ),
    ])
    .into()
}
