use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug)]
pub struct CustomColorMode {
    light_model: String,
    dark_model: String,
}

/// 飞书卡片颜色枚举值
#[derive(Debug)]
pub enum FeishuCardColor {
    Custom(String),

    BgWhite,

    White,

    Blue,

    Blue50,

    Blue100,

    Blue200,

    Blue300,

    Blue350,

    Blue400,

    Blue500,

    Blue600,

    Blue700,

    Blue800,

    Blue900,

    Carmine,

    Carmine50,

    Carmine100,

    Carmine200,

    Carmine300,

    Carmine350,

    Carmine400,

    Carmine500,

    Carmine600,

    Carmine700,

    Carmine800,

    Carmine900,

    Green,

    Green50,

    Green100,

    Green200,

    Green300,

    Green350,

    Green400,

    Green500,

    Green600,

    Green700,

    Green800,

    Green900,

    Indigo,

    Indigo50,

    Indigo100,

    Indigo200,

    Indigo300,

    Indigo350,

    Indigo400,

    Indigo500,

    Indigo600,

    Indigo700,

    Indigo800,

    Indigo900,

    Lime,

    Lime50,

    Lime100,

    Lime200,

    Lime300,

    Lime350,

    Lime400,

    Lime500,

    Lime600,

    Lime700,

    Lime800,

    Lime900,

    Grey,

    Grey50,

    Grey100,

    Grey200,

    Grey300,

    Grey350,

    Grey400,

    Grey500,

    Grey600,

    Grey700,

    Grey800,

    Grey900,

    Orange,

    Orange50,

    Orange100,

    Orange200,

    Orange300,

    Orange350,

    Orange400,

    Orange500,

    Orange600,

    Orange700,

    Orange800,

    Orange900,

    Purple,

    Purple50,

    Purple100,

    Purple200,

    Purple300,

    Purple350,

    Purple400,

    Purple500,

    Purple600,

    Purple700,

    Purple800,

    Purple900,

    Red,

    Red50,

    Red100,

    Red200,

    Red300,

    Red350,

    Red400,

    Red500,

    Red600,

    Red700,

    Red800,

    Red900,

    Sunflower,

    Sunflower50,

    Sunflower100,

    Sunflower200,

    Sunflower300,

    Sunflower350,

    Sunflower400,

    Sunflower500,

    Sunflower600,

    Sunflower700,

    Sunflower800,

    Sunflower900,

    Turquoise,

    Turquoise50,

    Turquoise100,

    Turquoise200,

    Turquoise300,

    Turquoise350,

    Turquoise400,

    Turquoise500,

    Turquoise600,

    Turquoise700,

    Turquoise800,

    Turquoise900,

    Violet,

    Violet50,

    Violet100,

    Violet200,

    Violet300,

    Violet350,

    Violet400,

    Violet500,

    Violet600,

    Violet700,

    Violet800,

    Violet900,

    Wathet,

    Wathet50,

    Wathet100,

    Wathet200,

    Wathet300,

    Wathet350,

    Wathet400,

    Wathet500,

    Wathet600,

    Wathet700,

    Wathet800,

    Wathet900,

    Yellow,

    Yellow50,

    Yellow100,

    Yellow200,

    Yellow300,

    Yellow350,

    Yellow400,

    Yellow500,

    Yellow600,

    Yellow700,

    Yellow800,

    Yellow900,
}

