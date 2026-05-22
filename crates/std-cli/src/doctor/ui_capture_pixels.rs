use crate::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapturePixelEvidence {
    pub(crate) samples: u32,
    pub(crate) unique_colors: u32,
    pub(crate) black_pixels: u32,
    pub(crate) white_pixels: u32,
}

pub(crate) fn verify_pixel_evidence(
    surface: &str,
    theme: &str,
    scenario: &str,
    evidence: &CapturePixelEvidence,
) -> Result<(), CliError> {
    if evidence.samples < 9 {
        return Err(CliError::Doctor(format!(
            "capture pixel evidence too weak for {surface} {theme} {scenario}: samples={}",
            evidence.samples
        )));
    }
    if evidence.unique_colors < 2 {
        return Err(CliError::Doctor(format!(
            "capture appears to be a single-color host carrier for {surface} {theme} {scenario}"
        )));
    }
    if evidence.black_pixels == evidence.samples {
        return Err(CliError::Doctor(format!(
            "capture appears to be all black host background for {surface} {theme} {scenario}"
        )));
    }
    if evidence.white_pixels == evidence.samples {
        return Err(CliError::Doctor(format!(
            "capture appears to be all white host background for {surface} {theme} {scenario}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_evidence_accepts_non_carrier_capture() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
        };

        verify_pixel_evidence("launcher", "dark", "results", &evidence).unwrap();
    }

    #[test]
    fn pixel_evidence_rejects_single_color_capture() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            unique_colors: 1,
            black_pixels: 0,
            white_pixels: 0,
        };

        let error = verify_pixel_evidence("launcher", "dark", "results", &evidence).unwrap_err();

        assert!(error.to_string().contains("single-color host carrier"));
    }

    #[test]
    fn pixel_evidence_rejects_all_black_or_white_capture() {
        let black = CapturePixelEvidence {
            samples: 9,
            unique_colors: 2,
            black_pixels: 9,
            white_pixels: 0,
        };
        let white = CapturePixelEvidence {
            samples: 9,
            unique_colors: 2,
            black_pixels: 0,
            white_pixels: 9,
        };

        assert!(verify_pixel_evidence("launcher", "dark", "results", &black)
            .unwrap_err()
            .to_string()
            .contains("all black"));
        assert!(
            verify_pixel_evidence("launcher", "light", "results", &white)
                .unwrap_err()
                .to_string()
                .contains("all white")
        );
    }
}
