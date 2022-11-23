pub use crate::areas_volumes::{GeometricalShapes, GeometricalVolumes};
pub mod areas_volumes;

pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let unit_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b)  as f64,
    };

    let all_units_area = times as f64 * unit_area;

    return (x * y) as f64 > all_units_area; 
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let unit_volume = match objects {
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
    };

    let all_units_area = times as f64 * unit_volume;

    return (x * y * z) as f64 > all_units_area;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(area_fit(2, 4, GeometricalShapes::Rectangle, 100, 2, 1), false);
        assert_eq!(area_fit(5, 5, GeometricalShapes::Triangle, 3, 5, 3), true);
        assert_eq!(volume_fit(5, 5, 5, GeometricalVolumes::Sphere, 3, 2, 0, 0), true);
        assert_eq!(volume_fit(5, 7, 5, GeometricalVolumes::Parallelepiped, 1, 6, 7, 4), true);
    }
}
