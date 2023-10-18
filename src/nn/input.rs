pub struct Input {
    pub agent_pos: (i32, i32),
    pub agent_dir: i32,

    pub foods_pos_in_view: Vec<(i32, i32)>,

    pub agents_pos_in_view: Vec<(i32, i32)>,
    pub agents_dir_in_view: Vec<i32>,

    pub walls_pos_in_view: Vec<(i32, i32)>,

    pub actions_count: i32,
}