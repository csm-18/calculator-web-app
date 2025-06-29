use wasm_bindgen::prelude::*;
mod calc;
#[wasm_bindgen]
pub fn calc(exp:String)-> String{
    calc::calc(&exp)

}