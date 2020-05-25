/*
The MIT License (MIT)

Copyright (c) 2016-2017 ggez-dev

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
 */

use std::cmp;
use std::f64;
use std::thread;
use std::time;

#[derive(Debug, Clone)]
struct LogBuffer<T>
    where
        T: Clone,
{
    head: usize,
    size: usize,
    /// The number of actual samples inserted, used for
    /// smarter averaging.
    samples: usize,
    contents: Vec<T>,
}

impl<T> LogBuffer<T>
    where
        T: Clone + Copy,
{
    fn new(size: usize, init_val: T) -> LogBuffer<T> {
        LogBuffer {
            head: 0,
            size,
            contents: vec![init_val; size],
            // Never divide by 0
            samples: 1,
        }
    }

    fn push(&mut self, item: T) {
        self.head = (self.head + 1) % self.contents.len();
        self.contents[self.head] = item;
        self.size = cmp::min(self.size + 1, self.contents.len());
        self.samples += 1;
    }

    fn contents(&self) -> &[T] {
        if self.samples > self.size {
            &self.contents
        } else {
            &self.contents[..self.samples]
        }
    }

    fn latest(&self) -> T {
        self.contents[self.head]
    }
}


#[derive(Debug)]
pub struct TimeContext {
    init_instant: time::Instant,
    last_instant: time::Instant,
    frame_durations: LogBuffer<time::Duration>,
    residual_update_dt: time::Duration,
    frame_count: usize,
}

// How many frames we log update times for.
const TIME_LOG_FRAMES: usize = 200;

impl TimeContext {
    pub fn new() -> TimeContext {
        let initial_dt = time::Duration::from_millis(16);
        TimeContext {
            init_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_durations: LogBuffer::new(TIME_LOG_FRAMES, initial_dt),
            residual_update_dt: time::Duration::from_secs(0),
            frame_count: 0,
        }
    }

    // notify frame updation to TimerContext.

    pub fn tick(&mut self) {
        let now = time::Instant::now();
        let time_since_last = now - self.last_instant;
        self.frame_durations.push(time_since_last);
        self.last_instant = now;
        self.frame_count += 1;

        self.residual_update_dt += time_since_last;
    }
}

impl Default for TimeContext {
    fn default() -> Self {
        Self::new()
    }
}


pub fn delta(tc: &TimeContext) -> time::Duration {
    tc.frame_durations.latest()
}


pub fn average_delta(tc: &TimeContext) -> time::Duration {
    let sum: time::Duration = tc.frame_durations.contents().iter().sum();
    // If our buffer is actually full, divide by its size.
    // Otherwise divide by the number of samples we've added
    if tc.frame_durations.samples > tc.frame_durations.size {
        sum / (tc.frame_durations.size as u32)
    } else {
        sum / (tc.frame_durations.samples as u32)
    }
}


pub fn duration_to_f64(d: time::Duration) -> f64 {
    let seconds = d.as_secs() as f64;
    let nanos = f64::from(d.subsec_nanos());
    seconds + (nanos * 1e-9)
}


pub fn f64_to_duration(t: f64) -> time::Duration {
    debug_assert!(t > 0.0, "f64_to_duration passed a negative number!");
    let seconds = t.trunc();
    let nanos = t.fract() * 1e9;
    time::Duration::new(seconds as u64, nanos as u32)
}


fn fps_as_duration(fps: u32) -> time::Duration {
    let target_dt_seconds = 1.0 / f64::from(fps);
    f64_to_duration(target_dt_seconds)
}


pub fn fps(tc: &TimeContext) -> f64 {
    let duration_per_frame = average_delta(tc);
    let seconds_per_frame = duration_to_f64(duration_per_frame);
    1.0 / seconds_per_frame
}


pub fn time_since_start(tc: &TimeContext) -> time::Duration {
    time::Instant::now() - tc.init_instant
}


pub fn check_update_time(timedata: &mut TimeContext, target_fps: u32) -> bool {
    let target_dt = fps_as_duration(target_fps);
    if timedata.residual_update_dt > target_dt {
        timedata.residual_update_dt -= target_dt;
        true
    } else {
        false
    }
}

pub fn remaining_update_time(ctx: &mut TimeContext) -> time::Duration {
    ctx.residual_update_dt
}

pub fn sleep(duration: time::Duration) {
    thread::sleep(duration);
}

pub fn yield_now() {
    thread::yield_now();
}

pub fn ticks(ctx: &TimeContext) -> usize {
    ctx.frame_count
}

pub struct TimeContextGroup {
    tc_vec: Vec<TimeContext>,

}

impl TimeContextGroup {
    pub fn new() -> TimeContextGroup {
        TimeContextGroup {
            tc_vec: vec![]
        }
    }

    pub fn tick_all(&mut self) {
        for index in 0..self.tc_vec.len() {
            self.tc_vec[index].tick()
        }
    }

    pub fn get(&mut self, index: usize) -> &mut TimeContext {
        let diff = (index + 1) as i32 - (self.tc_vec.len() as i32);

        if diff > 0 {
            let mut i = 0;
            while i < diff {
                self.tc_vec.push(TimeContext::new());
                i += 1;
            }
        }

        &mut self.tc_vec[index]
    }
}