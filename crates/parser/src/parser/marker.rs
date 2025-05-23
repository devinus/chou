use super::Parser;
use crate::event::Event;
use drop_bomb::DropBomb;
use syntax::SyntaxKind;

#[derive(Debug)]
pub(crate) struct Marker {
    pos: usize,
    bomb: DropBomb,
}

impl Marker {
    #[inline]
    pub(crate) fn new(pos: usize) -> Self {
        Self {
            pos,
            bomb: DropBomb::new("Markers need to be completed"),
        }
    }

    pub(crate) fn complete(mut self, p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        self.bomb.defuse();

        let event_at_pos = &mut p.events[self.pos];
        assert_eq!(*event_at_pos, Event::Placeholder);

        *event_at_pos = Event::StartNode {
            kind,
            forward_parent: None,
        };

        p.events.push(Event::FinishNode);

        CompletedMarker { pos: self.pos }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub(crate) fn precede(self, p: &mut Parser) -> Marker {
        let new_m = p.start();

        if let Event::StartNode {
            ref mut forward_parent,
            ..
        } = p.events[self.pos]
        {
            *forward_parent = Some(new_m.pos - self.pos);
        } else {
            unreachable!();
        }

        new_m
    }
}
