#![feature(non_ascii_idents)]
use csv::{Reader, StringRecord};
use serde::Deserialize;
use std::{error::Error, print};

#[derive(Deserialize)]
struct Row {
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
    收信: i32,
    隔日盈亏: f64,
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{},{})", self.日期, self.春,self.隔日盈亏)
    }
}


fn main()  {
    let r=rdr_from_file();
    match r{
        Ok(r)=>{parse(r,1);},
        Err(_)=>{},
    };
}

fn rdr_from_file() -> Result<Vec<Row>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("操作.csv")?;
    let mut v:Vec<Row>=Vec::new();
    for result in rdr.records() {
        let mut record = result?;
    
        let row: Row = record.deserialize(None)?;
      v.push(row);
    }
    Ok(v)
}

///fn parse(总数据表:Vec<Row>,单标志序号:u32)->(次数：u32,胜率：f64,盈亏比：f64，单信号累计：f64){}
fn parse(dat:Vec<Row>,flag:u32)->(u32,f64,f64,f64){
    for i in dat{
        if i.春==1{
            println!("{}",i);
        }
    }
(0,0.0,0.0,0.0)
}
