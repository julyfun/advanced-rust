#[derive(Debug, serde::Deserialize, Clone)] enum 东西 { 萝卜, 纸巾, 米老鼠, 空气, 真棒, 嘎吱嘎吱吃冻干 }
use 东西::*; use rand::{rng, rngs::ThreadRng, seq::IndexedRandom};
struct 猫咪 {} impl 猫咪 {
    pub fn 听(&self, 你说啥: 东西, 大脑: &mut ThreadRng) {
        match 你说啥 {
            真棒 => self.碰(嘎吱嘎吱吃冻干),
            _ => self.碰((*[萝卜, 纸巾, 米老鼠, 空气, 空气].choose(大脑).unwrap()).clone()),
        }
    }
    fn 碰(&self, 啥: 东西) {
        use std::{thread::sleep, time::Duration};
        sleep(Duration::from_secs(1u64));
        println!("🐱: {啥:?}")
    }
}
fn main() { loop {
    let 你说啥: 东西 = serde_json::from_str(
        ("{\"".to_string() + py_like::input().trim() + "\":null}").as_str()).unwrap();
    let mut 大脑 = rng();
    猫咪{}.听(你说啥, &mut 大脑);
}}

