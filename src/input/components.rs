use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Facing {
    pub direction: Vec2,
}

impl Default for Facing {
    fn default() -> Self {
        Self {
            direction: Vec2::new(0.0, -1.0),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionState {
    #[default]
    Idle,
    Moving,
    Dashing,
}

#[derive(Component, Debug, Clone)]
pub struct DashState {
    pub elapsed_ticks: u32,
    pub direction: Vec2,
    pub charges: u32,
    pub cooldown_remaining: u32,
    pub invulnerable: bool,
}

impl DashState {
    pub fn new(max_charges: u32) -> Self {
        Self {
            elapsed_ticks: 0,
            direction: Vec2::ZERO,
            charges: max_charges,
            cooldown_remaining: 0,
            invulnerable: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct InputSnapshot {
    pub move_direction: Vec2,
    pub dash_requested: bool,
    pub tick: u64,
}

const BUFFER_CAPACITY: usize = 8;

#[derive(Component, Debug)]
pub struct InputBuffer {
    pub buffer: [InputSnapshot; BUFFER_CAPACITY],
    pub write_index: usize,
    pub last_consumed_tick: u64,
}

impl Default for InputBuffer {
    fn default() -> Self {
        Self {
            buffer: [InputSnapshot::default(); BUFFER_CAPACITY],
            write_index: 0,
            last_consumed_tick: 0,
        }
    }
}

impl InputBuffer {
    pub fn push(&mut self, snapshot: InputSnapshot) {
        self.buffer[self.write_index % BUFFER_CAPACITY] = snapshot;
        self.write_index += 1;
    }

    pub fn iter_recent(&self, since_tick: u64) -> impl Iterator<Item = &InputSnapshot> {
        let start = self.write_index.saturating_sub(BUFFER_CAPACITY);
        (start..self.write_index)
            .map(|i| &self.buffer[i % BUFFER_CAPACITY])
            .filter(move |s| s.tick > since_tick)
    }

    pub fn latest(&self) -> Option<&InputSnapshot> {
        if self.write_index == 0 {
            return None;
        }
        Some(&self.buffer[(self.write_index - 1) % BUFFER_CAPACITY])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_buffer_push_and_wrap() {
        let mut buf = InputBuffer::default();
        for i in 0..10u64 {
            buf.push(InputSnapshot {
                tick: i + 1,
                ..default()
            });
        }
        assert_eq!(buf.write_index, 10);
        let latest = buf.latest().unwrap();
        assert_eq!(latest.tick, 10);
        let oldest_visible = buf.buffer[10 % BUFFER_CAPACITY];
        assert_eq!(oldest_visible.tick, 3);
    }

    #[test]
    fn input_buffer_iter_recent_filters_by_tick() {
        let mut buf = InputBuffer::default();
        for i in 1..=5u64 {
            buf.push(InputSnapshot {
                tick: i,
                dash_requested: i == 3 || i == 5,
                ..default()
            });
        }
        let recent: Vec<_> = buf.iter_recent(2).collect();
        assert_eq!(recent.len(), 3);
        assert_eq!(recent[0].tick, 3);
        assert_eq!(recent[2].tick, 5);
    }

    #[test]
    fn input_buffer_latest_when_empty() {
        let buf = InputBuffer::default();
        assert!(buf.latest().is_none());
    }
}
