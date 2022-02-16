pub type Sender = std::sync::mpsc::Sender<Type>;

pub enum Type {
    EndBreakTimer,
    EndNotifier,
    EndRest,
}

impl From<Type> for &'static [u8] {
    fn from(sound_type: Type) -> Self {
        match sound_type {
            Type::EndBreakTimer => ALERT_CLEAR_ANNOUNCE_TONES,
            Type::EndRest => ALERT_BELLS_ECHO,
            Type::EndNotifier => ALERT_QUICK_CHIME,
        }
    }
}

const ALERT_BELLS_ECHO: &[u8] =
    include_bytes!("../resources/sounds/mixkit-alert-bells-echo-765.wav");
const ALERT_QUICK_CHIME: &[u8] =
    include_bytes!("../resources/sounds/mixkit-alert-quick-chime-766.wav");
const ALERT_CLEAR_ANNOUNCE_TONES: &[u8] =
    include_bytes!("../resources/sounds/mixkit-clear-announce-tones-2861.wav");

pub fn try_play(bytes: &'static [u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
    let sink = stream_handle.play_once(std::io::Cursor::new(bytes))?;
    sink.sleep_until_end();
    Ok(())
}
