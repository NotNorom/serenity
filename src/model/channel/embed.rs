#[cfg(feature="utils")]
use ::utils::Colour;
#[cfg(feature="model")]
use ::internal::prelude::*;
#[cfg(feature="model")]
use ::builder::CreateEmbed;

/// Represents a rich embed which allows using richer markdown, multiple fields
/// and more. This was heavily inspired by [slack's attachments].
///
/// You can include an attachment in your own message by a user or a bot, or in
/// a webhook.
///
/// **Note**: Maximum amount of characters you can put is 256 in a field name,
/// 1024 in a field value, and 2048 in a description.
///
/// [slack's attachments]: https://api.slack.com/docs/message-attachments
#[derive(Clone, Debug, Deserialize)]
pub struct Embed {
    /// Information about the author of the embed.
    pub author: Option<EmbedAuthor>,
    /// The colour code of the embed.
    #[cfg(feature="utils")]
    #[serde(default, rename="color")]
    pub colour: Colour,
    /// The colour code of the embed.
    #[cfg(not(feature="utils"))]
    #[serde(default, rename="color")]
    pub colour: u32,
    /// The description of the embed.
    ///
    /// The maximum value for this field is 2048 unicode codepoints.
    pub description: Option<String>,
    /// The array of fields.
    ///
    /// The maximum number of fields is 25.
    #[serde(default)]
    pub fields: Vec<EmbedField>,
    /// Image information of the embed.
    pub image: Option<EmbedImage>,
    /// The type of the embed. For embeds not generated by Discord's backend,
    /// this will always be "rich".
    #[serde(rename="type")]
    pub kind: String,
    /// Provider information for the embed.
    ///
    /// For example, if the embed [`kind`] is `"video"`, the provider might
    /// contain information about YouTube.
    pub provider: Option<EmbedProvider>,
    /// Thumbnail information of the embed.
    pub thumbnail: Option<EmbedThumbnail>,
    /// Timestamp information.
    pub timestamp: Option<String>,
    /// The title of the embed.
    pub title: Option<String>,
    /// The URL of the embed.
    pub url: Option<String>,
    /// The embed's video information.
    ///
    /// This is present if the [`kind`] is `"video"`.
    pub video: Option<EmbedVideo>,
}

#[cfg(feature="model")]
impl Embed {
    /// Creates a fake Embed, giving back a `serde_json` map.
    ///
    /// This should only be useful in conjunction with [`Webhook::execute`].
    ///
    /// [`Webhook::execute`]: struct.Webhook.html
    ///
    /// # Examples
    ///
    /// Create an embed:
    ///
    /// ```rust,no_run
    /// use serenity::model::Embed;
    ///
    /// let embed = Embed::fake(|e| e
    ///     .title("Embed title")
    ///     .description("Making a basic embed")
    ///     .field(|f| f
    ///         .name("A field")
    ///         .value("Has some content.")
    ///         .inline(false)));
    /// ```
    #[inline]
    pub fn fake<F>(f: F) -> Value where F: FnOnce(CreateEmbed) -> CreateEmbed {
        Value::Object(f(CreateEmbed::default()).0)
    }
}

/// An author object in an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedAuthor {
    /// The URL of the author icon.
    ///
    /// This only supports HTTP(S).
    pub icon_url: Option<String>,
    /// The name of the author.
    pub name: String,
    /// A proxied URL of the author icon.
    pub proxy_icon_url: Option<String>,
    /// The URL of the author.
    pub url: Option<String>,
}

/// A field object in an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedField {
    /// Indicator of whether the field should display as inline.
    pub inline: bool,
    /// The name of the field.
    ///
    /// The maximum length of this field is 512 unicode codepoints.
    pub name: String,
    /// The value of the field.
    ///
    /// The maxiumum length of this field is 1024 unicode codepoints.
    pub value: String,
}

/// Footer information for an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedFooter {
    /// The URL of the footer icon.
    ///
    /// This only supports HTTP(S).
    pub icon_url: String,
    /// A proxied URL of the footer icon.
    pub proxy_icon_url: String,
    /// The associated text with the footer.
    pub text: String,
}

/// An image object in an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedImage {
    /// The height of the image.
    pub height: u64,
    /// A proxied URL of the image.
    pub proxy_url: String,
    /// Source URL of the image.
    ///
    /// This only supports HTTP(S).
    pub url: String,
    /// The width of the image.
    pub width: u64,
}

/// The provider of an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedProvider {
    /// The name of the provider.
    pub name: String,
    /// The URL of the provider.
    pub url: Option<String>,
}

/// The dimensions and URL of an embed thumbnail.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedThumbnail {
    /// The height of the thumbnail in pixels.
    pub height: u64,
    /// A proxied URL of the thumbnail.
    pub proxy_url: String,
    /// The source URL of the thumbnail.
    ///
    /// This only supports HTTP(S).
    pub url: String,
    /// The width of the thumbnail in pixels.
    pub width: u64,
}

/// Video information for an embed.
#[derive(Clone, Debug, Deserialize)]
pub struct EmbedVideo {
    /// The height of the video in pixels.
    pub height: u64,
    /// The source URL of the video.
    pub url: String,
    /// The width of the video in pixels.
    pub width: u64,
}
