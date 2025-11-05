use anyhow::{anyhow, Result};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tracing::{debug, error, info};
// Symphonia imports removed since we're not using the full API yet

/// Audio player state
#[derive(Debug, Clone, PartialEq)]
pub enum AudioState {
    Stopped,
    Playing,
    Paused,
    Loading,
}

/// Audio player for handling music playback
pub struct AudioPlayer {
    commands: mpsc::Sender<Command>,
    current_track: Arc<Mutex<Option<String>>>,
    volume: Arc<Mutex<f32>>,
    state: Arc<Mutex<AudioState>>,
}

type CommandResultSender = SyncSender<Result<(), anyhow::Error>>;

enum Command {
    Play {
        path: PathBuf,
        respond_to: CommandResultSender,
    },
    Pause {
        respond_to: CommandResultSender,
    },
    Resume {
        respond_to: CommandResultSender,
    },
    Stop {
        respond_to: CommandResultSender,
    },
    SetVolume {
        volume: f32,
        respond_to: CommandResultSender,
    },
    Shutdown,
}

impl AudioPlayer {
    /// Create a new audio player
    pub fn new() -> Result<Self> {
        let (command_tx, command_rx) = mpsc::channel::<Command>();
        let current_track = Arc::new(Mutex::new(None));
        let volume = Arc::new(Mutex::new(0.7));
        let state = Arc::new(Mutex::new(AudioState::Stopped));

        let current_track_thread = Arc::clone(&current_track);
        let volume_thread = Arc::clone(&volume);
        let state_thread = Arc::clone(&state);

        let (init_tx, init_rx) = mpsc::sync_channel(1);

        thread::Builder::new()
            .name("hexendrum-audio".into())
            .spawn(move || match OutputStream::try_default() {
                Ok((stream, stream_handle)) => {
                    let _ = init_tx.send(Ok(()));
                    let mut sink: Option<Sink> = None;
                    let mut current_volume = *volume_thread.lock().unwrap();

                    run_command_loop(
                        command_rx,
                        stream,
                        stream_handle,
                        &mut sink,
                        &mut current_volume,
                        &state_thread,
                        &current_track_thread,
                        &volume_thread,
                    );
                }
                Err(e) => {
                    let _ = init_tx.send(Err(anyhow!(e)));
                }
            })?;

        match init_rx.recv() {
            Ok(Ok(())) => Ok(Self {
                commands: command_tx,
                current_track,
                volume,
                state,
            }),
            Ok(Err(err)) => Err(err),
            Err(e) => Err(anyhow!("Audio thread initialization failed: {}", e)),
        }
    }

    /// Play an audio file
    pub fn play(&self, file_path: &Path) -> Result<()> {
        debug!("Attempting to play {:?}", file_path);

        {
            let mut state_guard = self.state.lock().unwrap();
            *state_guard = AudioState::Loading;
        }

        let (resp_tx, resp_rx) = mpsc::sync_channel(1);
        self.commands
            .send(Command::Play {
                path: file_path.to_path_buf(),
                respond_to: resp_tx,
            })
            .map_err(|e| anyhow!("Failed to send play command: {}", e))?;

        match resp_rx.recv() {
            Ok(Ok(())) => {
                info!("Playback started: {}", file_path.display());
                Ok(())
            }
            Ok(Err(err)) => {
                error!(
                    "Failed to start playback for {}: {}",
                    file_path.display(),
                    err
                );
                Err(err)
            }
            Err(e) => Err(anyhow!("Playback thread disconnected: {}", e)),
        }
    }

    /// Pause playback
    pub fn pause(&self) -> Result<()> {
        let (resp_tx, resp_rx) = mpsc::sync_channel(1);
        self.commands
            .send(Command::Pause {
                respond_to: resp_tx,
            })
            .map_err(|e| anyhow!("Failed to send pause command: {}", e))?;

        match resp_rx.recv() {
            Ok(result) => result,
            Err(e) => Err(anyhow!("Playback thread disconnected: {}", e)),
        }
    }