impl Serialize for FeishuCardColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FeishuCardColor::Custom(color) => serializer.serialize_str(color),
            FeishuCardColor::BgWhite => serializer.serialize_str("bg-white"),
            FeishuCardColor::White => serializer.serialize_str("white"),
            FeishuCardColor::Blue => serializer.serialize_str("blue"),
            FeishuCardColor::Blue50 => serializer.serialize_str("blue-50"),
            FeishuCardColor::Blue100 => serializer.serialize_str("blue-100"),
            FeishuCardColor::Blue200 => serializer.serialize_str("blue-200"),
            FeishuCardColor::Blue300 => serializer.serialize_str("blue-300"),
            FeishuCardColor::Blue350 => serializer.serialize_str("blue-350"),
            FeishuCardColor::Blue400 => serializer.serialize_str("blue-400"),
            FeishuCardColor::Blue500 => serializer.serialize_str("blue-500"),
            FeishuCardColor::Blue600 => serializer.serialize_str("blue-600"),
            FeishuCardColor::Blue700 => serializer.serialize_str("blue-700"),
            FeishuCardColor::Blue800 => serializer.serialize_str("blue-800"),
            FeishuCardColor::Blue900 => serializer.serialize_str("blue-900"),
            FeishuCardColor::Carmine => serializer.serialize_str("carmine"),
            FeishuCardColor::Carmine50 => serializer.serialize_str("carmine-50"),
            FeishuCardColor::Carmine100 => serializer.serialize_str("carmine-100"),
            FeishuCardColor::Carmine200 => serializer.serialize_str("carmine-200"),
            FeishuCardColor::Carmine300 => serializer.serialize_str("carmine-300"),
            FeishuCardColor::Carmine350 => serializer.serialize_str("carmine-350"),
            FeishuCardColor::Carmine400 => serializer.serialize_str("carmine-400"),
            FeishuCardColor::Carmine500 => serializer.serialize_str("carmine-500"),
            FeishuCardColor::Carmine600 => serializer.serialize_str("carmine-600"),
            FeishuCardColor::Carmine700 => serializer.serialize_str("carmine-700"),
            FeishuCardColor::Carmine800 => serializer.serialize_str("carmine-800"),
            FeishuCardColor::Carmine900 => serializer.serialize_str("carmine-900"),
            FeishuCardColor::Green => serializer.serialize_str("green"),
            FeishuCardColor::Green50 => serializer.serialize_str("green-50"),
            FeishuCardColor::Green100 => serializer.serialize_str("green-100"),
            FeishuCardColor::Green200 => serializer.serialize_str("green-200"),
            FeishuCardColor::Green300 => serializer.serialize_str("green-300"),
            FeishuCardColor::Green350 => serializer.serialize_str("green-350"),
            FeishuCardColor::Green400 => serializer.serialize_str("green-400"),
            FeishuCardColor::Green500 => serializer.serialize_str("green-500"),
            FeishuCardColor::Green600 => serializer.serialize_str("green-600"),
            FeishuCardColor::Green700 => serializer.serialize_str("green-700"),
            FeishuCardColor::Green800 => serializer.serialize_str("green-800"),
            FeishuCardColor::Green900 => serializer.serialize_str("green-900"),
            FeishuCardColor::Indigo => serializer.serialize_str("indigo"),
            FeishuCardColor::Indigo50 => serializer.serialize_str("indigo-50"),
            FeishuCardColor::Indigo100 => serializer.serialize_str("indigo-100"),
            FeishuCardColor::Indigo200 => serializer.serialize_str("indigo-200"),
            FeishuCardColor::Indigo300 => serializer.serialize_str("indigo-300"),
            FeishuCardColor::Indigo350 => serializer.serialize_str("indigo-350"),
            FeishuCardColor::Indigo400 => serializer.serialize_str("indigo-400"),
            FeishuCardColor::Indigo500 => serializer.serialize_str("indigo-500"),
            FeishuCardColor::Indigo600 => serializer.serialize_str("indigo-600"),
            FeishuCardColor::Indigo700 => serializer.serialize_str("indigo-700"),
            FeishuCardColor::Indigo800 => serializer.serialize_str("indigo-800"),
            FeishuCardColor::Indigo900 => serializer.serialize_str("indigo-900"),
            FeishuCardColor::Lime => serializer.serialize_str("lime"),
            FeishuCardColor::Lime50 => serializer.serialize_str("lime-50"),
            FeishuCardColor::Lime100 => serializer.serialize_str("lime-100"),
            FeishuCardColor::Lime200 => serializer.serialize_str("lime-200"),
            FeishuCardColor::Lime300 => serializer.serialize_str("lime-300"),
            FeishuCardColor::Lime350 => serializer.serialize_str("lime-350"),
            FeishuCardColor::Lime400 => serializer.serialize_str("lime-400"),
            FeishuCardColor::Lime500 => serializer.serialize_str("lime-500"),
            FeishuCardColor::Lime600 => serializer.serialize_str("lime-600"),
            FeishuCardColor::Lime700 => serializer.serialize_str("lime-700"),
            FeishuCardColor::Lime800 => serializer.serialize_str("lime-800"),
            FeishuCardColor::Lime900 => serializer.serialize_str("lime-900"),
            FeishuCardColor::Grey => serializer.serialize_str("grey"),
            FeishuCardColor::Grey50 => serializer.serialize_str("grey-50"),
            FeishuCardColor::Grey100 => serializer.serialize_str("grey-100"),
            FeishuCardColor::Grey200 => serializer.serialize_str("grey-200"),
            FeishuCardColor::Grey300 => serializer.serialize_str("grey-300"),
            FeishuCardColor::Grey350 => serializer.serialize_str("grey-350"),
            FeishuCardColor::Grey400 => serializer.serialize_str("grey-400"),
            FeishuCardColor::Grey500 => serializer.serialize_str("grey-500"),
            FeishuCardColor::Grey600 => serializer.serialize_str("grey-600"),
            FeishuCardColor::Grey700 => serializer.serialize_str("grey-700"),
            FeishuCardColor::Grey800 => serializer.serialize_str("grey-800"),
            FeishuCardColor::Grey900 => serializer.serialize_str("grey-900"),
            FeishuCardColor::Orange => serializer.serialize_str("orange"),
            FeishuCardColor::Orange50 => serializer.serialize_str("orange-50"),
            FeishuCardColor::Orange100 => serializer.serialize_str("orange-100"),
            FeishuCardColor::Orange200 => serializer.serialize_str("orange-200"),
            FeishuCardColor::Orange300 => serializer.serialize_str("orange-300"),
            FeishuCardColor::Orange350 => serializer.serialize_str("orange-350"),
            FeishuCardColor::Orange400 => serializer.serialize_str("orange-400"),
            FeishuCardColor::Orange500 => serializer.serialize_str("orange-500"),
            FeishuCardColor::Orange600 => serializer.serialize_str("orange-600"),
            FeishuCardColor::Orange700 => serializer.serialize_str("orange-700"),
            FeishuCardColor::Orange800 => serializer.serialize_str("orange-800"),
            FeishuCardColor::Orange900 => serializer.serialize_str("orange-900"),
            FeishuCardColor::Purple => serializer.serialize_str("purple"),
            FeishuCardColor::Purple50 => serializer.serialize_str("purple-50"),
            FeishuCardColor::Purple100 => serializer.serialize_str("purple-100"),
            FeishuCardColor::Purple200 => serializer.serialize_str("purple-200"),
            FeishuCardColor::Purple300 => serializer.serialize_str("purple-300"),
            FeishuCardColor::Purple350 => serializer.serialize_str("purple-350"),
            FeishuCardColor::Purple400 => serializer.serialize_str("purple-400"),
            FeishuCardColor::Purple500 => serializer.serialize_str("purple-500"),
            FeishuCardColor::Purple600 => serializer.serialize_str("purple-600"),
            FeishuCardColor::Purple700 => serializer.serialize_str("purple-700"),
            FeishuCardColor::Purple800 => serializer.serialize_str("purple-800"),
            FeishuCardColor::Purple900 => serializer.serialize_str("purple-900"),
            FeishuCardColor::Red => serializer.serialize_str("red"),
            FeishuCardColor::Red50 => serializer.serialize_str("red-50"),
            FeishuCardColor::Red100 => serializer.serialize_str("red-100"),
            FeishuCardColor::Red200 => serializer.serialize_str("red-200"),
            FeishuCardColor::Red300 => serializer.serialize_str("red-300"),
            FeishuCardColor::Red350 => serializer.serialize_str("red-350"),
            FeishuCardColor::Red400 => serializer.serialize_str("red-400"),
            FeishuCardColor::Red500 => serializer.serialize_str("red-500"),
            FeishuCardColor::Red600 => serializer.serialize_str("red-600"),
            FeishuCardColor::Red700 => serializer.serialize_str("red-700"),
            FeishuCardColor::Red800 => serializer.serialize_str("red-800"),
            FeishuCardColor::Red900 => serializer.serialize_str("red-900"),
            FeishuCardColor::Sunflower => serializer.serialize_str("sunflower"),
            FeishuCardColor::Sunflower50 => serializer.serialize_str("sunflower-50"),
            FeishuCardColor::Sunflower100 => serializer.serialize_str("sunflower-100"),
            FeishuCardColor::Sunflower200 => serializer.serialize_str("sunflower-200"),
            FeishuCardColor::Sunflower300 => serializer.serialize_str("sunflower-300"),
            FeishuCardColor::Sunflower350 => serializer.serialize_str("sunflower-350"),
            FeishuCardColor::Sunflower400 => serializer.serialize_str("sunflower-400"),
            FeishuCardColor::Sunflower500 => serializer.serialize_str("sunflower-500"),
            FeishuCardColor::Sunflower600 => serializer.serialize_str("sunflower-600"),
            FeishuCardColor::Sunflower700 => serializer.serialize_str("sunflower-700"),
            FeishuCardColor::Sunflower800 => serializer.serialize_str("sunflower-800"),
            FeishuCardColor::Sunflower900 => serializer.serialize_str("sunflower-900"),
            FeishuCardColor::Turquoise => serializer.serialize_str("turquoise"),
            FeishuCardColor::Turquoise50 => serializer.serialize_str("turquoise-50"),
            FeishuCardColor::Turquoise100 => serializer.serialize_str("turquoise-100"),
            FeishuCardColor::Turquoise200 => serializer.serialize_str("turquoise-200"),
            FeishuCardColor::Turquoise300 => serializer.serialize_str("turquoise-300"),
            FeishuCardColor::Turquoise350 => serializer.serialize_str("turquoise-350"),
            FeishuCardColor::Turquoise400 => serializer.serialize_str("turquoise-400"),
            FeishuCardColor::Turquoise500 => serializer.serialize_str("turquoise-500"),
            FeishuCardColor::Turquoise600 => serializer.serialize_str("turquoise-600"),
            FeishuCardColor::Turquoise700 => serializer.serialize_str("turquoise-700"),
            FeishuCardColor::Turquoise800 => serializer.serialize_str("turquoise-800"),
            FeishuCardColor::Turquoise900 => serializer.serialize_str("turquoise-900"),
            FeishuCardColor::Violet => serializer.serialize_str("violet"),
            FeishuCardColor::Violet50 => serializer.serialize_str("violet-50"),
            FeishuCardColor::Violet100 => serializer.serialize_str("violet-100"),
            FeishuCardColor::Violet200 => serializer.serialize_str("violet-200"),
            FeishuCardColor::Violet300 => serializer.serialize_str("violet-300"),
            FeishuCardColor::Violet350 => serializer.serialize_str("violet-350"),
            FeishuCardColor::Violet400 => serializer.serialize_str("violet-400"),
            FeishuCardColor::Violet500 => serializer.serialize_str("violet-500"),
            FeishuCardColor::Violet600 => serializer.serialize_str("violet-600"),
            FeishuCardColor::Violet700 => serializer.serialize_str("violet-700"),
            FeishuCardColor::Violet800 => serializer.serialize_str("violet-800"),
            FeishuCardColor::Violet900 => serializer.serialize_str("violet-900"),
            FeishuCardColor::Wathet => serializer.serialize_str("wathet"),
            FeishuCardColor::Wathet50 => serializer.serialize_str("wathet-50"),
            FeishuCardColor::Wathet100 => serializer.serialize_str("wathet-100"),
            FeishuCardColor::Wathet200 => serializer.serialize_str("wathet-200"),
            FeishuCardColor::Wathet300 => serializer.serialize_str("wathet-300"),
            FeishuCardColor::Wathet350 => serializer.serialize_str("wathet-350"),
            FeishuCardColor::Wathet400 => serializer.serialize_str("wathet-400"),
            FeishuCardColor::Wathet500 => serializer.serialize_str("wathet-500"),
            FeishuCardColor::Wathet600 => serializer.serialize_str("wathet-600"),
            FeishuCardColor::Wathet700 => serializer.serialize_str("wathet-700"),
            FeishuCardColor::Wathet800 => serializer.serialize_str("wathet-800"),
            FeishuCardColor::Wathet900 => serializer.serialize_str("wathet-900"),
            FeishuCardColor::Yellow => serializer.serialize_str("yellow"),
            FeishuCardColor::Yellow50 => serializer.serialize_str("yellow-50"),
            FeishuCardColor::Yellow100 => serializer.serialize_str("yellow-100"),
            FeishuCardColor::Yellow200 => serializer.serialize_str("yellow-200"),
            FeishuCardColor::Yellow300 => serializer.serialize_str("yellow-300"),
            FeishuCardColor::Yellow350 => serializer.serialize_str("yellow-350"),
            FeishuCardColor::Yellow400 => serializer.serialize_str("yellow-400"),
            FeishuCardColor::Yellow500 => serializer.serialize_str("yellow-500"),
            FeishuCardColor::Yellow600 => serializer.serialize_str("yellow-600"),
            FeishuCardColor::Yellow700 => serializer.serialize_str("yellow-700"),
            FeishuCardColor::Yellow800 => serializer.serialize_str("yellow-800"),
            FeishuCardColor::Yellow900 => serializer.serialize_str("yellow-900"),
        }
    }
}

