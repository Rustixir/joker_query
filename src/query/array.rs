use super::value::Value;



pub struct Array<'a> {
    pub array: Vec<Value<'a>>
}


impl<'a> From<Vec<i32>> for Array<'a> {
    fn from(value: Vec<i32>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<i64>> for Array<'a> {
    fn from(value: Vec<i64>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<f32>> for Array<'a> {
    fn from(value: Vec<f32>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<f64>> for Array<'a> {
    fn from(value: Vec<f64>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<bool>> for Array<'a> {
    fn from(value: Vec<bool>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<&'a str>> for Array<'a> {
    fn from(value: Vec<&'a str>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<String>> for Array<'a> {
    fn from(value: Vec<String>) -> Self {
        let mut array = Vec::<Value<'a>>::with_capacity(value.len());
        value.into_iter().for_each(|elem| array.push(elem.into()));
        return Array { array }
    }
}
impl<'a> From<Vec<Value<'a>>> for Array<'a> {
    fn from(value: Vec<Value<'a>>) -> Self {
        return Array { array: value }
    }
}