    /// Resume playback
    pub fn resume(&self) -> Result<()> {
        let (resp_tx, resp_rx) = mpsc::sync_channel(1);
        self.commands
            .send(Command::Resume {
                respond_to: resp_tx,
            })
            .map_err(|e| anyhow!("Failed to send resume command: {}", e))?;

        match resp_rx.recv() {
            Ok(result) => result,
            Err(e) => Err(anyhow!("Playback thread disconnected: {}", e)),
        }
    }

    /// Stop playback
    pub fn stop(&self) -> Result<()> {
        let (resp_tx, resp_rx) = mpsc::sync_channel(1);
        self.commands
            .send(Command::Stop {
                respond_to: resp_tx,
            })
            .map_err(|e| anyhow!("Failed to send stop command: {}", e))?;

        match resp_rx.recv() {
            Ok(result) => result,
            Err(e) => Err(anyhow!("Playback thread disconnected: {}", e)),
        }
    }

    /// Set volume (0.0 to 1.0)
    pub fn set_volume(&self, volume: f32) -> Result<()> {
        let volume = volume.clamp(0.0, 1.0);

        let (resp_tx, resp_rx) = mpsc::sync_channel(1);
        self.commands
            .send(Command::SetVolume {
                volume,
                respond_to: resp_tx,
            })
            .map_err(|e| anyhow!("Failed to send volume command: {}", e))?;

        match resp_rx.recv() {
            Ok(result) => result,
            Err(e) => Err(anyhow!("Playback thread disconnected: {}", e)),
        }
    }

    /// Get current volume
    pub fn get_volume(&self) -> f32 {
        *self.volume.lock().unwrap()
    }

    /// Get current playback state
    pub fn get_state(&self) -> AudioState {
        self.state.lock().unwrap().clone()
    }

    /// Get current track path
    pub fn get_current_track(&self) -> Option<String> {
        self.current_track.lock().unwrap().clone()
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        let _ = self.commands.send(Command::Shutdown);
    }
}

fn run_command_loop(
    command_rx: Receiver<Command>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: &mut Option<Sink>,
    current_volume: &mut f32,
    state: &Arc<Mutex<AudioState>>,
    current_track: &Arc<Mutex<Option<String>>>,
    volume: &Arc<Mutex<f32>>,
) {
    for command in command_rx {
        match command {
            Command::Play { path, respond_to } => {
                handle_stop_internal(sink, state, current_track);
                {
                    let mut state_guard = state.lock().unwrap();
                    *state_guard = AudioState::Loading;
                }

                let result: Result<()> = (|| {
                    let file = File::open(&path)?;
                    let reader = BufReader::new(file);
                    let decoder = Decoder::new(reader)
                        .map_err(|e| anyhow!("Failed to decode audio file: {}", e))?;

                    let new_sink = Sink::try_new(&stream_handle)
                        .map_err(|e| anyhow!("Failed to create playback sink: {}", e))?;
                    new_sink.set_volume(*current_volume);
                    new_sink.append(decoder);
                    new_sink.play();

                    {
                        let mut track_guard = current_track.lock().unwrap();
                        *track_guard = Some(path.to_string_lossy().to_string());
                    }

                    {
                        let mut state_guard = state.lock().unwrap();
                        *state_guard = AudioState::Playing;
                    }

                    *sink = Some(new_sink);
                    Ok(())
                })();

                match result {
                    Ok(()) => {
                        let _ = respond_to.send(Ok(()));
                    }
                    Err(err) => {
                        {
                            let mut state_guard = state.lock().unwrap();
                            *state_guard = AudioState::Stopped;
                        }
                        let mut track_guard = current_track.lock().unwrap();
                        *track_guard = None;
                        let _ = respond_to.send(Err(err));
                    }
                }
            }
            Command::Pause { respond_to } => {
                if let Some(active_sink) = sink.as_ref() {
                    active_sink.pause();
                    let mut state_guard = state.lock().unwrap();
                    *state_guard = AudioState::Paused;
                    debug!("Playback paused");
                }
                let _ = respond_to.send(Ok(()));
            }
            Command::Resume { respond_to } => {
                if let Some(active_sink) = sink.as_ref() {
                    active_sink.play();
                    let mut state_guard = state.lock().unwrap();
                    *state_guard = AudioState::Playing;
                    debug!("Playback resumed");
                }
                let _ = respond_to.send(Ok(()));
            }
            Command::Stop { respond_to } => {
                handle_stop_internal(sink, state, current_track);
                let _ = respond_to.send(Ok(()));
            }
            Command::SetVolume {
                volume: new_volume,
                respond_to,
            } => {
                *current_volume = new_volume;
                {
                    let mut volume_guard = volume.lock().unwrap();
                    *volume_guard = new_volume;
                }
                if let Some(active_sink) = sink.as_ref() {
                    active_sink.set_volume(new_volume);
                }
                let _ = respond_to.send(Ok(()));
            }
            Command::Shutdown => {
                handle_stop_internal(sink, state, current_track);
                break;
            }
        }
    }
}

