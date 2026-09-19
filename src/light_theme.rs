use bevy::feathers::{palette, tokens};
use bevy::color::{Alpha, Color, Luminance};
use bevy::platform::collections::HashMap;

use bevy::feathers::theme::ThemeProps;

const GRAY_4: Color = Color::oklcha(0.47, 0.011, 278.38, 1.0);
pub fn create_light_theme() -> ThemeProps {
    ThemeProps {
        color: HashMap::from([
            // Window
            //
            // GRAY_0 = window background
            // GRAY_1 = pane background
            // GRAY_2 = item background
            // GRAY_3 = active item
            //
            (tokens::WINDOW_BG, Color::srgb(0.97, 0.97, 0.97)),
            (tokens::FOCUS_RING, palette::ACCENT.with_alpha(0.5)),
            (tokens::TEXT_MAIN, Color::srgb(0.10, 0.10, 0.11)),
            (tokens::TEXT_DIM, Color::srgb(0.40, 0.40, 0.42)),

            // Button (normal)
            (tokens::BUTTON_BG, Color::srgb(0.93, 0.93, 0.94)),
            (
                tokens::BUTTON_BG_HOVER,
                Color::srgb(0.89, 0.89, 0.90),
            ),
            (
                tokens::BUTTON_BG_PRESSED,
                Color::srgb(0.84, 0.84, 0.85),
            ),
            (
                tokens::BUTTON_BG_DISABLED,
                Color::srgb(0.92, 0.92, 0.92),
            ),

            // Button (primary)
            (tokens::BUTTON_PRIMARY_BG, palette::ACCENT),
            (
                tokens::BUTTON_PRIMARY_BG_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::BUTTON_PRIMARY_BG_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::BUTTON_PRIMARY_BG_DISABLED,
                Color::srgb(0.75, 0.75, 0.75),
            ),

            // Button (plain)
            (tokens::BUTTON_PLAIN_BG, Color::NONE),
            (
                tokens::BUTTON_PLAIN_BG_HOVER,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::BUTTON_PLAIN_BG_PRESSED,
                Color::srgb(0.88, 0.88, 0.89),
            ),
            (tokens::BUTTON_PLAIN_BG_DISABLED, Color::NONE),

            // Button text
            (tokens::BUTTON_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::BUTTON_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),
            (tokens::BUTTON_PRIMARY_TEXT, palette::WHITE),
            (
                tokens::BUTTON_PRIMARY_TEXT_DISABLED,
                palette::WHITE.with_alpha(0.5),
            ),

            // Slider
            (
                tokens::SLIDER_BG,
                Color::srgb(0.88, 0.88, 0.89),
            ),
            (
                tokens::SLIDER_BG_HOVER,
                Color::srgb(0.84, 0.84, 0.85),
            ),
            (
                tokens::SLIDER_BG_PRESSED,
                Color::srgb(0.80, 0.80, 0.81),
            ),
            (
                tokens::SLIDER_BG_DISABLED,
                Color::srgb(0.90, 0.90, 0.90),
            ),
            (tokens::SLIDER_BAR, palette::ACCENT),
            (
                tokens::SLIDER_BAR_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::SLIDER_BAR_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::SLIDER_BAR_DISABLED,
                Color::srgb(0.75, 0.75, 0.76),
            ),
            (tokens::SLIDER_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::SLIDER_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),

            // Scrollbar
            (
                tokens::SCROLLBAR_BG,
                Color::srgb(0.92, 0.92, 0.93),
            ),
            (
                tokens::SCROLLBAR_THUMB,
                Color::srgb(0.65, 0.65, 0.67),
            ),
            (
                tokens::SCROLLBAR_THUMB_HOVER,
                Color::srgb(0.55, 0.55, 0.57),
            ),

            // Checkbox
            (
                tokens::CHECKBOX_BG,
                Color::srgb(0.94, 0.94, 0.95),
            ),
            (
                tokens::CHECKBOX_BG_HOVER,
                Color::srgb(0.90, 0.90, 0.91),
            ),
            (
                tokens::CHECKBOX_BG_PRESSED,
                Color::srgb(0.85, 0.85, 0.86),
            ),
            (
                tokens::CHECKBOX_BG_DISABLED,
                Color::srgb(0.90, 0.90, 0.90).with_alpha(0.5),
            ),
            (tokens::CHECKBOX_BG_CHECKED, palette::ACCENT),
            (
                tokens::CHECKBOX_BG_CHECKED_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::CHECKBOX_BG_CHECKED_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::CHECKBOX_BG_CHECKED_DISABLED,
                Color::srgb(0.75, 0.75, 0.76).with_alpha(0.5),
            ),

            (tokens::CHECKBOX_BORDER, Color::srgb(0.70, 0.70, 0.72)),
            (
                tokens::CHECKBOX_BORDER_HOVER,
                Color::srgb(0.60, 0.60, 0.62),
            ),
            (
                tokens::CHECKBOX_BORDER_PRESSED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (
                tokens::CHECKBOX_BORDER_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.5),
            ),
            (tokens::CHECKBOX_BORDER_CHECKED, palette::ACCENT),
            (
                tokens::CHECKBOX_BORDER_CHECKED_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::CHECKBOX_BORDER_CHECKED_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::CHECKBOX_BORDER_CHECKED_DISABLED,
                Color::srgb(0.75, 0.75, 0.76).with_alpha(0.5),
            ),

            (tokens::CHECKBOX_MARK, palette::WHITE),
            (
                tokens::CHECKBOX_MARK_DISABLED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (tokens::CHECKBOX_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::CHECKBOX_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),

            // Radio
            (tokens::RADIO_BORDER, Color::srgb(0.70, 0.70, 0.72)),
            (
                tokens::RADIO_BORDER_HOVER,
                Color::srgb(0.60, 0.60, 0.62),
            ),
            (
                tokens::RADIO_BORDER_PRESSED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (
                tokens::RADIO_BORDER_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.5),
            ),
            (tokens::RADIO_BORDER_CHECKED, palette::ACCENT),
            (
                tokens::RADIO_BORDER_CHECKED_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::RADIO_BORDER_CHECKED_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::RADIO_BORDER_CHECKED_DISABLED,
                Color::srgb(0.75, 0.75, 0.76).with_alpha(0.5),
            ),

            (tokens::RADIO_MARK, palette::ACCENT),
            (
                tokens::RADIO_MARK_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::RADIO_MARK_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::RADIO_MARK_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.5),
            ),

            (tokens::RADIO_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::RADIO_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),

            // Toggle Switch
            (
                tokens::SWITCH_BG,
                Color::srgb(0.85, 0.85, 0.86),
            ),
            (
                tokens::SWITCH_BG_HOVER,
                Color::srgb(0.80, 0.80, 0.81),
            ),
            (
                tokens::SWITCH_BG_PRESSED,
                Color::srgb(0.75, 0.75, 0.76),
            ),
            (
                tokens::SWITCH_BG_DISABLED,
                Color::srgb(0.85, 0.85, 0.86).with_alpha(0.5),
            ),
            (tokens::SWITCH_BG_CHECKED, palette::ACCENT),
            (
                tokens::SWITCH_BG_CHECKED_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::SWITCH_BG_CHECKED_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::SWITCH_BG_CHECKED_DISABLED,
                Color::srgb(0.75, 0.75, 0.76).with_alpha(0.5),
            ),

            (tokens::SWITCH_BORDER, Color::srgb(0.70, 0.70, 0.72)),
            (
                tokens::SWITCH_BORDER_HOVER,
                Color::srgb(0.60, 0.60, 0.62),
            ),
            (
                tokens::SWITCH_BORDER_PRESSED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (
                tokens::SWITCH_BORDER_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.5),
            ),
            (tokens::SWITCH_BORDER_CHECKED, palette::ACCENT),
            (
                tokens::SWITCH_BORDER_CHECKED_HOVER,
                palette::ACCENT.lighter(0.05),
            ),
            (
                tokens::SWITCH_BORDER_CHECKED_PRESSED,
                palette::ACCENT.lighter(0.10),
            ),
            (
                tokens::SWITCH_BORDER_CHECKED_DISABLED,
                Color::srgb(0.75, 0.75, 0.76).with_alpha(0.5),
            ),

            // Switch slide
            (tokens::SWITCH_SLIDE_BG, palette::WHITE),
            (tokens::SWITCH_SLIDE_BG_HOVER, palette::WHITE),
            (tokens::SWITCH_SLIDE_BG_PRESSED, palette::WHITE),
            (
                tokens::SWITCH_SLIDE_BG_DISABLED,
                Color::srgb(0.85, 0.85, 0.86).with_alpha(0.5),
            ),
            (tokens::SWITCH_SLIDE_BG_CHECKED, palette::WHITE),
            (tokens::SWITCH_SLIDE_BG_CHECKED_HOVER, palette::WHITE),
            (tokens::SWITCH_SLIDE_BG_CHECKED_PRESSED, palette::WHITE),
            (
                tokens::SWITCH_SLIDE_BG_CHECKED_DISABLED,
                Color::srgb(0.80, 0.80, 0.81).with_alpha(0.3),
            ),

            (tokens::SWITCH_SLIDE_BORDER, Color::srgb(0.70, 0.70, 0.72)),
            (
                tokens::SWITCH_SLIDE_BORDER_HOVER,
                Color::srgb(0.60, 0.60, 0.62),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_PRESSED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.5),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_CHECKED,
                Color::srgb(0.70, 0.70, 0.72),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_CHECKED_HOVER,
                Color::srgb(0.60, 0.60, 0.62),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_CHECKED_PRESSED,
                Color::srgb(0.50, 0.50, 0.52),
            ),
            (
                tokens::SWITCH_SLIDE_BORDER_CHECKED_DISABLED,
                Color::srgb(0.70, 0.70, 0.72).with_alpha(0.3),
            ),

            // Color plane
            (
                tokens::COLOR_PLANE_BG,
                Color::srgb(0.93, 0.93, 0.94),
            ),

            // Menus
            //
            // Popup = blanc
            // Item normal = blanc
            // Hover = gris clair
            // Pressed / focused = gris un peu plus foncé
            //
            (tokens::MENU_BG, palette::WHITE),
            (
                tokens::MENU_BORDER,
                Color::srgb(0.75, 0.75, 0.76),
            ),
            (
                tokens::MENUITEM_BG_HOVER,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::MENUITEM_BG_PRESSED,
                Color::srgb(0.87, 0.87, 0.88),
            ),
            (
                tokens::MENUITEM_BG_FOCUSED,
                Color::srgb(0.90, 0.90, 0.91),
            ),
            (tokens::MENUITEM_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::MENUITEM_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),

            // Text Input
            (tokens::TEXT_INPUT_BG, palette::WHITE),
            (
                tokens::TEXT_INPUT_LABEL_BG,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::TEXT_INPUT_TEXT,
                Color::srgb(0.10, 0.10, 0.11),
            ),
            (
                tokens::TEXT_INPUT_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),
            (
                tokens::TEXT_INPUT_CURSOR,
                palette::ACCENT.lighter(0.2),
            ),
            (tokens::TEXT_INPUT_SELECTION, palette::ACCENT),
            (
                tokens::TEXT_INPUT_SELECTION_UNFOCUSED,
                palette::TRANSPARENT,
            ),
            (tokens::TEXT_INPUT_X_AXIS, palette::X_AXIS),
            (tokens::TEXT_INPUT_Y_AXIS, palette::Y_AXIS),
            (tokens::TEXT_INPUT_Z_AXIS, palette::Z_AXIS),

            // Pane
            (
                tokens::PANE_HEADER_BG,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::PANE_HEADER_BORDER,
                Color::srgb(0.75, 0.75, 0.76),
            ),
            (
                tokens::PANE_HEADER_TEXT,
                Color::srgb(0.10, 0.10, 0.11),
            ),
            (
                tokens::PANE_HEADER_DIVIDER,
                Color::srgb(0.75, 0.75, 0.76),
            ),
            (
                tokens::PANE_BODY_BG,
                Color::srgb(0.97, 0.97, 0.97),
            ),

            // Subpane
            (
                tokens::SUBPANE_HEADER_BG,
                Color::srgb(0.90, 0.90, 0.91),
            ),
            (
                tokens::SUBPANE_HEADER_BORDER,
                Color::srgb(0.80, 0.80, 0.81),
            ),
            (
                tokens::SUBPANE_HEADER_TEXT,
                Color::srgb(0.10, 0.10, 0.11),
            ),
            (
                tokens::SUBPANE_BODY_BG,
                Color::srgb(0.97, 0.97, 0.97),
            ),
            (
                tokens::SUBPANE_BODY_BORDER,
                Color::srgb(0.85, 0.85, 0.86),
            ),

            // Group
            (
                tokens::GROUP_HEADER_BG,
                Color::srgb(0.90, 0.90, 0.91),
            ),
            (
                tokens::GROUP_HEADER_BORDER,
                Color::srgb(0.80, 0.80, 0.81),
            ),
            (
                tokens::GROUP_HEADER_TEXT,
                Color::srgb(0.10, 0.10, 0.11),
            ),
            (
                tokens::GROUP_BODY_BG,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::GROUP_BODY_BORDER,
                Color::srgb(0.80, 0.80, 0.81),
            ),

            // Listview
            (tokens::LISTROW_BG, Color::NONE),
            (
                tokens::LISTROW_BG_HOVER,
                Color::srgb(0.93, 0.93, 0.94),
            ),
            (
                tokens::LISTROW_BG_SELECTED,
                Color::srgb(0.87, 0.87, 0.88),
            ),
            (tokens::LISTROW_TEXT, Color::srgb(0.10, 0.10, 0.11)),
            (
                tokens::LISTROW_TEXT_DISABLED,
                Color::srgb(0.50, 0.50, 0.52).with_alpha(0.5),
            ),
        ]),
    }
}