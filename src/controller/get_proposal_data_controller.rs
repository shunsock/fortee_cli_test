use crate::presentation::pretty_json_presenter::print_pretty_json;
use crate::presentation::send_message::send_message;
use crate::presentation::send_message::RunningStatus;
use crate::use_case::build_structured_information::builder::build_structured_proposal_information;
use crate::use_case::download_html_page::downloader::download_html_page;
use crate::use_case::download_og_image::downloader::download_og_image;

pub fn get_proposal_data_controller(url: &str) {
    /*
     * Download HTML page from the given URL
     */
    download_html_page(url);
    send_message(
        RunningStatus::Success,
        "HTML page has been successfully downloaded.",
    );

    /*
     * Extract structured information from the downloaded HTML page
     */
    let proposal = build_structured_proposal_information();
    send_message(
        RunningStatus::Success,
        "Structured proposal information has been successfully created.",
    );

    /*
     * Print the structured information
     */
    print_pretty_json(&proposal);

    /*
     * Download OG image from the structured information
     */
    let image_path = download_og_image(&proposal.og_image_url, "og_image");
    let image_path = match image_path {
        Ok(image_path) => image_path,
        Err(console_messenger) => {
            console_messenger.show_message();
            return;
        }
    };
    send_message(
        RunningStatus::Success,
        "OG Image has been successfully downloaded.",
    );

    /*
     * Show how to get the downloaded image for users
     */
    send_message(
        RunningStatus::Notice,
        format!(
            "you can get data by running: cp `{}` path/your/directory",
            image_path.to_string_lossy()
        )
        .as_str(),
    );
}
