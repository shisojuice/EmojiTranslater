mod emoji_json;

use std::string::String;
use wasm_bindgen::prelude::*;
use serde_json::{from_str};
use serde::{Deserialize, Serialize};
use crate::emoji_json::get_json;
use unicode_segmentation::UnicodeSegmentation;
use lindera::{DictionaryConfig, DictionaryKind, Mode, Tokenizer, TokenizerConfig};


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone, Deserialize, Serialize, Debug)]
struct EmojiRow {
    cd: String,
    nm: String,
    jp: String,
}

//
// # emoji=>short_nameのMainとなる関数
//

/// Body要素内のTextContentのemojiをshort_nameに翻訳して返します
#[wasm_bindgen]
pub fn translate_emoji_into_name(language: u32,explain: bool){
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    get_element_text_content_to_name(&body,language,explain);
}

/// ループでhtml内のtextを探索します
fn get_element_text_content_to_name(element: &web_sys::Element,language: u32,explain: bool) {
    let children = element.children();
    for i in 0..children.length() {
        let child = children.item(i).unwrap();
        if child.children().length() == 0 {
            if let Some(text) = child.text_content() {
                child.set_text_content(Some(&*replace_emoji_with_name(&*text,language,explain)));
            }
        }
        get_element_text_content_to_name(&child,language,explain);
    }
}

/// text内のemojiをshort_nameに入替して返します
#[wasm_bindgen]
pub fn replace_emoji_with_name(text: &str,language: u32,explain: bool) -> String {
    text.graphemes(true)
        .map(|g| {
            if g.chars().all(|c| (0x1F000..=0x1F9FF).contains(&(c as u32))) {
                //単体絵文字
                get_short_name(g, language,explain)
            } else {
                if g.chars().count() > 1 {
                    if let Some(c) = g.chars().next() {
                        if (0x1F000..=0x1F9FF).contains(&(c as u32)) {
                            //合体絵文字
                            get_short_name(g, language,explain)
                        } else { g.to_string() }
                    } else { g.to_string() }
                } else { g.to_string() }
            }
        })
        .collect()
}

/// emoji_unicodeに紐づくshort_nameにして返します(explainの時は絵文字直後に説明を入れる)
fn get_short_name(emoji :&str,language: u32,explain: bool) -> String {
    let mut result = String::new();
    let unicode = format_emoji_to_unicode(emoji);

    let emoji_rows: Vec<EmojiRow> = from_str(get_json()).unwrap();
    let objs = emoji_rows.iter()
        .filter(|item| item.cd.as_str() == unicode);
    for obj in objs {
        if language == 0 {
            result= obj.clone().nm;
        }
        if language == 1 {
            result= obj.clone().jp;
        }
        break;
    }

    if explain {
        let mut result_explain = String::new();
        result_explain.push_str(emoji);
        result_explain.push_str(&* format!("(*{}*)",result));
        result_explain
    }
    else {
        format!("(*{}*)",result)
    }
}

/// emojiをunicodeにformatして返します
fn format_emoji_to_unicode(emoji :&str) -> String {
    let mut unicode = String::new();
    let mut i = 0;
    for c in emoji.chars()
    {
        if i>0 {unicode.push_str(" ")};
        unicode.push_str(&format!("U+{:X}", c as u32));
        i = i + 1;
    }
    unicode
}

//
// # short_name=>emojiのMainとなる関数
//

/// Body要素内のTextContentのshort_nameをemojiに翻訳して返します
#[wasm_bindgen]
pub fn translate_name_into_emoji(explain: bool){
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    get_element_text_content_to_emoji(&body,explain);
}

/// ループでhtml内のtextを探索します
fn get_element_text_content_to_emoji(element: &web_sys::Element,explain: bool) {
    let children = element.children();
    for i in 0..children.length() {
        let child = children.item(i).unwrap();
        if child.children().length() == 0 {
            if let Some(text) = child.text_content() {
                child.set_text_content(Some(&*replace_name_with_emoji(&*text,explain)));
            }
        }
        get_element_text_content_to_emoji(&child,explain);
    }
}

/// text内のshort_name(類似名含む名詞単位)をemojiに入替して返します
#[wasm_bindgen]
pub fn replace_name_with_emoji(text: &str,explain: bool) -> String {
    let mut result = String::new();
    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };
    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };
    let tokenizer = Tokenizer::from_config(config).unwrap();
    let tokens = tokenizer.tokenize(text).unwrap();
    for token in tokens {
        result.push_str(&*get_emoji(token.text,explain));
    }
    result
}

/// short_name(類似名含む名詞単位)に紐づくemojiにして返します
fn get_emoji(text: &str,explain: bool) -> String {
    let mut result = String::new();
    let emoji_rows: Vec<EmojiRow> = from_str(get_json()).unwrap();
    for row in emoji_rows {
       // short_name
       if row.nm == text {
           if explain { result.push_str(text); }
           result.push_str(&*format_unicode_to_emoji(&*row.cd));
           break;
       }
       // short_name_jp
       if row.jp == text {
           if explain { result.push_str(text); }
            result.push_str(&*format_unicode_to_emoji(&*row.cd));
            break;
       }
    }
    if result.is_empty() {
        result.push_str(text);
    }
    result
}

/// unicodeをemojiにformatして返します
fn format_unicode_to_emoji(unicode: &str) -> String {
    let result
        = unicode.split_whitespace().map(|code| {
            if let Some(stripped) = code.strip_prefix("U+") {
                if let Ok(hex) = u32::from_str_radix(stripped, 16) {
                    char::from_u32(hex).unwrap_or_default().to_string()
                } else {
                    code.to_string()
                }
            } else {
                code.to_string()
            }
        })
        .collect::<String>();
    result
}