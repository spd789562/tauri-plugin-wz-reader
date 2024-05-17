use wz_reader::{WzNodeArc, WzNodeCast};

pub fn get_image_node(root: &WzNodeArc, path: &str) -> Option<(WzNodeArc, String)> {
    let mut pathes = path.split('/');
    let mut node = root.clone();
    while let Some(path) = pathes.next() {
        let target = node.read().unwrap().at(path);
        if let Some(target) = target {
            node = target;
            if node.read().unwrap().try_as_image().is_some() {
                let rest = pathes.collect::<Vec<&str>>().join("/");
                return Some((node, rest));
            }
        } else {
            return None;
        }
    }
    None
}

pub fn get_node_without_parse(root: &WzNodeArc, path: &str) -> Option<WzNodeArc> {
    let has_img = path.contains(".img");

    if has_img {
        let mut pathes = path.split(".img");
        let img_path = pathes.next()?;
        let rest_path = pathes.next()?;

        let image_node = root.read().unwrap().at_path(img_path)?;
        let image_read = image_node.read().unwrap();
        let image = image_read.try_as_image()?;

        image.at_path(rest_path).ok()
    } else {
        let (image_node, rest_path) = get_image_node(root, path)?;
        let image_read = image_node.read().unwrap();
        let image = image_read.try_as_image()?;

        image.at_path(&rest_path).ok()
    }
}
