pub mod tga;

use alloc::vec::Vec;

pub trait Image {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn get_texture(&self) -> Vec<u32>;
}

// let scale_texture = |tex: &[u32], width: usize, scale: usize| -> Vec<u32> {
//     tex
//         .to_vec()
//         .chunks(width)
//         .flat_map(|row| {
//             let row_scaled = row
//                 .into_iter()
//                 // Repeat each pixel {scale} times
//                 .flat_map(|pixel| (0..scale).map(|_|*pixel))
//                 .collect::<Vec<_>>();

//             (0..scale)
//                 // Repeat each row {scale} times
//                 .flat_map(move |_| row_scaled.clone())
//         })
//         .collect::<Vec<u32>>()
// };