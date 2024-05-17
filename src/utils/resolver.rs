use futures::future::{BoxFuture, FutureExt};
use std::path::Path;
use tokio::fs::{self, DirEntry};

use wz_reader::{version::WzMapleVersion, WzNode, WzNodeArc, WzObjectType};

use super::{block_parse, block_parse_with_parent};
use crate::{Error, Result};

pub async fn get_root_wz_file_path(dir: &DirEntry) -> Option<String> {
    let dir_name = dir.file_name();
    let mut inner_wz_name = dir_name.to_str().unwrap().to_string();
    inner_wz_name.push_str(".wz");
    let inner_wz_path = dir.path().join(inner_wz_name);

    if fs::try_exists(&inner_wz_path).await.unwrap() {
        return Some(inner_wz_path.to_str().unwrap().to_string());
    }

    None
}

pub fn resolve_root_wz_file_dir<'a>(
    dir: &'a str,
    version: Option<WzMapleVersion>,
    patch_version: Option<i32>,
    parent: Option<&'a WzNodeArc>,
) -> BoxFuture<'a, Result<WzNodeArc>> {
    async move {
        let root_node: WzNodeArc = WzNode::from_wz_file(dir, version, patch_version, parent)
            .unwrap()
            .into();
        let wz_dir = Path::new(dir).parent().unwrap();

        block_parse(&root_node).await?;

        {
            let mut entries = fs::read_dir(wz_dir).await?;

            while let Some(entry) = entries.next_entry().await? {
                let file_type = entry.file_type().await?;
                let name = entry.file_name();

                let target_node = {
                    let root_node = root_node.read().unwrap();
                    root_node.at(name.to_str().unwrap())
                };

                if file_type.is_dir() && target_node.is_some() {
                    if let Some(file_path) = get_root_wz_file_path(&entry).await {
                        let dir_node = resolve_root_wz_file_dir(
                            &file_path,
                            version,
                            patch_version,
                            Some(&root_node),
                        )
                        .await?;

                        /* replace the original one */
                        let mut root_node_write = root_node.write().unwrap();
                        root_node_write
                            .children
                            .insert(name.to_str().unwrap().into(), dir_node);
                    }
                } else if file_type.is_file() {
                    //  check is XXX_nnn.wz
                    let file_path = entry.path();
                    let file_name = file_path.file_stem().unwrap().to_str().unwrap();

                    let splited = file_name.split('_').collect::<Vec<&str>>();

                    if splited.len() < 2 {
                        continue;
                    }

                    if splited.last().unwrap().parse::<u16>().is_err() {
                        continue;
                    }

                    let node = WzNode::from_wz_file(
                        file_path.to_str().unwrap(),
                        version,
                        patch_version,
                        None,
                    )
                    .unwrap()
                    .into_lock();

                    if block_parse_with_parent(&node, &root_node).await.is_ok() {
                        let mut node_write = node.write().unwrap();
                        let mut root_node_write = root_node.write().unwrap();
                        root_node_write.children.reserve(node_write.children.len());
                        for (name, child) in node_write.children.drain() {
                            root_node_write.children.insert(name, child);
                        }
                    }
                }
            }
        }

        Ok(root_node)
    }
    .boxed()
}

pub async fn resolve_base(path: &str, version: Option<WzMapleVersion>) -> Result<WzNodeArc> {
    if !path.ends_with("Base.wz") {
        return Err(Error::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "not a Base.wz",
        )));
    }

    let base_node = resolve_root_wz_file_dir(path, version, None, None).await?;

    let patch_version = {
        if let WzObjectType::File(file) = &base_node.read().unwrap().object_type {
            file.wz_file_meta.patch_version
        } else {
            -1
        }
    };

    {
        let wz_root_path = Path::new(path).parent().unwrap().parent().unwrap();

        let mut entries = fs::read_dir(wz_root_path).await?;

        while let Some(item) = entries.next_entry().await? {
            let file_name = item.file_name();

            let has_dir = base_node
                .read()
                .unwrap()
                .at(file_name.to_str().unwrap())
                .is_some();

            if has_dir {
                let wz_path = get_root_wz_file_path(&item).await;

                if let Some(file_path) = wz_path {
                    let dir_node = resolve_root_wz_file_dir(
                        &file_path,
                        version,
                        Some(patch_version),
                        Some(&base_node),
                    )
                    .await?;

                    /* replace the original one */
                    base_node
                        .write()
                        .unwrap()
                        .children
                        .insert(file_name.to_str().unwrap().into(), dir_node);
                }
            }
        }
    }

    Ok(base_node)
}
