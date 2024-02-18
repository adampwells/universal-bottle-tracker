use std::io::Read;

use docx_rs::*;
use nanoid::nanoid;
use qrcode_generator::QrCodeEcc;
use tokio::fs;
use tracing::log::debug;

pub async fn get_label_docx() -> Result<Vec<u8>, DocxError> {
    let temp_file_name = format!("./{}-table.docx", nanoid!());
    let path = std::path::Path::new(&temp_file_name);
    let file = std::fs::File::create(path).unwrap();

    let table = Table::new(
        (1..=7).map(|_| {
            TableRow::new(
                (1..=3).map(|_| {
                    generate_cell()
                }).collect()
            ).row_height(2216.0)
        }).collect()
    )
        .align(TableAlignmentType::Center)
        .clear_all_border();

    Docx::new()
        .page_size(11906, 16838)
        .page_margin(
            PageMargin {
                top: 680,
                left: 567,
                bottom: 454,
                right: 567,
                header: 0,
                footer: 0,
                gutter: 0,
            })
        .add_table(table)
        .build()
        .pack(&file)?;

    let mut file = std::fs::File::open(path).unwrap();
    let mut doc_bytes: Vec<u8> = Vec::new();
    let result = file.read_to_end(&mut doc_bytes);
    if let Err(e) = result {
        debug!("Error reading file: {:?}", e);
    }

    let result = fs::remove_file(path).await;
    if let Err(e) = result {
        debug!("Error deleting file: {:?}", e);
    }

    Ok(doc_bytes)
}

fn generate_cell() -> TableCell {
    let qr_code: Vec<u8> = qrcode_generator::to_png_to_vec(format!("https://app.wht1.au/?b={}", nanoid!()), QrCodeEcc::Low, 100).unwrap();
    let qr_code = Pic::new(qr_code).size(100, 100);
    TableCell::new()
        .width(3686, WidthType::DXA)
        .vertical_align(VAlignType::Center)
        .add_paragraph(
            Paragraph::new()
                .align(AlignmentType::Center)
                .add_run(
                    Run::new()
                        .add_image(qr_code)
                )
        )
}

#[cfg(test)]
mod tests {
    use crate::labels::get_label_docx;

    #[tokio::test]
    async fn exploration() {
        let result = get_label_docx().await;
        assert!(result.is_ok());
    }

}