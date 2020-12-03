use csv::{Reader,StringRecord};
use serde::Deserialize;
use std::error::Error;
fn main() {
    RdrFromFile().unwrap();
}

fn RdrFromFile() -> Result<(), Box<dyn Error>> {
    let mut rdr = Reader::from_path("操作.csv")?;
    for result in rdr.records() {
        let mut record = result?;
        
        println!("{:?}", record);
    }
    Ok(())
}




#[derive(Deserialize)]
struct Row<'a> {
    日期:String,
    早盘多空:u32,早盘寒暖,开,收,春,夏,伏,秋,冬,腊,气候,厄尔尼诺,小厄,酷暑,切变线,大暑,高压脊,梅雨末,晴空万里,冰雪消融,极曙,寒极暖生,艳阳高照,高气压,海市蜃楼,乍寒还暖,深寒,立夏,暮光,曙光,收盘寒暖,收信,隔日盈亏
}

fn example() -> Result<(), Box<dyn Error>> {
    let record = StringRecord::from(vec![
        "Boston", "United States", "4628910",
    ]);

    let row: Row = record.deserialize(None)?;
    assert_eq!(row.city, "Boston");
    assert_eq!(row.country, "United States");
    assert_eq!(row.population, 4628910);
    Ok(())}