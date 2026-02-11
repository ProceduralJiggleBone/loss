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
    let (lwidth, lheight) = lossrgb.dimensions();
    let subjectrgb = subject.to_rgb8();
    let (swidth, sheight) = subjectrgb.dimensions();
    println!("subject image has width: {:?} and height {:?}", swidth, sheight);
    let mut query_point = [234 as f32, 60 as f32, 11 as f32];
    let mut index = 0;

    println!("indexing loss...");
    for (x, y ,pixel) in lossrgb.enumerate_pixels() {
        println!("{:?} out of {:?}", y, lheight);
        let [r, g, b] = pixel.0;
        losspixels.push([r.into(), g.into(), b.into()]);
    }
    println!("loss indexed. Forming kdtree..");

    let loss_kdtree: ImmutableKdTree<f32, 3> = losspixels.as_slice().into();
    println!("kdtree formed.");
    assert_eq!(loss_kdtree.size(), losspixels.len());
    
    
    //index = loss_kdtree.nearest_one::<SquaredEuclidean>(&query_point).item;
    //println!("index: {:?}", index);
    //println!("item: {:?}", losspixels[index as usize])


    for (sx, sy ,spixel) in subjectrgb.enumerate_pixels() {
        let [sr, sg, sb] = spixel.0;
        query_point = [sr.into(), sg.into(), sb.into()];
        index = loss_kdtree.nearest_one::<SquaredEuclidean>(&query_point).item;
        println!("item: {:?} ({:?} out of {:?})", losspixels[index as usize], sy, sheight);
        //println!("x{} y{}", sx, sy);
    }
}
