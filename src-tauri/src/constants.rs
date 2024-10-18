pub const PROMPT_CONTEXT: &str = r#"
Take the USER_PROMPT and expand it by providing additional context and details. Some of the additional context you should inject into the USER_PROMPT include, but are not limited to, the following attributes:
- Lighting and Atmosphere: Attributes related to how light interacts with the scene.
- Color and Tone: The color and tonal range within the image.
- Subject Details and Modifiers: Additional attributes that modify the appearance or characteristics of the subject.
- Photographic Techniques: Techniques or photographic effects used in the image.
- Aspect Ratio and Dimensions: Specific attributes for the format and dimensions of the image.
- Realism Enhancements: Techniques to enhance realism in the image.
- Post-Processing and Effects: Image editing and post-processing effects applied to the image.


USER_PROMPT: {{}}

The final exanded prompt must human readable, precise, succinct, and in paragraph-form. It should be less than 100 words in length. DO NOT prefix or suffix any additional commentary or information in your response, only the finalized enhanced prompt.

ENHANCED_USER_PROMPT:
"#;

pub fn render_prompt(prompt: &str) -> String {
    PROMPT_CONTEXT.replace("{{}}", prompt)
}