impl<'de> Deserialize<'de> for FeishuCardColor {
    fn deserialize<D>(deserializer: D) -> Result<FeishuCardColor, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "bg-white" => Ok(FeishuCardColor::BgWhite),
            "white" => Ok(FeishuCardColor::White),
            "blue" => Ok(FeishuCardColor::Blue),
            "blue-50" => Ok(FeishuCardColor::Blue50),
            "blue-100" => Ok(FeishuCardColor::Blue100),
            "blue-200" => Ok(FeishuCardColor::Blue200),
            "blue-300" => Ok(FeishuCardColor::Blue300),
            "blue-350" => Ok(FeishuCardColor::Blue350),
            "blue-400" => Ok(FeishuCardColor::Blue400),
            "blue-500" => Ok(FeishuCardColor::Blue500),
            "blue-600" => Ok(FeishuCardColor::Blue600),
            "blue-700" => Ok(FeishuCardColor::Blue700),
            "blue-800" => Ok(FeishuCardColor::Blue800),
            "blue-900" => Ok(FeishuCardColor::Blue900),
            "carmine" => Ok(FeishuCardColor::Carmine),
            "carmine-50" => Ok(FeishuCardColor::Carmine50),
            "carmine-100" => Ok(FeishuCardColor::Carmine100),
            "carmine-200" => Ok(FeishuCardColor::Carmine200),
            "carmine-300" => Ok(FeishuCardColor::Carmine300),
            "carmine-350" => Ok(FeishuCardColor::Carmine350),
            "carmine-400" => Ok(FeishuCardColor::Carmine400),
            "carmine-500" => Ok(FeishuCardColor::Carmine500),
            "carmine-600" => Ok(FeishuCardColor::Carmine600),
            "carmine-700" => Ok(FeishuCardColor::Carmine700),
            "carmine-800" => Ok(FeishuCardColor::Carmine800),
            "carmine-900" => Ok(FeishuCardColor::Carmine900),
            "green" => Ok(FeishuCardColor::Green),
            "green-50" => Ok(FeishuCardColor::Green50),
            "green-100" => Ok(FeishuCardColor::Green100),
            "green-200" => Ok(FeishuCardColor::Green200),
            "green-300" => Ok(FeishuCardColor::Green300),
            "green-350" => Ok(FeishuCardColor::Green350),
            "green-400" => Ok(FeishuCardColor::Green400),
            "green-500" => Ok(FeishuCardColor::Green500),
            "green-600" => Ok(FeishuCardColor::Green600),
            "green-700" => Ok(FeishuCardColor::Green700),
            "green-800" => Ok(FeishuCardColor::Green800),
            "green-900" => Ok(FeishuCardColor::Green900),
            "indigo" => Ok(FeishuCardColor::Indigo),
            "indigo-50" => Ok(FeishuCardColor::Indigo50),
            "indigo-100" => Ok(FeishuCardColor::Indigo100),
            "indigo-200" => Ok(FeishuCardColor::Indigo200),
            "indigo-300" => Ok(FeishuCardColor::Indigo300),
            "indigo-350" => Ok(FeishuCardColor::Indigo350),
            "indigo-400" => Ok(FeishuCardColor::Indigo400),
            "indigo-500" => Ok(FeishuCardColor::Indigo500),
            "indigo-600" => Ok(FeishuCardColor::Indigo600),
            "indigo-700" => Ok(FeishuCardColor::Indigo700),
            "indigo-800" => Ok(FeishuCardColor::Indigo800),
            "indigo-900" => Ok(FeishuCardColor::Indigo900),
            "lime" => Ok(FeishuCardColor::Lime),
            "lime-50" => Ok(FeishuCardColor::Lime50),
            "lime-100" => Ok(FeishuCardColor::Lime100),
            "lime-200" => Ok(FeishuCardColor::Lime200),
            "lime-300" => Ok(FeishuCardColor::Lime300),
            "lime-350" => Ok(FeishuCardColor::Lime350),
            "lime-400" => Ok(FeishuCardColor::Lime400),
            "lime-500" => Ok(FeishuCardColor::Lime500),
            "lime-600" => Ok(FeishuCardColor::Lime600),
            "lime-700" => Ok(FeishuCardColor::Lime700),
            "lime-800" => Ok(FeishuCardColor::Lime800),
            "lime-900" => Ok(FeishuCardColor::Lime900),
            "grey" => Ok(FeishuCardColor::Grey),
            "grey-50" => Ok(FeishuCardColor::Grey50),
            "grey-100" => Ok(FeishuCardColor::Grey100),
            "grey-200" => Ok(FeishuCardColor::Grey200),
            "grey-300" => Ok(FeishuCardColor::Grey300),
            "grey-350" => Ok(FeishuCardColor::Grey350),
            "grey-400" => Ok(FeishuCardColor::Grey400),
            "grey-500" => Ok(FeishuCardColor::Grey500),
            "grey-600" => Ok(FeishuCardColor::Grey600),
            "grey-700" => Ok(FeishuCardColor::Grey700),
            "grey-800" => Ok(FeishuCardColor::Grey800),
            "grey-900" => Ok(FeishuCardColor::Grey900),
            "orange" => Ok(FeishuCardColor::Orange),
            "orange-50" => Ok(FeishuCardColor::Orange50),
            "orange-100" => Ok(FeishuCardColor::Orange100),
            "orange-200" => Ok(FeishuCardColor::Orange200),
            "orange-300" => Ok(FeishuCardColor::Orange300),
            "orange-350" => Ok(FeishuCardColor::Orange350),
            "orange-400" => Ok(FeishuCardColor::Orange400),
            "orange-500" => Ok(FeishuCardColor::Orange500),
            "orange-600" => Ok(FeishuCardColor::Orange600),
            "orange-700" => Ok(FeishuCardColor::Orange700),
            "orange-800" => Ok(FeishuCardColor::Orange800),
            "orange-900" => Ok(FeishuCardColor::Orange900),
            "purple" => Ok(FeishuCardColor::Purple),
            "purple-50" => Ok(FeishuCardColor::Purple50),
            "purple-100" => Ok(FeishuCardColor::Purple100),
            "purple-200" => Ok(FeishuCardColor::Purple200),
            "purple-300" => Ok(FeishuCardColor::Purple300),
            "purple-350" => Ok(FeishuCardColor::Purple350),
            "purple-400" => Ok(FeishuCardColor::Purple400),
            "purple-500" => Ok(FeishuCardColor::Purple500),
            "purple-600" => Ok(FeishuCardColor::Purple600),
            "purple-700" => Ok(FeishuCardColor::Purple700),
            "purple-800" => Ok(FeishuCardColor::Purple800),
            "purple-900" => Ok(FeishuCardColor::Purple900),
            "red" => Ok(FeishuCardColor::Red),
            "red-50" => Ok(FeishuCardColor::Red50),
            "red-100" => Ok(FeishuCardColor::Red100),
            "red-200" => Ok(FeishuCardColor::Red200),
            "red-300" => Ok(FeishuCardColor::Red300),
            "red-350" => Ok(FeishuCardColor::Red350),
            "red-400" => Ok(FeishuCardColor::Red400),
            "red-500" => Ok(FeishuCardColor::Red500),
            "red-600" => Ok(FeishuCardColor::Red600),
            "red-700" => Ok(FeishuCardColor::Red700),
            "red-800" => Ok(FeishuCardColor::Red800),
            "red-900" => Ok(FeishuCardColor::Red900),
            "sunflower" => Ok(FeishuCardColor::Sunflower),
            "sunflower-50" => Ok(FeishuCardColor::Sunflower50),
            "sunflower-100" => Ok(FeishuCardColor::Sunflower100),
            "sunflower-200" => Ok(FeishuCardColor::Sunflower200),
            "sunflower-300" => Ok(FeishuCardColor::Sunflower300),
            "sunflower-350" => Ok(FeishuCardColor::Sunflower350),
            "sunflower-400" => Ok(FeishuCardColor::Sunflower400),
            "sunflower-500" => Ok(FeishuCardColor::Sunflower500),
            "sunflower-600" => Ok(FeishuCardColor::Sunflower600),
            "sunflower-700" => Ok(FeishuCardColor::Sunflower700),
            "sunflower-800" => Ok(FeishuCardColor::Sunflower800),
            "sunflower-900" => Ok(FeishuCardColor::Sunflower900),
            "turquoise" => Ok(FeishuCardColor::Turquoise),
            "turquoise-50" => Ok(FeishuCardColor::Turquoise50),
            "turquoise-100" => Ok(FeishuCardColor::Turquoise100),
            "turquoise-200" => Ok(FeishuCardColor::Turquoise200),
            "turquoise-300" => Ok(FeishuCardColor::Turquoise300),
            "turquoise-350" => Ok(FeishuCardColor::Turquoise350),
            "turquoise-400" => Ok(FeishuCardColor::Turquoise400),
            "turquoise-500" => Ok(FeishuCardColor::Turquoise500),
            "turquoise-600" => Ok(FeishuCardColor::Turquoise600),
            "turquoise-700" => Ok(FeishuCardColor::Turquoise700),
            "turquoise-800" => Ok(FeishuCardColor::Turquoise800),
            "turquoise-900" => Ok(FeishuCardColor::Turquoise900),
            "violet" => Ok(FeishuCardColor::Violet),
            "violet-50" => Ok(FeishuCardColor::Violet50),
            "violet-100" => Ok(FeishuCardColor::Violet100),
            "violet-200" => Ok(FeishuCardColor::Violet200),
            "violet-300" => Ok(FeishuCardColor::Violet300),
            "violet-350" => Ok(FeishuCardColor::Violet350),
            "violet-400" => Ok(FeishuCardColor::Violet400),
            "violet-500" => Ok(FeishuCardColor::Violet500),
            "violet-600" => Ok(FeishuCardColor::Violet600),
            "violet-700" => Ok(FeishuCardColor::Violet700),
            "violet-800" => Ok(FeishuCardColor::Violet800),
            "violet-900" => Ok(FeishuCardColor::Violet900),
            "wathet" => Ok(FeishuCardColor::Wathet),
            "wathet-50" => Ok(FeishuCardColor::Wathet50),
            "wathet-100" => Ok(FeishuCardColor::Wathet100),
            "wathet-200" => Ok(FeishuCardColor::Wathet200),
            "wathet-300" => Ok(FeishuCardColor::Wathet300),
            "wathet-350" => Ok(FeishuCardColor::Wathet350),
            "wathet-400" => Ok(FeishuCardColor::Wathet400),
            "wathet-500" => Ok(FeishuCardColor::Wathet500),
            "wathet-600" => Ok(FeishuCardColor::Wathet600),
            "wathet-700" => Ok(FeishuCardColor::Wathet700),
            "wathet-800" => Ok(FeishuCardColor::Wathet800),
            "wathet-900" => Ok(FeishuCardColor::Wathet900),
            "yellow" => Ok(FeishuCardColor::Yellow),
            "yellow-50" => Ok(FeishuCardColor::Yellow50),
            "yellow-100" => Ok(FeishuCardColor::Yellow100),
            "yellow-200" => Ok(FeishuCardColor::Yellow200),
            "yellow-300" => Ok(FeishuCardColor::Yellow300),
            "yellow-350" => Ok(FeishuCardColor::Yellow350),
            "yellow-400" => Ok(FeishuCardColor::Yellow400),
            "yellow-500" => Ok(FeishuCardColor::Yellow500),
            "yellow-600" => Ok(FeishuCardColor::Yellow600),
            "yellow-700" => Ok(FeishuCardColor::Yellow700),
            "yellow-800" => Ok(FeishuCardColor::Yellow800),
            "yellow-900" => Ok(FeishuCardColor::Yellow900),
            _ => Ok(FeishuCardColor::Custom(s)),
        }
    }
}
