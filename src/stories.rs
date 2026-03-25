use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CharAnnotation {
    pub ch: String,
    pub pinyin: String,
    pub english: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExtraWord {
    pub hanzi: String,
    pub pinyin: String,
    pub english: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct StoryParagraph {
    pub chars: Vec<CharAnnotation>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Story {
    pub id: u32,
    pub title: String,
    pub title_cn: String,
    pub description: String,
    pub extra_vocab: Vec<ExtraWord>,
    pub paragraphs: Vec<StoryParagraph>,
}

fn p(text: &str) -> String { text.to_string() }

fn ch(ch: &str, pinyin: &str, english: &str) -> CharAnnotation {
    CharAnnotation {
        ch: ch.into(),
        pinyin: pinyin.into(),
        english: english.into(),
    }
}

fn para(chars: Vec<CharAnnotation>) -> StoryParagraph {
    StoryParagraph { chars }
}

fn extra(hanzi: &str, pinyin: &str, english: &str) -> ExtraWord {
    ExtraWord {
        hanzi: hanzi.into(),
        pinyin: pinyin.into(),
        english: english.into(),
    }
}

pub fn get_stories_meta() -> Vec<serde_json::Value> {
    get_all_stories()
        .iter()
        .map(|s| {
            serde_json::json!({
                "id": s.id,
                "title": s.title,
                "title_cn": s.title_cn,
                "description": s.description,
            })
        })
        .collect()
}

pub fn get_all_stories() -> Vec<Story> {
    vec![
        story_monkey_king(),
        story_mulan(),
        story_nian(),
        story_cowherd(),
        story_chang_e(),
    ]
}

fn story_monkey_king() -> Story {
    Story {
        id: 1,
        title: p("The Monkey King"),
        title_cn: p("美猴王"),
        description: p("Sun Wukong is born from a stone and becomes king of the monkeys. Simplified from Journey to the West."),
        extra_vocab: vec![
            extra("猴", "hóu", "monkey"),
            extra("王", "wáng", "king"),
            extra("石头", "shítou", "stone, rock"),
            extra("山", "shān", "mountain"),
            extra("海", "hǎi", "sea, ocean"),
            extra("瀑布", "pùbù", "waterfall"),
            extra("洞", "dòng", "cave"),
            extra("跳", "tiào", "to jump"),
            extra("勇敢", "yǒnggǎn", "brave"),
            extra("成", "chéng", "to become"),
        ],
        paragraphs: vec![
            para(vec![
                ch("很", "hěn", "very"), ch("久", "jiǔ", "long time"), ch("很", "hěn", "very"), ch("久", "jiǔ", "long time"),
                ch("以", "yǐ", "before"), ch("前", "qián", "before"), ch("，", ",", ","),
                ch("在", "zài", "at"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("大", "dà", "big"), ch("山", "shān", "mountain"), ch("上", "shàng", "on"),  ch("，", ",", ","),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("大", "dà", "big"), ch("石", "shí", "stone"), ch("头", "tou", "stone"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("石", "shí", "stone"), ch("头", "tou", "stone"),
                ch("里", "lǐ", "inside"), ch("出", "chū", "come out"), ch("来", "lái", "come"),
                ch("了", "le", "completed"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("小", "xiǎo", "small"), ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("这", "zhè", "this"), ch("个", "gè", "measure word"),
                ch("小", "xiǎo", "small"), ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"),
                ch("很", "hěn", "very"), ch("高", "gāo", "tall"), ch("兴", "xìng", "happy"), ch("。", ".", "."),
                ch("他", "tā", "he"), ch("看", "kàn", "to look"),
                ch("山", "shān", "mountain"), ch("，", ",", ","),
                ch("看", "kàn", "to look"), ch("水", "shuǐ", "water"), ch("，", ",", ","),
                ch("看", "kàn", "to look"), ch("花", "huā", "flower"), ch("，", ",", ","),
                ch("看", "kàn", "to look"), ch("树", "shù", "tree"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("他", "tā", "he"), ch("去", "qù", "to go"),
                ch("找", "zhǎo", "to look for"),
                ch("别", "bié", "other"), ch("的", "de", "possessive"),
                ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"), ch("。", ".", "."),
                ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("喜", "xǐ", "like"), ch("欢", "huān", "like"),
                ch("他", "tā", "him"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"), ch("们", "men", "plural"),
                ch("找", "zhǎo", "to find"), ch("到", "dào", "arrive"),
                ch("了", "le", "completed"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("大", "dà", "big"), ch("瀑", "pù", "waterfall"), ch("布", "bù", "waterfall"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("谁", "shéi", "who"), ch("能", "néng", "can"),
                ch("跳", "tiào", "jump"), ch("进", "jìn", "enter"),
                ch("去", "qù", "go"), ch("？", "?", "?"),
                ch("谁", "shéi", "who"), ch("就", "jiù", "then"),
                ch("是", "shì", "is"),
                ch("我", "wǒ", "our"), ch("们", "men", "plural"), ch("的", "de", "possessive"),
                ch("王", "wáng", "king"), ch("！", "!", "!"),
            ]),
            para(vec![
                ch("小", "xiǎo", "small"), ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"),
                ch("很", "hěn", "very"), ch("勇", "yǒng", "brave"), ch("敢", "gǎn", "brave"), ch("。", ".", "."),
                ch("他", "tā", "he"), ch("跳", "tiào", "jump"),
                ch("进", "jìn", "enter"), ch("了", "le", "completed"),
                ch("瀑", "pù", "waterfall"), ch("布", "bù", "waterfall"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("里", "lǐ", "inside"), ch("面", "miàn", "side"),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("很", "hěn", "very"), ch("大", "dà", "big"), ch("的", "de", "possessive"),
                ch("洞", "dòng", "cave"), ch("。", ".", "."),
                ch("有", "yǒu", "there is"),
                ch("桌", "zhuō", "table"), ch("子", "zi", "suffix"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"),
                ch("椅", "yǐ", "chair"), ch("子", "zi", "suffix"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"),
                ch("水", "shuǐ", "water"), ch("果", "guǒ", "fruit"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("小", "xiǎo", "small"), ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"),
                ch("出", "chū", "go out"), ch("来", "lái", "come"),
                ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"), ch("里", "lǐ", "inside"), ch("面", "miàn", "side"),
                ch("很", "hěn", "very"), ch("好", "hǎo", "good"), ch("！", "!", "!"),
                ch("我", "wǒ", "our"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("可", "kě", "can"), ch("以", "yǐ", "can"),
                ch("住", "zhù", "to live"), ch("在", "zài", "at"),
                ch("那", "nà", "there"), ch("儿", "r", "there"), ch("！", "!", "!"), ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("猴", "hóu", "monkey"), ch("子", "zi", "suffix"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("高", "gāo", "happy"), ch("兴", "xìng", "happy"), ch("。", ".", "."),
                ch("他", "tā", "they"), ch("们", "men", "plural"),
                ch("叫", "jiào", "to call"), ch("他", "tā", "him"),
                ch("“", "“", "quote"), ch("美", "měi", "beautiful"),
                ch("猴", "hóu", "monkey"), ch("王", "wáng", "king"), ch("“", "“", "quote"), ch("。", ".", "."),
            ]),
        ],
    }
}

fn story_mulan() -> Story {
    Story {
        id: 2,
        title: p("Mulan Goes to War"),
        title_cn: p("花木兰"),
        description: p("A young woman takes her father's place in the army. Simplified from the Ballad of Mulan."),
        extra_vocab: vec![
            extra("木兰", "Mùlán", "Mulan (name)"),
            extra("军", "jūn", "army"),
            extra("战", "zhàn", "war, battle"),
            extra("马", "mǎ", "horse"),
            extra("父亲", "fùqīn", "father"),
            extra("勇敢", "yǒnggǎn", "brave"),
            extra("将军", "jiāngjūn", "general"),
            extra("打仗", "dǎzhàng", "to fight a war"),
            extra("回来", "huílái", "to come back"),
        ],
        paragraphs: vec![
            para(vec![
                ch("很", "hěn", "very"), ch("久", "jiǔ", "long time"),
                ch("以", "yǐ", "before"), ch("前", "qián", "before"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("女", "nǚ", "female"), ch("孩", "hái", "child"), ch("子", "zi", "suffix"), ch("，", ",", ","),
                ch("她", "tā", "she"), ch("叫", "jiào", "called"),
                ch("木", "Mù", "Mulan"), ch("兰", "lán", "Mulan"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("木", "Mù", "Mulan"), ch("兰", "lán", "Mulan"), ch("的", "de", "possessive"),
                ch("父", "fù", "father"), ch("亲", "qīn", "father"),
                ch("很", "hěn", "very"), ch("老", "lǎo", "old"),
                ch("了", "le", "already"), ch("，", ",", ","),
                ch("身", "shēn", "body"), ch("体", "tǐ", "body"),
                ch("不", "bù", "not"), ch("好", "hǎo", "good"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("将", "jiāng", "general"), ch("军", "jūn", "general"),
                ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"),
                ch("每", "měi", "every"), ch("家", "jiā", "family"),
                ch("要", "yào", "must"),
                ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("人", "rén", "person"),
                ch("去", "qù", "to go"), ch("打", "dǎ", "to fight"),
                ch("仗", "zhàng", "war"), ch("。", ".", "."), ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("木", "Mù", "Mulan"), ch("兰", "lán", "Mulan"),
                ch("对", "duì", "to"), ch("父", "fù", "father"), ch("亲", "qīn", "father"),
                ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"),
                ch("爸", "bà", "dad"), ch("爸", "ba", "dad"), ch("，", ",", ","),
                ch("你", "nǐ", "you"), ch("不", "bù", "not"),
                ch("能", "néng", "can"), ch("去", "qù", "to go"), ch("。", ".", "."),
                ch("我", "wǒ", "I"), ch("去", "qù", "to go"), ch("！", "!", "!"),
                ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("她", "tā", "she"), ch("穿", "chuān", "to wear"),
                ch("了", "le", "completed"),
                ch("父", "fù", "father"), ch("亲", "qīn", "father"), ch("的", "de", "possessive"),
                ch("衣", "yī", "clothes"), ch("服", "fu", "clothes"), ch("，", ",", ","),
                ch("买", "mǎi", "to buy"), ch("了", "le", "completed"),
                ch("一", "yī", "one"), ch("匹", "pǐ", "measure word"),
                ch("马", "mǎ", "horse"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("木", "Mù", "Mulan"), ch("兰", "lán", "Mulan"),
                ch("很", "hěn", "very"), ch("勇", "yǒng", "brave"), ch("敢", "gǎn", "brave"), ch("。", ".", "."),
                ch("她", "tā", "she"), ch("打", "dǎ", "to fight"),
                ch("了", "le", "completed"),
                ch("很", "hěn", "very"), ch("多", "duō", "many"),
                ch("年", "nián", "year"), ch("的", "de", "possessive"),
                ch("仗", "zhàng", "war"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("战", "zhàn", "war"), ch("争", "zhēng", "war"),
                ch("完", "wán", "finished"), ch("了", "le", "completed"), ch("。", ".", "."),
                ch("将", "jiāng", "general"), ch("军", "jūn", "general"),
                ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"),
                ch("你", "nǐ", "you"), ch("想", "xiǎng", "to want"),
                ch("要", "yào", "to want"), ch("什", "shén", "what"), ch("么", "me", "what"), ch("？", "?", "?"),
                ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("木", "Mù", "Mulan"), ch("兰", "lán", "Mulan"),
                ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"),
                ch("我", "wǒ", "I"), ch("想", "xiǎng", "to want"),
                ch("回", "huí", "to return"), ch("家", "jiā", "home"), ch("。", ".", "."),
                ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("她", "tā", "she"), ch("回", "huí", "to return"),
                ch("到", "dào", "arrive"), ch("了", "le", "completed"),
                ch("家", "jiā", "home"), ch("，", ",", ","),
                ch("穿", "chuān", "to wear"), ch("了", "le", "completed"),
                ch("漂", "piào", "beautiful"), ch("亮", "liang", "beautiful"),
                ch("的", "de", "possessive"), ch("衣", "yī", "clothes"), ch("服", "fu", "clothes"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("朋", "péng", "friend"), ch("友", "yǒu", "friend"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("高", "gāo", "happy"), ch("兴", "xìng", "happy"),
                ch("她", "tā", "she"), ch("回", "huí", "return"),
                ch("来", "lái", "come"), ch("了", "le", "completed"), ch("。", ".", "."),
            ]),
        ],
    }
}

fn story_nian() -> Story {
    Story {
        id: 3,
        title: p("The Monster Nian"),
        title_cn: p("年兽"),
        description: p("Why Chinese New Year has fireworks and red decorations. A classic folk tale."),
        extra_vocab: vec![
            extra("兽", "shòu", "beast, monster"),
            extra("怕", "pà", "to be afraid of"),
            extra("红色", "hóngsè", "red color"),
            extra("声音", "shēngyīn", "sound"),
            extra("村子", "cūnzi", "village"),
            extra("跑", "pǎo", "to run"),
            extra("放", "fàng", "to set off"),
            extra("鞭炮", "biānpào", "firecrackers"),
            extra("过年", "guònián", "celebrate New Year"),
        ],
        paragraphs: vec![
            para(vec![
                ch("很", "hěn", "very"), ch("久", "jiǔ", "long time"),
                ch("以", "yǐ", "before"), ch("前", "qián", "before"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("大", "dà", "big"), ch("兽", "shòu", "beast"), ch("，", ",", ","),
                ch("他", "tā", "it"), ch("叫", "jiào", "called"),
                ch("“", "“", "quote"), ch("年", "nián", "Nian"), ch("“", "“", "quote"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("冬", "dōng", "winter"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("年", "nián", "Nian"),
                ch("都", "dōu", "always"),
                ch("来", "lái", "to come"),
                ch("村", "cūn", "village"), ch("子", "zi", "suffix"), ch("里", "lǐ", "inside"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("怕", "pà", "afraid"), ch("他", "tā", "it"), ch("。", ".", "."),
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("他", "tā", "they"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"),
                ch("跑", "pǎo", "to run"),
                ch("到", "dào", "to"),
                ch("山", "shān", "mountain"), ch("上", "shàng", "on"),
                ch("去", "qù", "to go"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("老", "lǎo", "old"), ch("人", "rén", "person"),
                ch("来", "lái", "to come"), ch("了", "le", "completed"), ch("。", ".", "."),
                ch("他", "tā", "he"), ch("说", "shuō", "to say"), ch("：", ":", ":"),
                ch("“", "“", "quote"),
                ch("我", "wǒ", "I"), ch("知", "zhī", "to know"), ch("道", "dào", "to know"),
                ch("年", "nián", "Nian"),
                ch("怕", "pà", "afraid of"),
                ch("什", "shén", "what"), ch("么", "me", "what"), ch("。", ".", "."),
                ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("“", "“", "quote"),
                ch("年", "nián", "Nian"), ch("怕", "pà", "afraid of"),
                ch("红", "hóng", "red"), ch("色", "sè", "color"), ch("，", ",", ","),
                ch("怕", "pà", "afraid of"),
                ch("大", "dà", "big"), ch("的", "de", "possessive"),
                ch("声", "shēng", "sound"), ch("音", "yīn", "sound"), ch("！", "!", "!"),
                ch("“", "“", "quote"),
            ]),
            para(vec![
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("在", "zài", "at"), ch("门", "mén", "door"),
                ch("上", "shàng", "on"),
                ch("放", "fàng", "to put"),
                ch("了", "le", "completed"),
                ch("红", "hóng", "red"), ch("色", "sè", "color"),
                ch("的", "de", "possessive"), ch("东", "dōng", "thing"), ch("西", "xi", "thing"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("他", "tā", "they"), ch("们", "men", "plural"),
                ch("还", "hái", "also"),
                ch("放", "fàng", "to set off"),
                ch("了", "le", "completed"),
                ch("鞭", "biān", "firecracker"), ch("炮", "pào", "firecracker"), ch("。", ".", "."),
                ch("声", "shēng", "sound"), ch("音", "yīn", "sound"),
                ch("很", "hěn", "very"), ch("大", "dà", "big"), ch("！", "!", "!"),
            ]),
            para(vec![
                ch("年", "nián", "Nian"),
                ch("来", "lái", "to come"), ch("了", "le", "completed"), ch("。", ".", "."),
                ch("他", "tā", "it"), ch("看", "kàn", "to see"),
                ch("到", "dào", "to"),
                ch("红", "hóng", "red"), ch("色", "sè", "color"), ch("，", ",", ","),
                ch("听", "tīng", "to hear"), ch("到", "dào", "to"),
                ch("大", "dà", "big"), ch("声", "shēng", "sound"), ch("音", "yīn", "sound"), ch("，", ",", ","),
                ch("很", "hěn", "very"), ch("怕", "pà", "afraid"), ch("！", "!", "!"),
            ]),
            para(vec![
                ch("年", "nián", "Nian"),
                ch("跑", "pǎo", "to run"), ch("了", "le", "completed"), ch("！", "!", "!"),
                ch("他", "tā", "it"),
                ch("再", "zài", "again"),
                ch("也", "yě", "also"),
                ch("没", "méi", "not"),
                ch("有", "yǒu", "have"),
                ch("回", "huí", "to return"), ch("来", "lái", "come"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("从", "cóng", "from"), ch("那", "nà", "that"),
                ch("以", "yǐ", "after"), ch("后", "hòu", "after"), ch("，", ",", ","),
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"),
                ch("过", "guò", "to celebrate"), ch("年", "nián", "New Year"), ch("，", ",", ","),
                ch("放", "fàng", "to set off"),
                ch("鞭", "biān", "firecracker"), ch("炮", "pào", "firecracker"), ch("，", ",", ","),
                ch("穿", "chuān", "to wear"),
                ch("红", "hóng", "red"), ch("衣", "yī", "clothes"), ch("服", "fu", "clothes"), ch("。", ".", "."),
            ]),
        ],
    }
}

fn story_cowherd() -> Story {
    Story {
        id: 4,
        title: p("The Cowherd and the Weaver Girl"),
        title_cn: p("牛郎织女"),
        description: p("A love story written in the stars. The origin of the Qixi Festival."),
        extra_vocab: vec![
            extra("牛郎", "Niúláng", "Cowherd (name)"),
            extra("织女", "Zhīnǚ", "Weaver Girl (name)"),
            extra("牛", "niú", "cow, ox"),
            extra("天", "tiān", "sky, heaven"),
            extra("河", "hé", "river"),
            extra("星", "xīng", "star"),
            extra("爱", "ài", "to love"),
            extra("哭", "kū", "to cry"),
        ],
        paragraphs: vec![
            para(vec![
                ch("牛", "Niú", "Cowherd"), ch("郎", "láng", "Cowherd"),
                ch("是", "shì", "is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("很", "hěn", "very"),
                ch("好", "hǎo", "good"), ch("的", "de", "possessive"),
                ch("男", "nán", "male"), ch("人", "rén", "person"), ch("。", ".", "."),
                ch("他", "tā", "he"), ch("没", "méi", "not"), ch("有", "yǒu", "have"),
                ch("爸", "bà", "dad"), ch("爸", "ba", "dad"),
                ch("妈", "mā", "mom"), ch("妈", "ma", "mom"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("他", "tā", "he"), ch("有", "yǒu", "have"),
                ch("一", "yī", "one"), ch("头", "tóu", "measure word"),
                ch("老", "lǎo", "old"), ch("牛", "niú", "cow"), ch("。", ".", "."),
                ch("老", "lǎo", "old"), ch("牛", "niú", "cow"),
                ch("是", "shì", "is"), ch("他", "tā", "his"),
                ch("最", "zuì", "most"), ch("好", "hǎo", "good"),
                ch("的", "de", "possessive"),
                ch("朋", "péng", "friend"), ch("友", "yǒu", "friend"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("织", "Zhī", "Weaver"), ch("女", "nǚ", "Girl"),
                ch("住", "zhù", "to live"), ch("在", "zài", "at"),
                ch("天", "tiān", "sky"), ch("上", "shàng", "above"), ch("。", ".", "."),
                ch("她", "tā", "she"), ch("很", "hěn", "very"),
                ch("漂", "piào", "beautiful"), ch("亮", "liang", "beautiful"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("织", "Zhī", "Weaver"), ch("女", "nǚ", "Girl"),
                ch("来", "lái", "to come"), ch("到", "dào", "arrive"),
                ch("了", "le", "completed"),
                ch("人", "rén", "people"), ch("的", "de", "possessive"),
                ch("家", "jiā", "home"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("牛", "Niú", "Cowherd"), ch("郎", "láng", "Cowherd"),
                ch("和", "hé", "and"),
                ch("织", "Zhī", "Weaver"), ch("女", "nǚ", "Girl"),
                ch("很", "hěn", "very"),
                ch("喜", "xǐ", "like"), ch("欢", "huān", "like"),
                ch("对", "duì", "each"), ch("方", "fāng", "other"), ch("。", ".", "."),
                ch("他", "tā", "they"), ch("们", "men", "plural"),
                ch("有", "yǒu", "have"), ch("了", "le", "completed"),
                ch("两", "liǎng", "two"), ch("个", "gè", "measure word"),
                ch("孩", "hái", "child"), ch("子", "zi", "suffix"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("但", "dàn", "but"), ch("是", "shì", "but"), ch("，", ",", ","),
                ch("天", "tiān", "heaven"), ch("上", "shàng", "above"),
                ch("的", "de", "possessive"),
                ch("王", "wáng", "king"),
                ch("不", "bù", "not"), ch("高", "gāo", "happy"), ch("兴", "xìng", "happy"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("他", "tā", "he"),
                ch("让", "ràng", "to make"),
                ch("织", "Zhī", "Weaver"), ch("女", "nǚ", "Girl"),
                ch("回", "huí", "to return"),
                ch("天", "tiān", "sky"), ch("上", "shàng", "above"), ch("去", "qù", "to go"), ch("。", ".", "."),
                ch("在", "zài", "at"),
                ch("他", "tā", "them"), ch("们", "men", "plural"),
                ch("中", "zhōng", "middle"), ch("间", "jiān", "between"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("条", "tiáo", "measure word"),
                ch("大", "dà", "big"), ch("河", "hé", "river"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("牛", "Niú", "Cowherd"), ch("郎", "láng", "Cowherd"),
                ch("和", "hé", "and"),
                ch("孩", "hái", "child"), ch("子", "zi", "suffix"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"),
                ch("哭", "kū", "to cry"), ch("了", "le", "completed"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("七", "qī", "seven"), ch("月", "yuè", "month"),
                ch("七", "qī", "seven"), ch("日", "rì", "day"), ch("，", ",", ","),
                ch("很", "hěn", "very"), ch("多", "duō", "many"),
                ch("鸟", "niǎo", "bird"),
                ch("来", "lái", "to come"),
                ch("帮", "bāng", "to help"), ch("助", "zhù", "to help"),
                ch("他", "tā", "them"), ch("们", "men", "plural"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("这", "zhè", "this"), ch("一", "yī", "one"), ch("天", "tiān", "day"), ch("，", ",", ","),
                ch("牛", "Niú", "Cowherd"), ch("郎", "láng", "Cowherd"),
                ch("和", "hé", "and"),
                ch("织", "Zhī", "Weaver"), ch("女", "nǚ", "Girl"),
                ch("可", "kě", "can"), ch("以", "yǐ", "can"),
                ch("在", "zài", "at"),
                ch("一", "yì", "together"), ch("起", "qǐ", "together"), ch("。", ".", "."),
            ]),
        ],
    }
}

fn story_chang_e() -> Story {
    Story {
        id: 5,
        title: p("Chang'e Flies to the Moon"),
        title_cn: p("嫦娥奔月"),
        description: p("The origin of the Mid-Autumn Festival. A woman flies to the moon."),
        extra_vocab: vec![
            extra("嫦娥", "Cháng'é", "Chang'e (name)"),
            extra("后羿", "Hòu Yì", "Hou Yi (name)"),
            extra("月亮", "yuèliang", "moon"),
            extra("药", "yào", "medicine"),
            extra("飞", "fēi", "to fly"),
            extra("坏", "huài", "bad"),
            extra("偷", "tōu", "to steal"),
        ],
        paragraphs: vec![
            para(vec![
                ch("很", "hěn", "very"), ch("久", "jiǔ", "long time"),
                ch("以", "yǐ", "before"), ch("前", "qián", "before"), ch("，", ",", ","),
                ch("天", "tiān", "sky"), ch("上", "shàng", "above"),
                ch("有", "yǒu", "there are"),
                ch("十", "shí", "ten"), ch("个", "gè", "measure word"),
                ch("太", "tài", "too"), ch("阳", "yáng", "sun"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("太", "tài", "too"), ch("热", "rè", "hot"), ch("了", "le", "completed"), ch("！", "!", "!"),
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("不", "bù", "not"), ch("高", "gāo", "happy"), ch("兴", "xìng", "happy"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("后", "Hòu", "Hou Yi"), ch("羿", "Yì", "Hou Yi"),
                ch("是", "shì", "is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("很", "hěn", "very"), ch("勇", "yǒng", "brave"), ch("敢", "gǎn", "brave"),
                ch("的", "de", "possessive"), ch("人", "rén", "person"), ch("。", ".", "."),
                ch("他", "tā", "he"),
                ch("把", "bǎ", "disposal"),
                ch("九", "jiǔ", "nine"), ch("个", "gè", "measure word"),
                ch("太", "tài", "too"), ch("阳", "yáng", "sun"),
                ch("打", "dǎ", "to hit"), ch("下", "xià", "down"),
                ch("来", "lái", "come"), ch("了", "le", "completed"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("都", "dōu", "all"), ch("很", "hěn", "very"),
                ch("高", "gāo", "happy"), ch("兴", "xìng", "happy"), ch("。", ".", "."),
                ch("他", "tā", "they"), ch("们", "men", "plural"),
                ch("给", "gěi", "to give"),
                ch("了", "le", "completed"),
                ch("后", "Hòu", "Hou Yi"), ch("羿", "Yì", "Hou Yi"),
                ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("药", "yào", "medicine"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("吃", "chī", "to eat"), ch("了", "le", "completed"),
                ch("这", "zhè", "this"), ch("个", "gè", "measure word"),
                ch("药", "yào", "medicine"), ch("，", ",", ","),
                ch("就", "jiù", "then"),
                ch("可", "kě", "can"), ch("以", "yǐ", "can"),
                ch("飞", "fēi", "to fly"),
                ch("到", "dào", "to"),
                ch("天", "tiān", "sky"), ch("上", "shàng", "above"),
                ch("去", "qù", "to go"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("后", "Hòu", "Hou Yi"), ch("羿", "Yì", "Hou Yi"),
                ch("的", "de", "possessive"),
                ch("妻", "qī", "wife"), ch("子", "zi", "suffix"),
                ch("叫", "jiào", "called"),
                ch("嫦", "Cháng", "Chang'e"), ch("娥", "é", "Chang'e"), ch("。", ".", "."),
                ch("他", "tā", "he"),
                ch("不", "bù", "not"), ch("想", "xiǎng", "to want"),
                ch("吃", "chī", "to eat"),
                ch("药", "yào", "medicine"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("但", "dàn", "but"), ch("是", "shì", "but"), ch("，", ",", ","),
                ch("有", "yǒu", "there is"), ch("一", "yī", "one"), ch("个", "gè", "measure word"),
                ch("坏", "huài", "bad"), ch("人", "rén", "person"),
                ch("想", "xiǎng", "to want"),
                ch("偷", "tōu", "to steal"),
                ch("这", "zhè", "this"), ch("个", "gè", "measure word"),
                ch("药", "yào", "medicine"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("嫦", "Cháng", "Chang'e"), ch("娥", "é", "Chang'e"),
                ch("不", "bù", "not"),
                ch("想", "xiǎng", "to want"),
                ch("让", "ràng", "to let"),
                ch("坏", "huài", "bad"), ch("人", "rén", "person"),
                ch("吃", "chī", "to eat"),
                ch("药", "yào", "medicine"), ch("。", ".", "."),
                ch("她", "tā", "she"),
                ch("自", "zì", "self"), ch("己", "jǐ", "self"),
                ch("吃", "chī", "to eat"), ch("了", "le", "completed"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("她", "tā", "she"),
                ch("飞", "fēi", "to fly"),
                ch("到", "dào", "to"), ch("了", "le", "completed"),
                ch("月", "yuè", "moon"), ch("亮", "liang", "moon"),
                ch("上", "shàng", "above"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("后", "Hòu", "Hou Yi"), ch("羿", "Yì", "Hou Yi"),
                ch("很", "hěn", "very"),
                ch("想", "xiǎng", "to miss"),
                ch("她", "tā", "her"), ch("。", ".", "."),
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("八", "bā", "eight"), ch("月", "yuè", "month"),
                ch("十", "shí", "ten"), ch("五", "wǔ", "five"), ch("日", "rì", "day"), ch("，", ",", ","),
                ch("他", "tā", "he"),
                ch("看", "kàn", "to look at"),
                ch("月", "yuè", "moon"), ch("亮", "liang", "moon"), ch("，", ",", ","),
                ch("想", "xiǎng", "to miss"),
                ch("嫦", "Cháng", "Chang'e"), ch("娥", "é", "Chang'e"), ch("。", ".", "."),
            ]),
            para(vec![
                ch("现", "xiàn", "now"), ch("在", "zài", "now"), ch("，", ",", ","),
                ch("人", "rén", "people"), ch("们", "men", "plural"),
                ch("每", "měi", "every"), ch("年", "nián", "year"),
                ch("八", "bā", "eight"), ch("月", "yuè", "month"),
                ch("十", "shí", "ten"), ch("五", "wǔ", "five"), ch("日", "rì", "day"),
                ch("都", "dōu", "all"),
                ch("一", "yì", "together"), ch("起", "qǐ", "together"),
                ch("看", "kàn", "to look at"),
                ch("月", "yuè", "moon"), ch("亮", "liang", "moon"), ch("。", ".", "."),
            ]),
        ],
    }
}
