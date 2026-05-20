use crate::LauncherState;
use std_types::{ActionExecution, ActionPreview};

impl LauncherState {
    pub fn start_voice_input(&mut self) {
        self.controller.start_voice_input();
    }

    pub fn apply_voice_transcript(&mut self, transcript: impl AsRef<str>) -> Option<ActionPreview> {
        let query = clean_voice_transcript(transcript.as_ref());
        self.controller.finish_voice_input();
        self.update_query(query)
    }

    pub fn trigger_voice_transcript(
        &mut self,
        transcript: impl AsRef<str>,
    ) -> Option<ActionExecution> {
        self.apply_voice_transcript(transcript)?;
        self.trigger_selected()
    }
}

pub fn clean_voice_transcript(transcript: &str) -> String {
    let normalized = transcript
        .split_whitespace()
        .map(|part| {
            part.trim_matches(|ch: char| {
                matches!(
                    ch,
                    ',' | '.' | '?' | '!' | ':' | ';' | '"' | '\'' | '(' | ')' | '[' | ']'
                )
            })
        })
        .filter(|part| !part.is_empty())
        .filter(|part| !is_filler_word(part))
        .collect::<Vec<_>>();
    normalized.join(" ")
}

fn is_filler_word(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "um" | "uh"
            | "erm"
            | "ah"
            | "like"
            | "actually"
            | "basically"
            | "please"
            | "just"
            | "嗯"
            | "呃"
            | "那个"
            | "就是"
            | "请"
            | "帮我"
    )
}
