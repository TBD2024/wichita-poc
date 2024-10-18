pub const PROMPT_CONTEXT: &str = r#"
Take the USER_PROMPT and expand it by providing additional context and details. Reference the "PROMPT_GUIDELINE" included below for assistance.

PROMPT_GUIDELINE:
---
Lighting and Atmosphere:
  Details: Attributes related to how light interacts with the scene.
  Lighting Type:
    Description: The primary source or style of lighting used in the scene.
    Enum:
    - Natural light
    - Artificial light
    - Soft light
    - Harsh light
    - Backlighting
    - Spotlight
    - Ambient lighting
    - Other
  Time of Day:
    Description: The time of day as it influences light and shadows.
    Enum:
    - Morning
    - Noon
    - Afternoon
    - Evening
    - Dusk
    - Night
    - Twilight
    - Other
  Shadows:
    Description: The presence and style of shadows in the image.
    Enum:
    - Soft shadows
    - Harsh shadows
    - Long shadows
    - No shadows
    - Diffused shadows
    - Other
Color and Tone:
  Details: The color and tonal range within the image.
  Color Palette:
    Description: The overall color scheme or range of colors in the image.
    Enum:
    - Vibrant
    - Pastel
    - Muted
    - Monochrome
    - Natural
    - Warm tones
    - Cool tones
    - Other
  Tonal Range:
    Description: The contrast between light and dark areas in the image.
    Enum:
    - High contrast
    - Low contrast
    - Balanced contrast
    - Dark tones
    - Bright tones
    - Other
  Saturation:
    Description: The intensity of the colors in the image.
    Enum:
    - Highly saturated
    - Desaturated
    - Neutral saturation
    - Over-saturated
    - Muted
    - Other
Subject Details and Modifiers:
  Details: Additional attributes that modify the appearance or characteristics of
    the subject.
  Clothing and Fashion:
    Description: The style or type of clothing the person is wearing.
    Enum:
    - Casual
    - Formal
    - Traditional
    - Business
    - Athletic
    - Winterwear
    - Summerwear
    - Other
  Skin and Hair Details:
    Description: Specific details about the person's skin and hair.
    Enum:
    - Freckled
    - Wrinkled
    - Tanned
    - Pale
    - Smooth
    - Curly hair
    - Straight hair
    - Short hair
    - Long hair
    - Bald
    - Other
  Expression:
    Description: The emotion displayed through facial expression.
    Enum:
    - Happy
    - Serious
    - Pensive
    - Neutral
    - Smiling
    - Frowning
    - Angry
    - Other
  Activity and Action:
    Description: The action or activity the subject is engaged in.
    Enum:
    - Walking
    - Running
    - Sitting
    - Working
    - Talking
    - Reading
    - Exercising
    - Other
Colors and Tones:
  Details: Attributes of the color palette and tonal values in the image.
  Color Palette:
    Description: The overall color scheme in the image.
    Enum:
    - Warm tones
    - Cool tones
    - Neutral tones
    - Muted tones
    - Bright tones
    - Monochrome
    - Other
  Tonal Range:
    Description: The contrast between light and dark areas in the image.
    Enum:
    - High contrast
    - Low contrast
    - Balanced contrast
    - Dark tones
    - Light tones
    - Other
  Saturation:
    Description: The intensity of the colors.
    Enum:
    - Highly saturated
    - Desaturated
    - Natural saturation
    - Over-saturated
    - Muted
    - Other
Material and Texture:
  Details: Description of the texture or material characteristics.
  Surfaces and Textures:
    Description: The texture or finish of surfaces in the image.
    Enum:
    - Smooth
    - Rough
    - Glossy
    - Matte
    - Textured
    - Grainy
    - Polished
    - Other
  Fabrics:
    Enum:
    - Wool
    - Cotton
    - Leather
    - Denim
    - Silk
    - Velvet
    - Other
Photographic Techniques:
  Details: Techniques or photographic effects used in the image.
  Exposure:
    Description: How light or dark the image is.
    Enum:
    - Well-exposed
    - Overexposed
    - Underexposed
    - High exposure
    - Low exposure
    - Other
  Motion Blur:
    Description: The effect of motion blur in the image.
    Enum:
    - Motion blur
    - No motion blur
    - Slight blur
    - Heavy blur
    - Other
  Grain/Noise:
    Description: The presence of grain or noise in the image.
    Enum:
    - Clean
    - Slight grain
    - Heavy grain
    - High ISO noise
    - Other
Aspect Ratio and Dimensions:
  Details: Specific attributes for the format and dimensions of the image.
  Aspect Ratio:
    Description: The width-to-height ratio of the image.
    Enum:
    - '4:3'
    - '16:9'
    - '1:1'
    - Panoramic
    - Portrait
    - Landscape
    - Other
  Resolution:
    Description: The pixel dimensions of the image.
    Enum:
    - High-resolution
    - Low-resolution
    - Medium-resolution
    - Other
Realism Enhancements:
  Details: Techniques to enhance realism in the image.
  Photorealism:
    Description: The degree of realism in the image, emphasizing lifelike details.
    Enum:
    - Hyper-realistic
    - Natural realism
    - Stylized realism
    - Other
  Lens Effects:
    Description: The effects produced by camera lenses, such as lens flare or bokeh.
    Enum:
    - Bokeh
    - Lens flare
    - Vignette
    - Depth of field blur
    - Other
Post-Processing and Effects:
  Details: Image editing and post-processing effects applied to the image.
  Filters and Editing Styles:
    Description: The overall editing style or filter applied to the image.
    Enum:
    - HDR
    - Vintage
    - Sepia
    - Black-and-white
    - High contrast
    - Soft focus
    - Other
  Sharpness:
    Description: The sharpness or clarity of the image.
    Enum:
    - High sharpness
    - Soft focus
    - Medium sharpness
    - Other


USER_PROMPT: {{}}

Enhance the USER_PROMPT based on PROMPT_GUIDELINE to add more specificity. The final exanded prompt must be succinct. DO NOT include additional commentary or information in the response, only the enhanced prompt.

ENHANCED_USER_PROMPT:
"#;

pub fn render_prompt(prompt: &str) -> String {
    PROMPT_CONTEXT.replace("{{}}", prompt)
}