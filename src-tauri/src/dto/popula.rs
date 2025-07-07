use serde::{ Deserialize, Serialize };
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Popula {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    //年份
    pub year: Option<i32>,
    //总人口
    pub population: Option<f64>,
    //自然增长人口
    pub growth_popula: Option<f64>,
    //年平均数
    pub annual_average: Option<f64>,
    //大于15岁
    pub over15: Option<f64>,
    //新生儿
    pub newborn: Option<f64>,
    //死亡数量
    pub death: Option<f64>,
    //总抚养比
    pub total_dependency_ratio: Option<f64>,
    //幼儿抚养比
    pub child_dependency_ratio: Option<f64>,
    //老年抚养比
    pub old_dependency_ratio: Option<f64>,
    //出生率
    pub birth_rate: Option<f64>,
    //死亡率
    pub mortality: Option<f64>,
    //自然增长率
    pub growth_rate: Option<f64>,
    //初婚人数
    pub first_marriage: Option<f64>,
    //未婚人数
    pub unmarried: Option<f64>,
    //再婚人数
    pub remarry: Option<f64>,
    //离婚人数
    pub divorce: Option<f64>,
    //0-14岁
    pub y0_y14: Option<f64>,
    //15-64岁
    pub y15_y64: Option<f64>,
    //大于65岁
    pub over65: Option<f64>,
}
