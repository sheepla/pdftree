use lopdf::{Document, Object, ObjectId};
use serde::Serialize;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, clap::Parser)]
struct Args {
    path: PathBuf,

    #[arg(short, long, help = "Output as JSON")]
    json: bool,
}

#[derive(Debug, Serialize)]
struct OutlineItem {
    title: Option<String>,
    children: Vec<OutlineItem>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to load the document: {0}")]
    DocumentLoad(lopdf::Error),

    #[error("Missing or invalid Root in trailer: {0}")]
    MissingRoot(lopdf::Error),

    #[error("Catalog object is not a dictionary: {0}")]
    InvalidCatalog(lopdf::Error),

    #[error("Outlines object is not found or invalid: {0}")]
    InvalidOutlines(lopdf::Error),

    #[error("Outline root dictionary is invalid: {0}")]
    InvalidOutlineRoot(lopdf::Error),

    #[error("Failed to extract title: {0}")]
    ExtractTitle(lopdf::Error),

    #[error("Failed to find first child node: {0}")]
    FindFirstChild(lopdf::Error),

    #[error("Failed to find next sibling node: {0}")]
    FindNextSibling(lopdf::Error),

    #[error("Failed to decode bytes as UTF-16BE: {0}")]
    DecodeUtf16(std::string::FromUtf16Error),

    #[error("JSON serialization failed: {0}")]
    Json(#[from] serde_json::Error),
}

fn main() -> Result<(), Error> {
    let args = Args::parse();

    let doc = Document::load(args.path).map_err(Error::DocumentLoad)?;

    let catalog_id: lopdf::ObjectId = doc
        .trailer
        .get(b"Root")
        .map_err(Error::MissingRoot)?
        .as_reference()
        .map_err(|err| Error::MissingRoot(err))?;

    let catalog: &lopdf::Dictionary = doc
        .get_object(catalog_id)
        .map_err(Error::InvalidCatalog)?
        .as_dict()
        .map_err(Error::InvalidCatalog)?;

    let outlines_id: lopdf::ObjectId = catalog
        .get(b"Outlines")
        .map_err(Error::InvalidOutlines)?
        .as_reference() // Get ObjectID
        .map_err(Error::InvalidOutlines)?;

    let outline_root: &lopdf::Object = doc
        .get_object(outlines_id)
        .map_err(Error::InvalidOutlines)?;

    let outline_dict: &lopdf::Dictionary =
        outline_root.as_dict().map_err(Error::InvalidOutlineRoot)?;

    let first_outline_item_id: lopdf::ObjectId = outline_dict
        .get(b"First")
        .map_err(Error::InvalidOutlines)?
        .as_reference() // Get ObjectID
        .map_err(Error::InvalidOutlines)?;

    #[cfg(debug_assertions)]
    dbg!(&first_outline_item_id);

    let outline_tree = collect_outline(&doc, first_outline_item_id)?;

    if args.json {
        serde_json::to_writer_pretty(std::io::stdout(), &outline_tree).map_err(Error::Json)?;
    } else {
        print_outline(&outline_tree, 0);
    }

    Ok(())
}

fn collect_outline(doc: &Document, obj_id: ObjectId) -> Result<Vec<OutlineItem>, Error> {
    let mut items = Vec::new();
    let mut current_id: Option<lopdf::ObjectId> = Some(obj_id);

    while let Some(obj_id) = current_id {
        let outline: &lopdf::Dictionary = doc
            .get_object(obj_id)
            .map_err(Error::InvalidOutlines)?
            .as_dict()
            .map_err(Error::InvalidOutlines)?;

        // Extract title
        let title: Option<String> = if let Some(title_object) = outline.get(b"Title").ok() {
            if let Object::String(title_bytes, _string_format) = title_object {
                if title_bytes.starts_with(&[0xFE, 0xFF]) {
                    let utf16_chars = title_bytes[2..]
                        .chunks(2)
                        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
                        .collect::<Vec<u16>>();

                    Some(String::from_utf16(&utf16_chars).map_err(Error::DecodeUtf16)?)
                } else {
                    Some(String::from_utf8_lossy(title_bytes).to_string())
                }
            } else {
                None
            }
        } else {
            None
        };

        #[cfg(debug_assertions)]
        dbg!(&title);

        // Extract children from child nodes reccursively
        let children = if let Some(first_child_id) =
            outline.get(b"First").and_then(|o| o.as_reference()).ok()
        {
            #[cfg(debug_assertions)]
            dbg!(&first_child_id);
            collect_outline(doc, first_child_id)?
        } else {
            Vec::new()
        };

        items.push(OutlineItem { title, children });

        // Next sibling node
        current_id = outline.get(b"Next").and_then(|o| o.as_reference()).ok();
    }

    Ok(items)
}

fn print_outline(items: &[OutlineItem], indent: usize) {
    for item in items {
        println!(
            "{}- {}",
            "  ".repeat(indent * 2),
            item.title.clone().unwrap_or("<Untitled>".to_string())
        );
        print_outline(&item.children, indent + 1);
    }
}