fn handle_stop_internal(
    sink: &mut Option<Sink>,
    state: &Arc<Mutex<AudioState>>,
    current_track: &Arc<Mutex<Option<String>>>,
) {
    if let Some(active_sink) = sink.take() {
        active_sink.stop();
        debug!("Playback stopped");
    }

    {
        let mut track_guard = current_track.lock().unwrap();
        *track_guard = None;
    }

    {
        let mut state_guard = state.lock().unwrap();
        *state_guard = AudioState::Stopped;
    }
}

/// Get audio file duration
pub fn get_audio_duration(file_path: &Path) -> Result<Duration> {
    use symphonia::core::{
        codecs::DecoderOptions, errors::Error as SymphoniaError, formats::FormatOptions,
        io::MediaSourceStream, meta::MetadataOptions, probe::Hint,
    };

    let reader = File::open(file_path)?;
    let mss = MediaSourceStream::new(Box::new(reader), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = file_path.extension().and_then(|ext| ext.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe().format(
        &hint,
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    )?;

    let mut format = probed.format;
    let track = format
        .default_track()
        .ok_or_else(|| anyhow!("No default audio track found"))?;
    let codec_params = track.codec_params.clone();
    let track_id = track.id;

    if let (Some(n_frames), Some(sample_rate)) = (codec_params.n_frames, codec_params.sample_rate) {
        let seconds = n_frames as f64 / sample_rate as f64;
        return Ok(Duration::from_secs_f64(seconds));
    }

    if let Some(sample_rate) = codec_params.sample_rate {
        let mut decoder =
            symphonia::default::get_codecs().make(&codec_params, &DecoderOptions::default())?;
        let mut total_frames: u64 = 0;

        loop {
            match format.next_packet() {
                Ok(packet) => {
                    if packet.track_id() != track_id {
                        continue;
                    }

                    match decoder.decode(&packet) {
                        Ok(decoded) => {
                            total_frames = total_frames.saturating_add(decoded.frames() as u64);
                        }
                        Err(SymphoniaError::DecodeError(_)) => continue,
                        Err(SymphoniaError::IoError(_)) => break,
                        Err(SymphoniaError::ResetRequired) => {
                            decoder = symphonia::default::get_codecs()
                                .make(&codec_params, &DecoderOptions::default())?;
                        }
                        Err(err) => return Err(anyhow!(err)),
                    }
                }
                Err(SymphoniaError::IoError(_)) => break,
                Err(SymphoniaError::ResetRequired) => {
                    decoder = symphonia::default::get_codecs()
                        .make(&codec_params, &DecoderOptions::default())?;
                }
                Err(err) => return Err(anyhow!(err)),
            }
        }

        if total_frames > 0 {
            let seconds = total_frames as f64 / sample_rate as f64;
            return Ok(Duration::from_secs_f64(seconds));
        }
    }

    Ok(Duration::from_secs(0))
}

/// Check if a file is a supported audio format
pub fn is_supported_audio_format(file_path: &Path) -> bool {
    if let Some(extension) = file_path.extension() {
        if let Some(ext_str) = extension.to_str() {
            let supported = ["mp3", "flac", "ogg", "wav", "m4a", "aac"];
            return supported.contains(&ext_str.to_lowercase().as_str());
        }
    }
    false
}

/// Initialize the audio system
pub async fn init() -> Result<()> {
    // Initialize the default audio output stream
    let (_stream, _stream_handle) = OutputStream::try_default()?;

    info!("Audio system initialized successfully");
    Ok(())
}
