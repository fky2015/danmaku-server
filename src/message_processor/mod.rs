use crate::actor_models::session::Danmaku;

#[derive(Default, Debug)]
pub struct MessageProcessor {}

impl MessageProcessor {
    pub(crate) fn process(&mut self, danmaku: Danmaku) -> Result<Danmaku, String> {
        let new_text = text_constraint(danmaku.text);
        if new_text.is_err() {
            return Err(new_text.expect_err("process: something wrong!"));
        }
        Ok(Danmaku {
            user: danmaku.user,
            text: new_text.unwrap(),
            color: danmaku.color,
            r#type: danmaku.r#type,
        })
    }
}

fn text_constraint(text: String) -> Result<String, String> {
    if text.chars().count() <= 30 {
        if text.trim().is_empty() {
            Err("can't be empty.".to_owned())
        } else {
            Ok(text)
        }
    } else {
        Err("Danmaku Length exceeds length limit.".to_owned())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_text() {
        assert_eq!(text_constraint("   ".to_owned()).is_err(), true);
        assert_eq!(text_constraint("啦啦".to_owned()).is_err(), false);
        assert_eq!(text_constraint(" ".repeat(40).to_owned()).is_err(), true);
        assert_eq!(text_constraint(" ".repeat(29).to_owned()).is_err(), true);
        assert_eq!(text_constraint("哦".repeat(30).to_owned()).is_err(), false)
    }
}
