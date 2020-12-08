#![feature(non_ascii_idents)]
use csv::{Reader, StringRecord};
use serde::Deserialize;
use std::{error::Error, print};

#[derive(Deserialize)]
struct CSVDATA {
    日期: String,
    早盘多空: i32,
    早盘寒暖: i32,
    开: f64,
    收: f64,
    春: i32,
    夏: i32,
    伏: i32,
    秋: i32,
    冬: i32,
    腊: i32,
    气候: i32,
    厄尔尼诺: i32,
    小厄: i32,
    酷暑: i32,
    切变线: i32,
    大暑: i32,
    高压脊: i32,
    梅雨末: i32,
    晴空万里: i32,
    冰雪消融: i32,
    极曙: i32,
    寒极暖生: i32,
    艳阳高照: i32,
    高气压: i32,
    海市蜃楼: i32,
    乍寒还暖: i32,
    深寒: i32,
    立夏: i32,
    暮光: i32,
    曙光: i32,
    收盘寒暖: i32,
    隔日盈亏: f64,
}

impl std::fmt::Display for CSVDATA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.日期, self.春, self.隔日盈亏)
    }
}

struct HUICE {
    data: Vec<CSVDATA>,
}

impl HUICE {
    //读取文件创建结构
    fn from_file() -> HUICE {
        let mut dat = Vec::new();
        let rdr = Reader::from_path("操作.csv"); //打开文件
        match rdr {
            Ok(mut r) => {
                let rd = r.records();
                for i in rd {
                    //逐条读取
                    if let Ok(resu) = i {
                        let d: CSVDATA = resu.deserialize(None).unwrap();
                        dat.push(d);
                    }
                }
            }
            Err(_) => println!("读取csv文件错误！"),
        }
        HUICE { data: dat }
    }

    //fn parse(筛后数据表:Vec<Row>,)->(总次数：u32,胜次，亏次，胜率：f64,盈亏比：f64，单信号累积：f64){}
    fn parse(&self, tem: Vec<&CSVDATA>) -> (u32, u32, u32, f64, f64, f64) {
        let no = tem.len();

        let mut sum = 0.0;
        let mut win = 0;
        let mut loss = 0;
        let mut w_sum = 0.0;
        let mut l_sum = 0.0;
        let mut ykb: f64;
        for i in tem {
            sum += i.隔日盈亏;
            if i.隔日盈亏 > 0.0 {
                win += 1;
                w_sum += i.隔日盈亏;
            } else if i.隔日盈亏 < 0.0 {
                loss += 1;
                l_sum -= i.隔日盈亏;
            }
        }
        let sl: f64 = win as f64 / (win as f64 + loss as f64) * 100.0;
        if l_sum == 0.0 {
            ykb = 100.0;
        } else {
            ykb = w_sum / l_sum;
        }

        (no as u32, win, loss, sl, ykb, sum)
    }

    fn p_out(&self, t: Vec<&CSVDATA>, n: &str) {
        let out = self.parse(t);
        println!(
            "{}            \t{}\t{}\t{}\t{:.2}\t{:.2}\t{:.2}",
            n, out.0, out.1, out.2, out.3, out.4, out.5
        );
    }

    fn chun(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.春 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "春");
    }

    fn xia(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.夏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "夏");
    }
    fn qiu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.秋 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "秋");
    }
    fn dong(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.冬 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "冬");
    }
    fn fu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.伏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "伏");
    }
    fn la(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.腊 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "腊");
    }
    fn eenn(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔");
    }
    fn eenn_chun(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.春 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_春");
    }
    fn eenn_xia(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.夏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_夏");
    }

    fn eenn_qiu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.秋 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_秋");
    }
    fn eenn_dong(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.冬 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_冬");
    }
    fn eenn_fu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.伏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_伏");
    }
    fn eenn_la(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.厄尔尼诺 == 1 && i.腊 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "厄尔_腊");
    }
    fn xiaoe(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄");
    }
    fn xiaoe_chun(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.春 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_春");
    }
    fn xiaoe_xia(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.夏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_夏");
    }
    fn xiaoe_qiu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.秋 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_秋");
    }
    fn xiaoe_dong(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.冬 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_冬");
    }
    fn xiaoe_fu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.伏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_伏");
    }
    fn xiaoe_la(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.小厄 == 1 && i.腊 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "小厄_腊");
    }
    fn kushu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.酷暑 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "酷暑");
    }
    fn qiebian(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.切变线 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "切变线");
    }
    fn dashu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.大暑 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "大暑");
    }
    fn gaoyaji(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.高压脊 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "高压脊");
    }
    fn meiyumo(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.梅雨末 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "梅雨末");
    }
    fn qingkongwanli(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.晴空万里 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "晴空万里");
    }
    fn bingxuexiaorong(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.冰雪消融 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "冰雪消融");
    }
    fn jidishuguang(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光");
    }
    fn jidishuguang_chun(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.春 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光春");
    }
    fn jidishuguang_xia(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.夏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光夏");
    }
    fn jidishuguang_qiu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.秋 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光秋");
    }
    fn jidishuguang_dong(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.冬 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光冬");
    }
    fn jidishuguang_fu(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.伏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光伏");
    }
    fn jidishuguang_la(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.极曙 == 1 && i.腊 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地曙光腊");
    }
    fn hanjinuansheng(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.寒极暖生 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "寒极暖生");
    }
    fn yanyanggaozhao(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.艳阳高照 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "艳阳高照");
    }
    fn gaoqiya(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.高气压 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "高气压");
    }
    fn haishishenlou(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.海市蜃楼 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "海市蜃楼");
    }
    fn zhahanhainuan(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.乍寒还暖 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "乍寒还暖");
    }
    fn jidishenhan(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.深寒 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "极地深寒");
    }
    fn lixia(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.立夏 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "立夏");
    }
    //做空信号
    fn muguang(&self) {}
    fn shuguang(&self) {
        let mut tmp = Vec::new();
        for i in &self.data {
            if i.曙光 == 1 {
                tmp.push(i);
            }
        }
        self.p_out(tmp, "曙光");
    }
}

fn main() {
    let r = HUICE::from_file();
    println!("信号             \t总次数\t胜次\t亏次\t胜率\t盈亏比\t单信号累积\t");

    r.eenn();
    
    r.xiaoe();
    
    r.kushu();
    r.qiebian();
    r.dashu();
    r.gaoyaji();
    r.meiyumo();
    r.qingkongwanli();
    r.bingxuexiaorong();
    r.jidishuguang();
    
    r.hanjinuansheng();
    r.yanyanggaozhao();
    r.gaoqiya();
    r.haishishenlou();
    r.zhahanhainuan();
    r.jidishenhan();
    r.lixia();
    r.shuguang();
    r.eenn_chun();
    r.eenn_xia();
    r.eenn_qiu();
    r.eenn_dong();
    r.eenn_fu();
    r.eenn_la();
    r.xiaoe_chun();
    r.xiaoe_xia();
    r.xiaoe_qiu();
    r.xiaoe_dong();
    r.xiaoe_fu();
    r.xiaoe_la();
    r.jidishuguang_chun();
    r.jidishuguang_xia();
    r.jidishuguang_qiu();
    r.jidishuguang_dong();
    r.jidishuguang_fu();
    r.jidishuguang_la();
}
