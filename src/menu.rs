use bevy::{app::AppExit, prelude::*};

use super::{
    GameState, TEXT_COLOR, despawn_screen::despawn_screen, menu_cloud::animate_clouds,
    menu_cloud::despawn_clouds, menu_cloud::spawn_clouds,
};

#[derive(Resource)]
pub struct MenuAssets {
    pub clouds: Vec<Handle<Image>>,
}

pub fn menu_plugin(app: &mut App) {
    app.init_state::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnEnter(MenuState::Main), spawn_clouds)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnExit(MenuState::Main), despawn_clouds)
        .add_systems(OnExit(MenuState::Main), cleanup_assets)
        .add_systems(
            Update,
            (menu_action, button_system, animate_clouds).run_if(in_state(GameState::Menu)),
        );
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
    Play,
    Quit,
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let button_node = Node {
        width: Val::Percent(40.0),
        height: Val::Percent(10.0),
        margin: UiRect::all(Val::Percent(5.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        top: Val::Percent(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: window.height() * 0.045,
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                height: Val::Percent(50.0),
                width: Val::Percent(40.0),
                ..default()
            },
            BackgroundColor(Color::linear_rgba(0.0, 0.0, 0.0, 0.5)),
            children![
                (
                    Text::new("Dark City"),
                    TextFont {
                        font_size: window.height() * 0.1,
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        top: Val::Percent(10.0),
                        margin: UiRect::bottom(Val::Percent(5.0)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![(
                        Text::new("New Game"),
                        button_text_font.clone(),
                        TextColor(TEXT_COLOR),
                    ),]
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![(Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),]
                ),
            ]
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::Play => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}

pub fn load_menu_assets(commands: &mut Commands, asset_server: Res<AssetServer>) {
    let cloud_images = vec![
        asset_server.load("menu/cloud1.png"),
        asset_server.load("menu/cloud2.png"),
    ];

    commands.insert_resource(MenuAssets {
        clouds: cloud_images,
    });
}

pub fn are_menu_assets_loaded(
    menu_assets: Res<MenuAssets>,
    asset_server: Res<AssetServer>,
) -> bool {
    let all_loaded = menu_assets
        .clouds
        .iter()
        .all(|handle| asset_server.is_loaded_with_dependencies(handle));

    all_loaded
}

fn cleanup_assets(mut commands: Commands) {
    commands.remove_resource::<MenuAssets>();
}
