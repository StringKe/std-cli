use crate::CliError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CapturePixelEvidence {
    pub(crate) samples: u32,
    pub(crate) opaque_samples: u32,
    pub(crate) unique_colors: u32,
    pub(crate) black_pixels: u32,
    pub(crate) white_pixels: u32,
    pub(crate) transparent_pixels: u32,
    pub(crate) edge_samples: u32,
    pub(crate) edge_transparent_pixels: u32,
    pub(crate) edge_black_pixels: u32,
    pub(crate) edge_white_pixels: u32,
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
    if evidence.opaque_samples < 5 {
        return Err(CliError::Doctor(format!(
            "capture opaque pixel evidence too weak for {surface} {theme} {scenario}: opaque_samples={}",
            evidence.opaque_samples
        )));
    }
    if evidence.opaque_samples + evidence.transparent_pixels > evidence.samples {
        return Err(CliError::Doctor(format!(
            "capture pixel evidence count mismatch for {surface} {theme} {scenario}"
        )));
    }
    verify_edge_evidence(surface, theme, scenario, evidence)?;
    if evidence.unique_colors < 2 {
        return Err(CliError::Doctor(format!(
            "capture appears to be a single-color host carrier for {surface} {theme} {scenario}"
        )));
    }
    if dominant_carrier_pixels(evidence.black_pixels, evidence.opaque_samples) {
        return Err(CliError::Doctor(format!(
            "capture appears to be dominant black host background for {surface} {theme} {scenario}"
        )));
    }
    if dominant_carrier_pixels(evidence.white_pixels, evidence.opaque_samples) {
        return Err(CliError::Doctor(format!(
            "capture appears to be dominant white host background for {surface} {theme} {scenario}"
        )));
    }
    Ok(())
}

fn verify_edge_evidence(
    surface: &str,
    theme: &str,
    scenario: &str,
    evidence: &CapturePixelEvidence,
) -> Result<(), CliError> {
    if evidence.edge_samples < 8 {
        return Err(CliError::Doctor(format!(
            "capture edge pixel evidence too weak for {surface} {theme} {scenario}: edge_samples={}",
            evidence.edge_samples
        )));
    }
    if evidence.edge_black_pixels > 0 {
        return Err(CliError::Doctor(format!(
            "capture edge includes black host carrier for {surface} {theme} {scenario}"
        )));
    }
    if evidence.edge_white_pixels > 0 {
        return Err(CliError::Doctor(format!(
            "capture edge includes white host carrier for {surface} {theme} {scenario}"
        )));
    }
    if surface == "launcher" && evidence.edge_transparent_pixels < 6 {
        return Err(CliError::Doctor(format!(
            "launcher capture edge must prove transparent host gutter for {theme} {scenario}: edge_transparent_pixels={}",
            evidence.edge_transparent_pixels
        )));
    }
    if surface == "studio" && evidence.edge_transparent_pixels > 0 {
        return Err(CliError::Doctor(format!(
            "studio capture edge must be opaque single host viewport for {theme} {scenario}"
        )));
    }
    Ok(())
}

fn dominant_carrier_pixels(count: u32, samples: u32) -> bool {
    count.saturating_mul(3) >= samples.saturating_mul(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pixel_evidence_accepts_non_carrier_capture() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 7,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        verify_pixel_evidence("launcher", "dark", "results", &evidence).unwrap();
    }

    #[test]
    fn pixel_evidence_rejects_single_color_capture() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 1,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 7,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        let error = verify_pixel_evidence("launcher", "dark", "results", &evidence).unwrap_err();

        assert!(error.to_string().contains("single-color host carrier"));
    }

    #[test]
    fn pixel_evidence_rejects_dominant_black_or_white_capture() {
        let black = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 2,
            black_pixels: 7,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 7,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };
        let white = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 2,
            black_pixels: 0,
            white_pixels: 7,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 7,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        assert!(verify_pixel_evidence("launcher", "dark", "results", &black)
            .unwrap_err()
            .to_string()
            .contains("dominant black"));
        assert!(
            verify_pixel_evidence("launcher", "light", "results", &white)
                .unwrap_err()
                .to_string()
                .contains("dominant white")
        );
    }

    #[test]
    fn pixel_evidence_accepts_non_dominant_black_or_white_samples() {
        let dark = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 4,
            black_pixels: 5,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 0,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };
        let light = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 4,
            black_pixels: 0,
            white_pixels: 5,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 0,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        verify_pixel_evidence("studio", "dark", "dashboard", &dark).unwrap();
        verify_pixel_evidence("studio", "light", "dashboard", &light).unwrap();
    }

    #[test]
    fn pixel_evidence_ignores_transparent_host_gutter_samples() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 6,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 3,
            edge_samples: 8,
            edge_transparent_pixels: 8,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        verify_pixel_evidence("launcher", "dark", "collapsed", &evidence).unwrap();
    }

    #[test]
    fn pixel_evidence_uses_opaque_samples_for_carrier_ratio() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 6,
            unique_colors: 3,
            black_pixels: 4,
            white_pixels: 0,
            transparent_pixels: 3,
            edge_samples: 8,
            edge_transparent_pixels: 8,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        let error = verify_pixel_evidence("launcher", "dark", "collapsed", &evidence).unwrap_err();

        assert!(error.to_string().contains("dominant black"));
    }

    #[test]
    fn pixel_evidence_rejects_mostly_transparent_capture_without_panel_samples() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 4,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 5,
            edge_samples: 8,
            edge_transparent_pixels: 8,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        let error = verify_pixel_evidence("launcher", "dark", "collapsed", &evidence).unwrap_err();

        assert!(error.to_string().contains("opaque pixel evidence too weak"));
    }

    #[test]
    fn pixel_evidence_rejects_launcher_without_transparent_edge_gutter() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 0,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        let error = verify_pixel_evidence("launcher", "dark", "results", &evidence).unwrap_err();

        assert!(error.to_string().contains("transparent host gutter"));
    }

    #[test]
    fn pixel_evidence_rejects_black_or_white_capture_edge() {
        let mut evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 7,
            edge_black_pixels: 1,
            edge_white_pixels: 0,
        };

        assert!(
            verify_pixel_evidence("launcher", "dark", "results", &evidence)
                .unwrap_err()
                .to_string()
                .contains("black host carrier")
        );

        evidence.edge_black_pixels = 0;
        evidence.edge_white_pixels = 1;
        assert!(
            verify_pixel_evidence("launcher", "light", "results", &evidence)
                .unwrap_err()
                .to_string()
                .contains("white host carrier")
        );
    }

    #[test]
    fn pixel_evidence_rejects_transparent_studio_edge() {
        let evidence = CapturePixelEvidence {
            samples: 9,
            opaque_samples: 9,
            unique_colors: 3,
            black_pixels: 0,
            white_pixels: 0,
            transparent_pixels: 0,
            edge_samples: 8,
            edge_transparent_pixels: 1,
            edge_black_pixels: 0,
            edge_white_pixels: 0,
        };

        let error = verify_pixel_evidence("studio", "dark", "dashboard", &evidence).unwrap_err();

        assert!(error.to_string().contains("opaque single host viewport"));
    }
}
