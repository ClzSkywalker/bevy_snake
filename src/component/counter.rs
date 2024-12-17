use bevy::prelude::*;

#[derive(Debug, Component)]
#[require(Text, TextFont, TextLayout, Node)]
pub struct CounterComp;

impl CounterComp {
    pub fn new(value: String) -> (Self, Text, TextFont, TextLayout, Node) {
        (
            CounterComp,
            Text::new(value),
            TextFont {
                font_size: 30.,
                ..default()
            },
            TextLayout::new_with_justify(JustifyText::Center),
            Node {
                position_type: PositionType::Relative,
                top: Val::Px(5.),
                justify_self: JustifySelf::Center,
                ..default()
            },
        )
    }
}
