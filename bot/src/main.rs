use akaze::Akaze;
use cv::BitArray;

struct TargetPath {
    /// Bookmark image path
    bm_path: String,
}

impl TargetPath {
    fn new(bm_path: &str) -> Self {
        Self {
            bm_path: bm_path.to_owned(),
        }
    }
}

fn main() {
    let references = TargetPath::new("./references/bm_ref.png");

    let bm_image: image::DynamicImage =
        image::open(&references.bm_path).expect("invalid bookmark reference path");
    let print = image::open("./references/whole_item_ref.png").expect("invalid print path");

    let akaze: Akaze = Akaze::default();

    let (kp_bm, desc_bm) = akaze.extract(&bm_image);
    let (kp_print, desc_print) = akaze.extract(&print);

    // let matches = symm
}

fn matching(a_descriptors: &[BitArray<64>], b_descriptors: &[BitArray<64>]) -> Vec<Option<usize>> {
    let knn_b = LinearKnn {
        metric: Hamming,
        iter: b_descriptors.iter(),
    };
    (0..a_descriptors.len())
        .map(|a_feature| {
            let knn = knn_b.knn(&a_descriptors[a_feature], 2);
            if knn[0].distance + 24 < knn[1].distance {
                Some(knn[0].index)
            } else {
                None
            }
        })
        .collect()
}

fn match_images(img_a: &[BitArray<64>], img_b: &[BitArray<64>]) -> Vec<[usize; 2]> {
    let forward_matches = matching(img_a, img_b);
    let reverse_matches = matching(img_b, img_a);

    forward_matches.into_iter();

    vec![]
}
