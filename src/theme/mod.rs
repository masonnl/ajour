use de::deserialize_color_hex_string;
use serde::Deserialize;

#[derive(Clone, Copy, Deserialize)]
pub struct Theme {
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub primary: iced::Color,
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub secondary: iced::Color,
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub surface: iced::Color,
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub on_surface: iced::Color,
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub background: iced::Color,
    #[serde(deserialize_with = "deserialize_color_hex_string")]
    pub error: iced::Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            primary: iced::Color::from_rgb(0.73, 0.52, 0.99),
            secondary: iced::Color::from_rgb(0.88, 0.74, 0.28),
            surface: iced::Color::from_rgb(0.12, 0.12, 0.12),
            on_surface: iced::Color::from_rgb(0.88, 0.88, 0.88),
            background: iced::Color::from_rgb(0.07, 0.07, 0.07),
            error: iced::Color::from_rgb(0.76, 0.19, 0.28),
        }
    }
}

// Newtype on iced::Color so we can impl Deserialzer for it
struct Color(iced::Color);

mod de {
    use super::Color;
    use serde::de::{self, Error, Unexpected, Visitor};
    use std::fmt;

    pub fn deserialize_color_hex_string<'de, D>(deserializer: D) -> Result<iced::Color, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct ColorVisitor;

        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = Color;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a hex string in the format of '#09ACDF'")
            }

            #[allow(clippy::unnecessary_unwrap)]
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                if s.len() == 7 {
                    let hash = &s[0..1];
                    let r = u8::from_str_radix(&s[1..3], 16);
                    let g = u8::from_str_radix(&s[3..5], 16);
                    let b = u8::from_str_radix(&s[5..7], 16);

                    if hash == "#" && r.is_ok() && g.is_ok() && b.is_ok() {
                        return Ok(Color(iced::Color {
                            r: r.unwrap() as f32 / 255.0,
                            g: g.unwrap() as f32 / 255.0,
                            b: b.unwrap() as f32 / 255.0,
                            a: 1.0,
                        }));
                    }
                }

                Err(de::Error::invalid_value(Unexpected::Str(s), &self))
            }
        }

        deserializer.deserialize_any(ColorVisitor).map(|c| c.0)
    }
}

#[cfg(test)]
mod tests {
    use super::de::deserialize_color_hex_string;
    use serde::de::value::{Error, StrDeserializer};
    use serde::de::IntoDeserializer;

    #[test]
    fn test_hex_color_deser() {
        let colors = [
            "AABBCC", "AABBCG", "#AABBCG", "#AABB091", "#AABBCC", "#AABB09",
        ];

        for (idx, color_str) in colors.iter().enumerate() {
            let deserializer: StrDeserializer<Error> = color_str.into_deserializer();

            let color = deserialize_color_hex_string(deserializer);

            if idx < 4 {
                assert!(color.is_err());
            } else {
                assert!(color.is_ok());
            }
        }
    }
}
