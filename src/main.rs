//https://cad-comic.com/wp-content/uploads/2017/03/cad-20080602-358b1.x79890.jpg
use image;
use kiddo::ImmutableKdTree;
use kiddo::SquaredEuclidean;
use kiddo::NearestNeighbour;
use image::ImageReader;

fn main() {
    let subject = image::open("/home/raspberrypi/1280px-Orange_tabby_cat_sitting_on_fallen_leaves-Hisashi-01A.jpg").expect("Subject not found!");
    let loss = image::open("cad-20080602-358b1.x79890.jpg").expect("Loss not found!");
    let mut losspixels: Vec<[f32; 3]> = vec![];
    let lossrgb = loss.to_rgb8();
    let subjectrgb = subject.to_rgb8();
    let query_point = [1 as f32, 1 as f32, 1 as f32];
    for (x, y ,pixel) in lossrgb.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        losspixels.push([r.into(), g.into(), b.into()]);
    }

    //println!("{:?}", losspixels);


    let loss_kdtree: ImmutableKdTree<f32, 3> = losspixels.as_slice().into();

    assert_eq!(loss_kdtree.size(), losspixels.len());
    
    println!("{:?}", loss_kdtree.nearest_one::<SquaredEuclidean>(&query_point).item);
    //println!("{:?}", losspixels[loss_kdtree.nearest_one::<SquaredEuclidean>(&query_point).item]);




    /*for (sx, sy ,spixel) in subjectrgb.enumerate_pixels() {
        let [sr, sg, sb] = spixel.0;


        for (lx, ly ,lpixel) in lossrgb.enumerate_pixels() {
            let [lr, lg, lb] = lpixel.0;
            
            let mut current_best = [0, 0, 2147483646];
            //                     x  y  d
            
            let tempr: i32 = (sr as i32 - lr as i32);
            let tempg: i32 = (sg as i32 - lg as i32);
            let tempb: i32 = (sb as i32 - lb as i32);

            let current = tempr.pow(2) + tempg.pow(2) + tempb.pow(2);

            if current < current_best[2] {
                current_best[2] = current;
            }
            //println!("{}", current);
        }
        println!("x{} y{}", sx, sy);
    }*/


    //println!("{:?}", losspixels);
}
