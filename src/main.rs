use noise::core::simplex::simplex_3d;
use noise::permutationtable::PermutationTable;
use nalgebra::Vector3;

fn main() {
    let seed = 11;
    let octaves = 5;
    let persistence = 0.5;
    let lacunarity = 2.0;
    let scale = 0.01;

    let hasher = PermutationTable::new(seed);

    let width = 100;
    let height = 100;

    for y in 0..height {
        for x in 0..width {
            let mut noise_value = 0.0;
            let mut frequency = 1.0;
            let mut amplitude = 1.0;

            let z = 0.0;
            let mut point = Vector3::new(x as f64 * scale, y as f64 * scale, z);

            for _ in 0..octaves {
                let (noise_val, _) = simplex_3d(point, &hasher); // Utilisation de la première valeur
                noise_value += noise_val * amplitude;
                amplitude *= persistence;
                frequency *= lacunarity;

                point = Vector3::new(
                    point.x * lacunarity,
                    point.y * lacunarity,
                    point.z * lacunarity,
                );
            }

            let height_value = ((noise_value + 1.0) * 0.5 * 255.0) as u32;
            print!("{:3} ", height_value);
        }
        println!();
    }
}
