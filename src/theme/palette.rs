use egui::Color32;

pub struct Palette {
    pub accent: Color32,
    pub accent_soft: Color32,
    pub success: Color32,
    pub danger: Color32,
    pub surface_0: Color32,
    pub surface_1: Color32,
    pub surface_2: Color32,
    pub surface_3: Color32,
    pub border_soft: Color32,
    pub text: Color32,
    pub text_muted: Color32,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            accent: Color32::from_rgb(59, 130, 246),
            accent_soft: Color32::from_rgb(147, 197, 253),
            success: Color32::from_rgb(34, 197, 94),
            danger: Color32::from_rgb(239, 68, 68),
            surface_0: Color32::from_rgb(30, 32, 39),
            surface_1: Color32::from_rgb(24, 26, 33),
            surface_2: Color32::from_rgb(38, 40, 48),
            surface_3: Color32::from_rgb(52, 55, 64),
            border_soft: Color32::from_rgba_premultiplied(255, 255, 255, 18),
            text: Color32::from_rgb(236, 239, 244),
            text_muted: Color32::from_rgb(180, 185, 193),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palette_default() {
        let palette = Palette::default();

        assert_eq!(palette.accent, Color32::from_rgb(59, 130, 246));
        assert_eq!(palette.accent_soft, Color32::from_rgb(147, 197, 253));
        assert_eq!(palette.success, Color32::from_rgb(34, 197, 94));
        assert_eq!(palette.danger, Color32::from_rgb(239, 68, 68));
        assert_eq!(palette.surface_0, Color32::from_rgb(30, 32, 39));
        assert_eq!(palette.surface_1, Color32::from_rgb(24, 26, 33));
        assert_eq!(palette.surface_2, Color32::from_rgb(38, 40, 48));
        assert_eq!(palette.surface_3, Color32::from_rgb(52, 55, 64));
        assert_eq!(
            palette.border_soft,
            Color32::from_rgba_premultiplied(255, 255, 255, 18)
        );
        assert_eq!(palette.text, Color32::from_rgb(236, 239, 244));
        assert_eq!(palette.text_muted, Color32::from_rgb(180, 185, 193));
    }

    #[test]
    fn test_palette_color_properties() {
        let palette = Palette::default();

        assert!(palette.accent.r() > 0);
        assert!(palette.accent.g() > 0);
        assert!(palette.accent.b() > 0);

        assert_ne!(palette.success, palette.danger);

        assert_ne!(palette.text, palette.text_muted);

        assert_ne!(palette.surface_0, palette.surface_1);
        assert_ne!(palette.surface_1, palette.surface_2);
        assert_ne!(palette.surface_2, palette.surface_3);
    }

    #[test]
    fn test_palette_dark_theme_properties() {
        let palette = Palette::default();

        assert!(palette.surface_0.r() < 100);
        assert!(palette.surface_0.g() < 100);
        assert!(palette.surface_0.b() < 100);

        assert!(palette.surface_1.r() < 100);
        assert!(palette.surface_1.g() < 100);
        assert!(palette.surface_1.b() < 100);

        assert!(palette.text.r() > 200);
        assert!(palette.text.g() > 200);
        assert!(palette.text.b() > 200);
    }

    #[test]
    fn test_palette_semantic_colors() {
        let palette = Palette::default();

        assert!(palette.success.g() > palette.success.r());
        assert!(palette.success.g() > palette.success.b());

        assert!(palette.danger.r() > palette.danger.g());
        assert!(palette.danger.r() > palette.danger.b());

        assert!(palette.accent.b() > palette.accent.r());
        assert!(palette.accent.b() > palette.accent.g());
    }

    #[test]
    fn test_palette_contrast() {
        let palette = Palette::default();

        assert!(palette.text.r() > palette.surface_0.r());
        assert!(palette.text.g() > palette.surface_0.g());
        assert!(palette.text.b() > palette.surface_0.b());

        assert!(palette.text.r() > palette.text_muted.r());
        assert!(palette.text.g() > palette.text_muted.g());
        assert!(palette.text.b() > palette.text_muted.b());
    }

    #[test]
    fn test_palette_consistency() {
        let palette1 = Palette::default();
        let palette2 = Palette::default();

        assert_eq!(palette1.accent, palette2.accent);
        assert_eq!(palette1.success, palette2.success);
        assert_eq!(palette1.danger, palette2.danger);
        assert_eq!(palette1.text, palette2.text);
    }
}
