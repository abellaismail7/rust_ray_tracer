//use super::vec3::Vec3;

//#[derive(Debug)]
//pub enum PatternTypes {
//    Stripped(Vec3, Vec3),
//    Unified(Vec3),
//}
//
//#[derive(Debug)]
//pub struct Pattern {
//    kind: PatternTypes,
//}
//
//impl Pattern {
//
//    pub fn get_color(&self, point: &Vec3) -> &Vec3 {
//        match &self.kind {
//            PatternTypes::Stripped(odd_color, even_color ) => {
//                if point.x.floor() % 2.0 == 0.0{
//                    return even_color;
//                }
//                odd_color
//            }
//            PatternTypes::Unified(color) => {
//                color
//            },
//        }
//    }
//}
//
