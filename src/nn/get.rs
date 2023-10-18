use rand::Rng;
use crate::logic;
use crate::models::gen::Gen;
use crate::nn::input::Input;
use crate::nn::network;

pub fn get_action_for_agent(agent: usize, actions_count: i32, gen: &Gen) -> i32 {
    let agent = &gen.agents[agent];

    let agents_pos_in_view = logic::get_at::get_all_agents_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    let agents_dir_in_view = agents_pos_in_view.iter().map(|a| a.dir as i32).collect();
    let agents_pos_in_view = agents_pos_in_view.iter().map(|a| (a.pos.x, a.pos.y)).collect();

    let foods_pos_in_view = logic::get_at::get_all_food_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    let foods_pos_in_view = foods_pos_in_view.iter().map(|f| (f.pos.x, f.pos.y)).collect();

    let walls_pos_in_view = logic::get_at::get_all_walls_in_view(agent.pos, agent.dir, agent.view_distance, gen);
    let walls_pos_in_view = walls_pos_in_view.iter().map(|w| (w.x, w.y)).collect();

    let input = Input {
        agent_pos: (agent.pos.x, agent.pos.y),
        agent_dir: agent.dir as i32,
        foods_pos_in_view,
        agents_pos_in_view,
        agents_dir_in_view,
        walls_pos_in_view,
        actions_count,
    };

    let output = network::nn(&input);

    output.action
}
