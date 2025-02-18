use makepad_widgets::*;

live_design! {
    TITLE_TEXT = {
        font_size: (14),
        font: {path: dep("crate://self/resources/fonts/A-HarmonyOS-Sans-SC-400-Regular.ttf")}
    }

    REGULAR_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/fonts/A-HarmonyOS-Sans-SC-400-Regular.ttf")}
    }

    TEXT_SUB = {
        font_size: (FONT_SIZE_SUB),
        font: {path: dep("crate://self/resources/fonts/A-HarmonyOS-Sans-SC-400-Regular.ttf")}
    }

    COLOR_PROFILE_CIRCLE = #xfff8ee
    COLOR_DIVIDER = #x00000018
    COLOR_DIVIDER_DARK = #x00000044
}
