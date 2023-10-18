use crate::{config, gen_loop, nn};
use crate::logic::{get_at, utils};
use crate::models::action::Action;
use crate::models::agent::Agent;
use crate::models::gen::Gen;

fn resolve_action_for(agent_i: usize, gen: &mut Gen) -> Action {
    let result = nn::get::get_action_for_agent(agent_i, Action::ActionCount as i32, &*gen);

    Action::from(result)
}

fn step_agent(agent_i: usize, gen: &mut Gen) {
    let action = resolve_action_for(agent_i, gen);

    let mut agent = gen.agents[agent_i].clone();

    if agent.is_dead {
        return;
    }

    agent.energy -= config::AGENT_ENERGY_LOSS;

    if agent.energy <= 0 {
        agent.is_dead = true;
        gen.agents[agent_i] = agent;
        return;
    }

    agent.action = action;
    agent.score += 1;

    execute_action(action, &mut agent, gen);

    gen.agents[agent_i] = agent;
}

fn execute_action(action: Action, agent: &mut Agent, gen: &mut Gen) {
    match action {
        Action::MoveForward => {
            let front = agent.get_front();
            let agent_option = get_at::get_agent_at(&front, gen);
            if agent_option.is_some() {
                if !gen.agents[agent_option.unwrap()].is_dead {
                    return;
                }
            }
            let wall = get_at::get_wall_at(&front, gen).is_some();
            if wall {
                return;
            }

            if utils::is_out_of_bounds(&front) {
                return;
            }

            agent.pos = front;
        }
        Action::TurnLeft => {
            agent.dir = agent.dir.turn_left();
        }
        Action::TurnRight => {
            agent.dir = agent.dir.turn_right();
        }
        Action::Eat => {
            let food = get_at::get_food_at(&agent.pos, gen);
            match food {
                None => {}
                Some(food) => {
                    gen.food.remove(food);
                    agent.energy += config::AGENT_ENERGY_RESTORE;
                }
            }
        }
        Action::AttackAgentInFront => {
            let agent_in_front = get_at::get_agent_at(&agent.get_front(), gen);
            match agent_in_front {
                None => {}
                Some(agent) => {
                    gen.agents[agent].energy -= config::AGENT_ENERGY_LOSS;
                }
            }
        }
        Action::None => {}
        Action::ActionCount => {
            panic!("Invalid action");
        }
    }
}

pub fn step_gen(gen: &mut Gen) {
    for agent in 0..gen.agents.len() {
        step_agent(agent, gen);
    }
}
