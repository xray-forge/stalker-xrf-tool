use crate::data::file_description::FileDescription;
use crate::data::sprite_description::SpriteDescription;
use crate::description::pack_description_options::PackDescriptionOptions;
use crate::TextureResult;
use roxmltree::{Document, Node, ParsingOptions};
use std::collections::HashMap;
use std::fs;
use std::fs::{File, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};

pub struct XmlDescriptionCollection {
  pub files: HashMap<String, FileDescription>,
}

impl XmlDescriptionCollection {
  /// Get descriptions from provided options.
  /// Handle both directory and single file as inputs.
  pub fn get_descriptions(options: &PackDescriptionOptions) -> TextureResult<Self> {
    if options.description.is_dir() {
      println!(
        "Check texture descriptions from dir: {:?}",
        options.description
      );

      let mut files: HashMap<String, FileDescription> = HashMap::new();
      let entries: ReadDir = fs::read_dir(&options.description)?;

      for entry in entries.flatten() {
        let path: PathBuf = entry.path();

        if let Some(extension) = path.extension() {
          if extension == "xml" {
            let descriptions: HashMap<String, FileDescription> =
              Self::get_description(options, &path)?;

            descriptions
              .into_iter()
              .for_each(|(name, description)| match files.get_mut(&name) {
                None => {
                  files.insert(name, description);
                }
                Some(existing) => {
                  if options.is_verbose {
                    println!("Merging textures for {name}");
                  }

                  existing.sprites.extend(description.sprites);
                }
              })
          }
        }
      }

      Ok(Self { files })
    } else {
      Ok(Self {
        files: Self::get_description(options, &options.description)?,
      })
    }
  }

  /// Get descriptions from provided file path.
  pub fn get_description(
    options: &PackDescriptionOptions,
    path: &Path,
  ) -> TextureResult<HashMap<String, FileDescription>> {
    if options.is_verbose {
      println!("Found texture description: {:?}", path);
    }

    let mut descriptions: HashMap<String, FileDescription> = HashMap::new();

    let mut file: File = File::open(path)?;
    let mut text: String = String::new();

    file.read_to_string(&mut text)?;

    let document: Document = match Document::parse_with_options(
      &text,
      ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
      },
    ) {
      Ok(doc) => doc,
      Err(error) => {
        if options.is_strict {
          panic!("Failed to parse xml: {:?} - {:?}", path, error)
        }

        println!("Error parsing XML file: {:?} - {:?}", path, error);
        return Ok(HashMap::new());
      }
    };

    let window: Option<Node> = document
      .root()
      .children()
      .find(|it| it.is_element() && it.tag_name().name().eq("w"));

    if let Some(window) = window {
      for file in window
        .children()
        .filter(|it| it.is_element() && it.tag_name().name().eq("file"))
      {
        let file_name: Option<&str> = file.attribute("name");

        if let Some(file_name) = file_name {
          if options.is_verbose {
            println!("Parsing file: {file_name}");
          }

          let mut file_description: FileDescription = FileDescription::new(file_name);

          for node in file
            .descendants()
            .filter(|it| it.is_element() && it.tag_name().name().eq("texture"))
          {
            if let Some(sprite) = SpriteDescription::new_optional_from_node(node) {
              file_description.add_sprite(sprite);
            } else {
              println!(
                "Skip texture node: {:?} ({:?})",
                node.attribute("id").unwrap_or("unknown"),
                node
                  .attributes()
                  .map(|it| format!("{}={}", it.name(), it.value()))
                  .collect::<Vec<String>>(),
              );
            }
          }

          if file_description.sprites.is_empty() {
            println!(
              "Skip definitions node \"{file_name}\" without textures (in {:?})",
              path
            );
          } else {
            match descriptions.get_mut(&file_description.name) {
              None => {
                descriptions.insert(file_description.name.clone(), file_description);
              }
              Some(existing) => {
                if options.is_verbose {
                  println!("Merging textures for {file_name}");
                }

                file_description
                  .sprites
                  .into_iter()
                  .for_each(|it| existing.sprites.push(it));
              }
            }
          }
        } else {
          println!("Invalid file node supplied without name attribute");
        }
      }
    } else {
      println!("Got no 'w' tag for file '{:?}'", options.description);
    }

    Ok(descriptions)
  }
}
