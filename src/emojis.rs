use crate::utils;
use dirs;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emoji {
    pub code: String,
    pub name: String,
    pub emoji: String,
    pub entity: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmojisObject {
    pub emojis: Vec<Emoji>,
}

pub fn get_emojis() -> Result<EmojisObject, serde_json::Error> {
    let home_dir = dirs::home_dir().expect(&utils::format_error_message(
        "Failed to get home directory.",
    ));
    let emojis_json_path = home_dir.join("emojis.json");

    if emojis_json_path.exists() {
        let emojis_json = std::fs::read_to_string(emojis_json_path).expect(
            &utils::format_error_message("Failed to read file from home directory."),
        );

        let emojis_object: EmojisObject = serde_json::from_str(&emojis_json)
            .expect(&utils::format_error_message("Invalid JSON format."));

        return Ok(emojis_object);
    }

    let emoji_data = r#"{
        "emojis": [
            {
                "emoji": "🎨",
                "entity": "&#x1f3a8;",
                "code": ":art:",
                "description": "Improve structure / format of the code.",
                "name": "art"
            },
            {
                "emoji": "⚡️",
                "entity": "&#x26a1;",
                "code": ":zap:",
                "description": "Improve performance.",
                "name": "zap"
            },
            {
                "emoji": "🔥",
                "entity": "&#x1f525;",
                "code": ":fire:",
                "description": "Remove code or files.",
                "name": "fire"
            },
            {
                "emoji": "🐛",
                "entity": "&#x1f41b;",
                "code": ":bug:",
                "description": "Fix a bug.",
                "name": "bug"
            },
            {
                "emoji": "🚑️",
                "entity": "&#128657;",
                "code": ":ambulance:",
                "description": "Critical hotfix.",
                "name": "ambulance"
            },
            {
                "emoji": "✨",
                "entity": "&#x2728;",
                "code": ":sparkles:",
                "description": "Introduce new features.",
                "name": "sparkles"
            },
            {
                "emoji": "📝",
                "entity": "&#x1f4dd;",
                "code": ":memo:",
                "description": "Add or update documentation.",
                "name": "memo"
            },
            {
                "emoji": "🚀",
                "entity": "&#x1f680;",
                "code": ":rocket:",
                "description": "Deploy stuff.",
                "name": "rocket"
            },
            {
                "emoji": "💄",
                "entity": "&#ff99cc;",
                "code": ":lipstick:",
                "description": "Add or update the UI and style files.",
                "name": "lipstick"
            },
            {
                "emoji": "🎉",
                "entity": "&#127881;",
                "code": ":tada:",
                "description": "Begin a project.",
                "name": "tada"
            },
            {
                "emoji": "✅",
                "entity": "&#x2705;",
                "code": ":white_check_mark:",
                "description": "Add, update, or pass tests.",
                "name": "white-check-mark"
            },
            {
                "emoji": "🔒️",
                "entity": "&#x1f512;",
                "code": ":lock:",
                "description": "Fix security or privacy issues.",
                "name": "lock"
            },
            {
                "emoji": "🔐",
                "entity": "&#x1f510;",
                "code": ":closed_lock_with_key:",
                "description": "Add or update secrets.",
                "name": "closed-lock-with-key"
            },
            {
                "emoji": "🔖",
                "entity": "&#x1f516;",
                "code": ":bookmark:",
                "description": "Release / Version tags.",
                "name": "bookmark"
            },
            {
                "emoji": "🚨",
                "entity": "&#x1f6a8;",
                "code": ":rotating_light:",
                "description": "Fix compiler / linter warnings.",
                "name": "rotating-light"
            },
            {
                "emoji": "🚧",
                "entity": "&#x1f6a7;",
                "code": ":construction:",
                "description": "Work in progress.",
                "name": "construction"
            },
            {
                "emoji": "💚",
                "entity": "&#x1f49a;",
                "code": ":green_heart:",
                "description": "Fix CI Build.",
                "name": "green-heart"
            },
            {
                "emoji": "⬇️",
                "entity": "⬇️",
                "code": ":arrow_down:",
                "description": "Downgrade dependencies.",
                "name": "arrow-down"
            },
            {
                "emoji": "⬆️",
                "entity": "⬆️",
                "code": ":arrow_up:",
                "description": "Upgrade dependencies.",
                "name": "arrow-up"
            },
            {
                "emoji": "📌",
                "entity": "&#x1F4CC;",
                "code": ":pushpin:",
                "description": "Pin dependencies to specific versions.",
                "name": "pushpin"
            },
            {
                "emoji": "👷",
                "entity": "&#x1f477;",
                "code": ":construction_worker:",
                "description": "Add or update CI build system.",
                "name": "construction-worker"
            },
            {
                "emoji": "📈",
                "entity": "&#x1F4C8;",
                "code": ":chart_with_upwards_trend:",
                "description": "Add or update analytics or track code.",
                "name": "chart-with-upwards-trend"
            },
            {
                "emoji": "♻️",
                "entity": "&#x267b;",
                "code": ":recycle:",
                "description": "Refactor code.",
                "name": "recycle"
            },
            {
                "emoji": "➕",
                "entity": "&#10133;",
                "code": ":heavy_plus_sign:",
                "description": "Add a dependency.",
                "name": "heavy-plus-sign"
            },
            {
                "emoji": "➖",
                "entity": "&#10134;",
                "code": ":heavy_minus_sign:",
                "description": "Remove a dependency.",
                "name": "heavy-minus-sign"
            },
            {
                "emoji": "🔧",
                "entity": "&#x1f527;",
                "code": ":wrench:",
                "description": "Add or update configuration files.",
                "name": "wrench"
            },
            {
                "emoji": "🔨",
                "entity": "&#128296;",
                "code": ":hammer:",
                "description": "Add or update development scripts.",
                "name": "hammer"
            },
            {
                "emoji": "🌐",
                "entity": "&#127760;",
                "code": ":globe_with_meridians:",
                "description": "Internationalization and localization.",
                "name": "globe-with-meridians"
            },
            {
                "emoji": "✏️",
                "entity": "&#59161;",
                "code": ":pencil2:",
                "description": "Fix typos.",
                "name": "pencil2"
            },
            {
                "emoji": "💩",
                "entity": "&#58613;",
                "code": ":poop:",
                "description": "Write bad code that needs to be improved.",
                "name": "poop"
            },
            {
                "emoji": "⏪️",
                "entity": "&#9194;",
                "code": ":rewind:",
                "description": "Revert changes.",
                "name": "rewind"
            },
            {
                "emoji": "🔀",
                "entity": "&#128256;",
                "code": ":twisted_rightwards_arrows:",
                "description": "Merge branches.",
                "name": "twisted-rightwards-arrows"
            },
            {
                "emoji": "📦️",
                "entity": "&#1F4E6;",
                "code": ":package:",
                "description": "Add or update compiled files or packages.",
                "name": "package"
            },
            {
                "emoji": "👽️",
                "entity": "&#1F47D;",
                "code": ":alien:",
                "description": "Update code due to external API changes.",
                "name": "alien"
            },
            {
                "emoji": "🚚",
                "entity": "&#1F69A;",
                "code": ":truck:",
                "description": "Move or rename resources (e.g.: files, paths, routes).",
                "name": "truck"
            },
            {
                "emoji": "📄",
                "entity": "&#1F4C4;",
                "code": ":page_facing_up:",
                "description": "Add or update license.",
                "name": "page-facing-up"
            },
            {
                "emoji": "💥",
                "entity": "&#x1f4a5;",
                "code": ":boom:",
                "description": "Introduce breaking changes.",
                "name": "boom"
            },
            {
                "emoji": "🍱",
                "entity": "&#1F371",
                "code": ":bento:",
                "description": "Add or update assets.",
                "name": "bento"
            },
            {
                "emoji": "♿️",
                "entity": "&#9855;",
                "code": ":wheelchair:",
                "description": "Improve accessibility.",
                "name": "wheelchair"
            },
            {
                "emoji": "💡",
                "entity": "&#128161;",
                "code": ":bulb:",
                "description": "Add or update comments in source code.",
                "name": "bulb"
            },
            {
                "emoji": "🍻",
                "entity": "&#x1f37b;",
                "code": ":beers:",
                "description": "Write code drunkenly.",
                "name": "beers"
            },
            {
                "emoji": "💬",
                "entity": "&#128172;",
                "code": ":speech_balloon:",
                "description": "Add or update text and literals.",
                "name": "speech-balloon"
            },
            {
                "emoji": "🗃️",
                "entity": "&#128451;",
                "code": ":card_file_box:",
                "description": "Perform database related changes.",
                "name": "card-file-box"
            },
            {
                "emoji": "🔊",
                "entity": "&#128266;",
                "code": ":loud_sound:",
                "description": "Add or update logs.",
                "name": "loud-sound"
            },
            {
                "emoji": "🔇",
                "entity": "&#128263;",
                "code": ":mute:",
                "description": "Remove logs.",
                "name": "mute"
            },
            {
                "emoji": "👥",
                "entity": "&#128101;",
                "code": ":busts_in_silhouette:",
                "description": "Add or update contributor(s).",
                "name": "busts-in-silhouette"
            },
            {
                "emoji": "🚸",
                "entity": "&#128696;",
                "code": ":children_crossing:",
                "description": "Improve user experience / usability.",
                "name": "children-crossing"
            },
            {
                "emoji": "🏗️",
                "entity": "&#1f3d7;",
                "code": ":building_construction:",
                "description": "Make architectural changes.",
                "name": "building-construction"
            },
            {
                "emoji": "📱",
                "entity": "&#128241;",
                "code": ":iphone:",
                "description": "Work on responsive design.",
                "name": "iphone"
            },
            {
                "emoji": "🤡",
                "entity": "&#129313;",
                "code": ":clown_face:",
                "description": "Mock things.",
                "name": "clown-face"
            },
            {
                "emoji": "🥚",
                "entity": "&#129370;",
                "code": ":egg:",
                "description": "Add or update an easter egg.",
                "name": "egg"
            },
            {
                "emoji": "🙈",
                "entity": "&#8bdfe7;",
                "code": ":see_no_evil:",
                "description": "Add or update a .gitignore file.",
                "name": "see-no-evil"
            },
            {
                "emoji": "📸",
                "entity": "&#128248;",
                "code": ":camera_flash:",
                "description": "Add or update snapshots.",
                "name": "camera-flash"
            },
            {
                "emoji": "⚗️",
                "entity": "&#x2697;",
                "code": ":alembic:",
                "description": "Perform experiments.",
                "name": "alembic"
            },
            {
                "emoji": "🔍️",
                "entity": "&#128269;",
                "code": ":mag:",
                "description": "Improve SEO.",
                "name": "mag"
            },
            {
                "emoji": "🏷️",
                "entity": "&#127991;",
                "code": ":label:",
                "description": "Add or update types.",
                "name": "label"
            },
            {
                "emoji": "🌱",
                "entity": "&#127793;",
                "code": ":seedling:",
                "description": "Add or update seed files.",
                "name": "seedling"
            },
            {
                "emoji": "🚩",
                "entity": "&#x1F6A9;",
                "code": ":triangular_flag_on_post:",
                "description": "Add, update, or remove feature flags.",
                "name": "triangular-flag-on-post"
            },
            {
                "emoji": "🥅",
                "entity": "&#x1F945;",
                "code": ":goal_net:",
                "description": "Catch errors.",
                "name": "goal-net"
            },
            {
                "emoji": "💫",
                "entity": "&#x1f4ab;",
                "code": ":dizzy:",
                "description": "Add or update animations and transitions.",
                "name": "dizzy"
            },
            {
                "emoji": "🗑️",
                "entity": "&#x1F5D1;",
                "code": ":wastebasket:",
                "description": "Deprecate code that needs to be cleaned up.",
                "name": "wastebasket"
            },
            {
                "emoji": "🛂",
                "entity": "&#x1F6C2;",
                "code": ":passport_control:",
                "description": "Work on code related to authorization, roles and permissions.",
                "name": "passport-control"
            },
            {
                "emoji": "🩹",
                "entity": "&#x1FA79;",
                "code": ":adhesive_bandage:",
                "description": "Simple fix for a non-critical issue.",
                "name": "adhesive-bandage"
            },
            {
                "emoji": "🧐",
                "entity": "&#x1F9D0;",
                "code": ":monocle_face:",
                "description": "Data exploration/inspection.",
                "name": "monocle-face"
            },
            {
                "emoji": "⚰️",
                "entity": "&#x26B0;",
                "code": ":coffin:",
                "description": "Remove dead code.",
                "name": "coffin"
            },
            {
                "emoji": "🧪",
                "entity": "&#x1F9EA;",
                "code": ":test_tube:",
                "description": "Add a failing test.",
                "name": "test-tube"
            },
            {
                "emoji": "👔",
                "entity": "&#128084;",
                "code": ":necktie:",
                "description": "Add or update business logic.",
                "name": "necktie"
            },
            {
                "emoji": "🩺",
                "entity": "&#x1FA7A;",
                "code": ":stethoscope:",
                "description": "Add or update healthcheck.",
                "name": "stethoscope"
            },
            {
                "emoji": "🧱",
                "entity": "&#x1f9f1;",
                "code": ":bricks:",
                "description": "Infrastructure related changes.",
                "name": "bricks"
            },
            {
                "emoji": "🧑‍💻",
                "entity": "&#129489;&#8205;&#128187;",
                "code": ":technologist:",
                "description": "Improve developer experience.",
                "name": "technologist"
            },
            {
                "emoji": "💸",
                "entity": "&#x1F4B8;",
                "code": ":money_with_wings:",
                "description": "Add sponsorships or money related infrastructure.",
                "name": "money-with-wings"
            },
            {
                "emoji": "🧵",
                "entity": "&#x1F9F5;",
                "code": ":thread:",
                "description": "Add or update code related to multithreading or concurrency.",
                "name": "thread"
            },
            {
                "emoji": "🦺",
                "entity": "&#x1F9BA;",
                "code": ":safety_vest:",
                "description": "Add or update code related to validation.",
                "name": "safety-vest"
            },
            {
                "emoji": "✈️",
                "entity": "&#x2708;",
                "code": ":airplane:",
                "description": "Improve offline support.",
                "name": "airplane"
            }
        ]
    }
    "#;

    let emojis_object: EmojisObject = serde_json::from_str(emoji_data)
        .expect(&utils::format_error_message("Invalid JSON format."));

    std::fs::write(&emojis_json_path, emoji_data).expect(&utils::format_error_message(&format!(
        "Failed to write file to home directory: {}",
        emojis_json_path.display()
    )));

    return Ok(emojis_object);
}
