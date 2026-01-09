use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct FpsLimiter {
    target_frametime: Duration,
    next_frame_timestamp: Instant,
    poll_interval_sec: Duration,
    spin_reserve_sec: Duration,
}

impl FpsLimiter {
    pub fn new(fps_limit: u32, poll_interval_sec: f32, spin_reserve_sec: f32) -> Self {
        let target_frametime: Duration = calc_target_frametime(fps_limit as f32);

        Self {
            target_frametime,
            next_frame_timestamp: Instant::now() + target_frametime,
            poll_interval_sec: Duration::from_secs_f32(poll_interval_sec),
            spin_reserve_sec: Duration::from_secs_f32(spin_reserve_sec),
        }
    }
}

pub fn limit_fps(fps_limiter: &mut FpsLimiter, value: u32) {
    let target_frametime: Duration = calc_target_frametime(value as f32);

    fps_limiter.target_frametime = target_frametime;
    fps_limiter.next_frame_timestamp = Instant::now() + target_frametime;
}

pub fn wait_for_next_frame(fps_limiter: &mut FpsLimiter) -> f32 {
    if fps_limiter.target_frametime == Duration::ZERO {
        let delta_time: f32 = calc_delta_time(fps_limiter.next_frame_timestamp, Duration::ZERO);
        fps_limiter.next_frame_timestamp = Instant::now();
        return delta_time;
    }

    // Sleep until close to target
    while Instant::now() + fps_limiter.spin_reserve_sec < fps_limiter.next_frame_timestamp {
        let remaining: Duration =
            fps_limiter.next_frame_timestamp - Instant::now() - fps_limiter.spin_reserve_sec;
        sleep(fps_limiter.poll_interval_sec.min(remaining));
    }

    // Busy wait at the end for precision
    while Instant::now() < fps_limiter.next_frame_timestamp {
        std::hint::spin_loop();
    }

    let delta_time: f32 = calc_delta_time(
        fps_limiter.next_frame_timestamp,
        fps_limiter.target_frametime,
    );

    let frame_is_late: bool = Instant::now() > fps_limiter.next_frame_timestamp;
    fps_limiter.next_frame_timestamp = if frame_is_late {
        Instant::now() + fps_limiter.target_frametime
    } else {
        fps_limiter.next_frame_timestamp + fps_limiter.target_frametime
    };

    delta_time
}

/// This function should never, receive a negative `target_fps`,
/// as the module's public access for fps limit numbers uses u32.
fn calc_target_frametime(target_fps: f32) -> Duration {
    let fps_is_uncapped: bool = target_fps == 0.0;

    if fps_is_uncapped {
        Duration::ZERO
    } else {
        Duration::from_secs_f32(1.0 / target_fps)
    }
}

fn calc_delta_time(next_frame_timestamp: Instant, target_frametime: Duration) -> f32 {
    let now = Instant::now();
    let prev_frame = if target_frametime == Duration::ZERO {
        // uncapped FPS: measure delta since last timestamp
        next_frame_timestamp
    } else {
        next_frame_timestamp
            .checked_sub(target_frametime)
            .unwrap_or(next_frame_timestamp)
    };
    now.checked_duration_since(prev_frame)
        .unwrap_or(Duration::ZERO)
        .as_secs_f32()
}
