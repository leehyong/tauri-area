use rbatis::crud::{CRUDTable};

#[crud_table(table_name: "province")]
#[derive(Clone, Debug)]
pub struct Province {
    code: String,
    name: String,
}

#[crud_table(table_name: "city")]
#[derive(Clone, Debug)]
pub struct City {
    code: String,
    name: String,
    #[allow(non_camel_case_types)]
    provinceCode: String,
}

#[crud_table(table_name: "area")]
#[derive(Clone, Debug)]
pub struct Area {
    code: String,
    name: String,
    provinceCode: String,
    cityCode: String,
}

#[crud_table(table_name: "street" )]
#[derive(Clone, Debug)]
pub struct Street {
    code: String,
    name: String,
    provinceCode: String,
    cityCode: String,
    areaCode: String,
}

#[crud_table(table_name: "village")]
#[derive(Clone, Debug)]
pub struct Village {
    code: String,
    name: String,
    provinceCode: String,
    cityCode: String,
    areaCode: String,
    streetCode: String,
}

