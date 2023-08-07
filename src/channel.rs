use crate::ffield_unit::FFieldUnit;
pub fn serialize(units: &[FFieldUnit]) -> String {
    units
        .iter()
        .map(|unit| unit.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
