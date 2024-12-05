use crate::chapter::Page;

use epub_builder::EpubBuilder;
use epub_builder::Result;
use epub_builder::ZipLibrary;
use epub_builder::EpubContent;
use epub_builder::ReferenceType;
use epub_builder::TocElement;

#[cfg(not(target_arch = "wasm32"))]

pub fn gen_epub(pages: Vec<Page>) -> Result<Vec<u8>> {
    let dummy_content = "Dummy content. This should be valid XHTML if you want a valid EPUB!";
    let dummy_image = "Not really a PNG image";
    let dummy_css = include_str!("main.css");

    let mut output = Vec::<u8>::new();

    // Create a new EpubBuilder using the zip library
    let mut binding = EpubBuilder::new(ZipLibrary::new()?)?;
    let builder = binding
    // Set some metadata
        .metadata("author", "Ollie Lynas")?
        .metadata("title", "Activity Book For Nerds")?
    // Set the stylesheet (create a "stylesheet.css" file in EPUB that is used by some generated files)
        .stylesheet(dummy_css.as_bytes())?
    // Add a image cover file
        .add_cover_image("cover.png", dummy_image.as_bytes(), "image/png")?
    // Add a resource that is not part of the linear document structure
        .add_resource("some_image.png", dummy_image.as_bytes(), "image/png")?
        ;
        for page in pages {
            builder.add_content(
                EpubContent::new(format!("{}.xml", page.title.clone().unwrap_or(fastrand::i64(0..i64::MAX).to_string())), 
                format!("<code>{}</code>",page.to_string().replace("\n", "<br>").replace(" ", "&nbsp;")).as_bytes())
            )?;
        }
        builder.generate(&mut output)?;
    Ok(output)
}