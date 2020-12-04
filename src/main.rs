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
                let mut rd = r.records();
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

    //fn parse(总数据表:Vec<Row>,单标志序号:u32)->(次数：u32,胜率：f64,盈亏比：f64，单信号累积：f64){}
    fn parse<T>(&self, tem: Vec<CSVDATA>) -> (u32, f64, f64, f64) {
        let no = tem.len();

        let mut sum = 0.0;
        let mut win = 0.0;
        let mut loss = 0.0;
        let mut w_sum = 0.0;
        let mut l_sum = 0.0;
        for i in tem {
            l_sum+=i.隔日盈亏;
            if i.隔日盈亏 > 0.0 {
                win+=1.0;
                w_sum+=i.隔日盈亏;
            } else if i.隔日盈亏 < 0.0 {
                loss+=1.0;
                l_sum-=i.隔日盈亏;
            }
        }
        let sl:f64=win  /(win+loss)*100.0;
        let ykb=w_sum/l_sum;
        (no as u32, sl, ykb, sum)
    }
}

fn main() {
    let r = HUICE::from_file();

    //let t= r.parse(1);
}
