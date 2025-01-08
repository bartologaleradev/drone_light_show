pub mod colors {

    #[derive(PartialEq)]
    enum BlendMode {
        NORMAL,
        MULTIPLY,
        SCREEN,
        DARKEN,
        LIGHTEN,
        OVERLAY,
        SOFT_LIGHT,
        HARD_LIGHT
    }

    impl BlendMode {
        fn blend_normal(source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                backdrop[i] = a * source[i] + b * backdrop[i]
            }
        }

        fn blend_multiply (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                backdrop[i] = a * backdrop[i] * source[i] + b * backdrop[i]
            }
        }
        
        fn blend_screen(source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                backdrop[i] = a * (1.0 -(1.0 - backdrop[i]) * (1.0 - source[i])) + b * backdrop[i];
            }
        }

        fn blend_darken (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                backdrop[i] =  a * backdrop[i].min(source[i]) + b * backdrop[i];
            }
        }

        fn blend_lighten (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                backdrop[i] =  a * backdrop[i].max(source[i]) + b * backdrop[i];
            }
        }

        fn blend_overlay (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                if backdrop[i] >= 0.5 {
                    backdrop[i] = (a * (1.0 - (2.0 - 2.0 * backdrop[i]) * ( 1.0 - source[i])) + b * backdrop[i]);
                } else {
                    backdrop[i] = a * (2.0 * backdrop[i]) * source[i] + b * backdrop[i];
                }
            }
        }

        fn blend_hard_light (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                if source[i] <= 0.5 {
                    backdrop[i] = a * backdrop[i] * (2.0 * source[i]) + b * backdrop[i];
                } else {
                    backdrop[i] = (a * (1.0 - (1.0 - backdrop[i]) * ( 2.0 - 2.0 * source[i]))) + b * backdrop[i];
                }
            }
        }

        /**
         * There are multiple variants for this mode; the variant below is the W3C recomendation, not the one in Photosop
         * See https://imagineer.in/blog/math-behind-blend-modes/
         */
        fn blend_soft_light (source: &[f32; 4], backdrop: &mut [f32; 4], a: f32, b: f32) {
            for i in 0..4 {
                if source[i] <= 0.5 {
                    backdrop[i] = (a * (backdrop[i] - (1.0 - 2.0 * source[i]) * backdrop[i] * (1.0 - backdrop[i])))
                    + b * backdrop[i];
                } else {
                    let mut d: f32;

                    if backdrop[i] <= 0.25 {
                        d = ((16.0 * backdrop[i] - 12.0) * backdrop[i] + 4.0) * backdrop[i];
                    } else {
                        d = backdrop[i].powf(0.5);
                    }

                    backdrop[i] = (a * (backdrop[i] + (2.0 * source[i] - 1.0) * (d - backdrop[i])) - b * backdrop[i]);
                }
            }
        }

        /**
         * Blend two colors according to standard alpha compositing rules, using the given 
         * blending mode and updating the second color in-place
         */
        pub fn blend_in_place (source: &[f32; 4], backdrop: &mut [f32; 4], mode: BlendMode) {
            let alpha_source = source[3];

            //Shortcut for fully transparent source
            if alpha_source <= 0.0 { return;}

            //Apply the blending mode to the RGB part of the source and the backdrop
            if alpha_source >= 1.0 && mode == BlendMode::NORMAL {
                *backdrop = *source;
                return;
            }
            let alpha_backdrop = backdrop[3];
            let alpha_overlay: f32;
            let a: f32;

            if alpha_backdrop >= 1.0{
                alpha_overlay = 1.0;
                a = alpha_source;
            } else {
                alpha_overlay = 1.0 - (1.0 - alpha_source) * (1.0 - alpha_backdrop);
                a = alpha_source / alpha_overlay;
            }

            let b = 1.0 - a;
            //Obtenemos la funcion de calculo en base al tipo de BlendMode
            match mode {
                BlendMode::NORMAL => BlendMode::blend_normal(source, backdrop, a, b),
                BlendMode::MULTIPLY => BlendMode::blend_multiply(source, backdrop, a, b),
                BlendMode::SCREEN => BlendMode::blend_screen(source, backdrop, a, b),
                BlendMode::DARKEN => BlendMode::blend_darken(source, backdrop, a, b),
                BlendMode::LIGHTEN => BlendMode::blend_lighten(source, backdrop, a, b),
                BlendMode::OVERLAY => BlendMode::blend_overlay(source, backdrop, a, b),
                BlendMode::SOFT_LIGHT => BlendMode::blend_soft_light(source, backdrop, a, b),
                BlendMode::HARD_LIGHT => BlendMode::blend_hard_light(source, backdrop, a, b),
            }

            backdrop[3] = alpha_overlay;
        }

    }
}