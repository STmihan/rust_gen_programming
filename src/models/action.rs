use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy)]
pub enum Action {
    None = 0,
    MoveForward,
    TurnLeft,
    TurnRight,
    Eat,
    AttackAgentInFront,

    ActionCount,
}

impl From<i32> for Action {
    fn from(action: i32) -> Self {
        match action {
            0 => Action::None,
            1 => Action::MoveForward,
            2 => Action::TurnLeft,
            3 => Action::TurnRight,
            4 => Action::Eat,
            5 => Action::AttackAgentInFront,

            _ => {
                panic!("Invalid action");
            }
        }
    }
}

impl Debug for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let action = match self {
            Action::None => "None",
            Action::MoveForward => "Move forward",
            Action::TurnLeft => "Turn left",
            Action::TurnRight => "Turn right",
            Action::Eat => "Eat",
            Action::AttackAgentInFront => "Attack agent in front of me",

            _ => {
                panic!("Invalid action");
            }
        };
        write!(f, "{}", action)
    }
}
