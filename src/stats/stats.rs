use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant},
};

use crossbeam_channel::Sender;

use crate::ui::{pixel::PixelBatchUpdate, text::render_string_at_position};

#[derive(Clone, Copy)]
struct ChunksData {
    total_chunks: u32,
    completed_chunks: u32,
}

#[derive(Clone, Copy)]
struct FrameData {
    start_time: Instant,
    end_time: Option<Instant>,
    samples_per_pixel: u32,
    chunks_data: ChunksData,
}

#[derive(Clone, Copy)]
struct FramesData {
    total_frames: u32,
    completed_frames: u32,
}

#[derive(Clone)]
pub struct StatsData {
    current_frame: FrameData,
    frames: FramesData,
    pixel_batch_sender: Sender<PixelBatchUpdate>,
}

#[derive(Clone)]
pub struct Stats {
    data: Arc<Mutex<StatsData>>,
}

fn create_new_frame(total_chunks: u32, samples_per_pixel: u32) -> FrameData {
    return FrameData {
        start_time: Instant::now(),
        end_time: None,
        samples_per_pixel,
        chunks_data: ChunksData {
            total_chunks: total_chunks,
            completed_chunks: 0,
        },
    };
}

fn init(stats: Stats) {
    let mut last_progress = 0.0;
    thread::spawn(move || loop {
        let data_clone = Arc::clone(&stats.data);

        let mut data: Option<StatsData> = None;

        match data_clone.try_lock() {
            Ok(stats) => {
                data = Some(stats.clone());
            }
            Err(_) => {}
        }

        match data {
            Some(data) => {
                let frame = data.current_frame;
                let chunks = frame.chunks_data;

                let current_frame_duration: Duration = match frame.end_time {
                    None => frame.start_time.elapsed(),
                    Some(end_time) => end_time.duration_since(frame.start_time),
                };

                let completed_chunks = chunks.completed_chunks;
                let progress = completed_chunks as f32 / chunks.total_chunks.max(1) as f32;
                let remaining_chunks = chunks.total_chunks - completed_chunks;
                let per_chunk = current_frame_duration / completed_chunks.max(1);
                let remaining_chunks_time = per_chunk * remaining_chunks as u32;

                let run_time_string = format!(
                    "Time:     \t{:03}.{:03}s\nRemaining:\t{:03}.{:03}s\nProgress: \t{:03.02}%",
                    current_frame_duration.as_secs(),
                    current_frame_duration.subsec_millis(),
                    remaining_chunks_time.as_secs(),
                    remaining_chunks_time.subsec_millis(),
                    progress * 100.0,
                );
                let settings_string = format!("Setting\n\tSpp: {}", frame.samples_per_pixel);
                let chunks_string = format!(
                    "Chunks:\t{:06}\n\tTime  : {}.{:03}ms ",
                    chunks.total_chunks,
                    per_chunk.as_millis(),
                    per_chunk.subsec_micros(),
                );
                let update_string = format!(
                    "{}\n{}\n{}",
                    run_time_string, settings_string, chunks_string
                );

                let pixels_update = render_string_at_position(10, 10, update_string);

                data.pixel_batch_sender.send(pixels_update).unwrap();

                if progress == last_progress {
                    continue;
                }

                last_progress = progress;

                print!(
                        "\rChunks: {}/{} {:2.2}% Chunk: {}.{:.3}ms Runtime: {}.{}s Remaining: {}.{}s                 ",
                        completed_chunks,
                        chunks.total_chunks,
                        progress * 100.0,
                        per_chunk.as_millis(),
                        per_chunk.subsec_micros(),
                        current_frame_duration.as_secs(),
                        current_frame_duration.subsec_millis(),
                        remaining_chunks_time.as_secs(),
                        remaining_chunks_time.subsec_millis()
                    );
            }
            _ => {}
        }

        thread::sleep(Duration::from_millis(100));
    });
}

impl Stats {
    pub fn new(pixel_batch_sender: Sender<PixelBatchUpdate>, total_frames: u32) -> Stats {
        return Stats {
            data: Arc::new(Mutex::new(StatsData {
                current_frame: create_new_frame(0, 0),
                frames: FramesData {
                    total_frames,
                    completed_frames: 0,
                },
                pixel_batch_sender,
            })),
        };
    }
    pub fn init(self) {
        init(self);
    }
    pub fn complete_chunk(self) {
        let mut data = self.data.lock().unwrap();

        data.current_frame.chunks_data.completed_chunks += 1;
    }
    pub fn start_current_frame(self, total_chunks: u32, samples_per_pixel: u32) {
        let mut data = self.data.lock().unwrap();
        data.current_frame = create_new_frame(total_chunks, samples_per_pixel);
    }
    pub fn complete_frame(self) {
        let mut data = self.data.lock().unwrap();
        data.current_frame.end_time = Some(Instant::now());
        data.frames.completed_frames += 1;
    }
}
