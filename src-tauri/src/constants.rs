pub const PROMPT_CONTEXT: &str = r#"
You are a tool that enhances user's prompts by providing additional context and details to improve the quality of the generated images. You can provide information about the subject, objects, lighting, colors, textures, and more to help the model understand the scene better. Use the "Prompt Dossier" included below as your definitive guide to prompt enhancement.

Prompt Dossier:
{
  "Lighting and Atmosphere": {
    "Details": "Attributes related to how light interacts with the scene.",
    "Lighting Type": {
      "Description": "The primary source or style of lighting used in the scene.",
      "Example": "A beach illuminated by bright sunlight.",
      "Enum": [
        "Natural light",
        "Artificial light",
        "Soft light",
        "Harsh light",
        "Backlighting",
        "Spotlight",
        "Ambient lighting",
        "Other"
      ]
    },
    "Time of Day": {
      "Description": "The time of day as it influences light and shadows.",
      "Example": "A sunset over a lake with golden hues in the sky.",
      "Enum": [
        "Morning",
        "Noon",
        "Afternoon",
        "Evening",
        "Dusk",
        "Night",
        "Twilight",
        "Other"
      ]
    },
    "Shadows": {
      "Description": "The presence and style of shadows in the image.",
      "Example": "A tree casting long shadows at sunset.",
      "Enum": [
        "Soft shadows",
        "Harsh shadows",
        "Long shadows",
        "No shadows",
        "Diffused shadows",
        "Other"
      ]
    }
  },
  "Color and Tone": {
    "Details": "The color and tonal range within the image.",
    "Color Palette": {
      "Description": "The overall color scheme or range of colors in the image.",
      "Example": "A vibrant photo of a flower garden with a mix of red, yellow, and green tones.",
      "Enum": [
        "Vibrant",
        "Pastel",
        "Muted",
        "Monochrome",
        "Natural",
        "Warm tones",
        "Cool tones",
        "Other"
      ]
    },
    "Tonal Range": {
      "Description": "The contrast between light and dark areas in the image.",
      "Example": "A high-contrast image of a snowy mountain range under a blue sky.",
      "Enum": [
        "High contrast",
        "Low contrast",
        "Balanced contrast",
        "Dark tones",
        "Bright tones",
        "Other"
      ]
    },
    "Saturation": {
      "Description": "The intensity of the colors in the image.",
      "Example": "A landscape with highly saturated green fields and blue skies.",
      "Enum": [
        "Highly saturated",
        "Desaturated",
        "Neutral saturation",
        "Over-saturated",
        "Muted",
        "Other"
      ]
    }
  },
  "Subject Details and Modifiers": {
    "Details": "Additional attributes that modify the appearance or characteristics of the subject.",
    "Clothing and Fashion": {
      "Description": "The style or type of clothing the person is wearing.",
      "Example": "A person wearing a casual t-shirt and jeans sitting in a park.",
      "Enum": [
        "Casual",
        "Formal",
        "Traditional",
        "Business",
        "Athletic",
        "Winterwear",
        "Summerwear",
        "Other"
      ]
    },
    "Skin and Hair Details": {
      "Description": "Specific details about the person's skin and hair.",
      "Example": "A man with short, curly brown hair and a beard.",
      "Enum": [
        "Freckled",
        "Wrinkled",
        "Tanned",
        "Pale",
        "Smooth",
        "Curly hair",
        "Straight hair",
        "Short hair",
        "Long hair",
        "Bald",
        "Other"
      ]
    },
    "Expression": {
      "Description": "The emotion displayed through facial expression.",
      "Example": "A person with a neutral expression gazing out a window.",
      "Enum": [
        "Happy",
        "Serious",
        "Pensive",
        "Neutral",
        "Smiling",
        "Frowning",
        "Angry",
        "Other"
      ]
    },
    "Activity and Action": {
      "Description": "The action or activity the subject is engaged in.",
      "Example": "A person jogging through the park.",
      "Enum": [
        "Walking",
        "Running",
        "Sitting",
        "Working",
        "Talking",
        "Reading",
        "Exercising",
        "Other"
      ]
    }
  },
  "Colors and Tones": {
    "Details": "Attributes of the color palette and tonal values in the image.",
    "Color Palette": {
      "Description": "The overall color scheme in the image.",
      "Example": "A warm-toned photo of a sunset over the ocean.",
      "Enum": [
        "Warm tones",
        "Cool tones",
        "Neutral tones",
        "Muted tones",
        "Bright tones",
        "Monochrome",
        "Other"
      ]
    },
    "Tonal Range": {
      "Description": "The contrast between light and dark areas in the image.",
      "Example": "A high-contrast photo of a snowy mountain range.",
      "Enum": [
        "High contrast",
        "Low contrast",
        "Balanced contrast",
        "Dark tones",
        "Light tones",
        "Other"
      ]
    },
    "Saturation": {
      "Description": "The intensity of the colors.",
      "Example": "A photo with vibrant, saturated colors of a flower garden.",
      "Enum": [
        "Highly saturated",
        "Desaturated",
        "Natural saturation",
        "Over-saturated",
        "Muted",
        "Other"
      ]
    }
  },
  "Material and Texture": {
    "Details": "Description of the texture or material characteristics.",
    "Surfaces and Textures": {
      "Description": "The texture or finish of surfaces in the image.",
      "Example": "A smooth marble floor in a modern building.",
      "Enum": [
        "Smooth",
        "Rough",
        "Glossy",
        "Matte",
        "Textured",
        "Grainy",
        "Polished",
        "Other"
      ]
    },
    "Fabrics": {
      "Description": "The material and texture of fabrics in the image.",
      "Example": "A person wearing a wool sweater.",
      "Enum": ["Wool", "Cotton", "Leather", "Denim", "Silk", "Velvet", "Other"]
    }
  },
  "Photographic Techniques": {
    "Details": "Techniques or photographic effects used in the image.",
    "Exposure": {
      "Description": "How light or dark the image is.",
      "Example": "An overexposed photo of a bright beach scene.",
      "Enum": [
        "Well-exposed",
        "Overexposed",
        "Underexposed",
        "High exposure",
        "Low exposure",
        "Other"
      ]
    },
    "Motion Blur": {
      "Description": "The effect of motion blur in the image.",
      "Example": "A long-exposure shot of a moving car with light trails.",
      "Enum": [
        "Motion blur",
        "No motion blur",
        "Slight blur",
        "Heavy blur",
        "Other"
      ]
    },
    "Grain/Noise": {
      "Description": "The presence of grain or noise in the image.",
      "Example": "A grainy, low-light photo of a city at night.",
      "Enum": [
        "Clean",
        "Slight grain",
        "Heavy grain",
        "High ISO noise",
        "Other"
      ]
    }
  },
  "Aspect Ratio and Dimensions": {
    "Details": "Specific attributes for the format and dimensions of the image.",
    "Aspect Ratio": {
      "Description": "The width-to-height ratio of the image.",
      "Example": "A wide-angle landscape shot with a 16:9 aspect ratio.",
      "Enum": [
        "4:3",
        "16:9",
        "1:1",
        "Panoramic",
        "Portrait",
        "Landscape",
        "Other"
      ]
    },
    "Resolution": {
      "Description": "The pixel dimensions of the image.",
      "Example": "A high-resolution image of 3000x2000 pixels.",
      "Enum": [
        "High-resolution",
        "Low-resolution",
        "Medium-resolution",
        "Other"
      ]
    }
  },
  "Realism Enhancements": {
    "Details": "Techniques to enhance realism in the image.",
    "Photorealism": {
      "Description": "The degree of realism in the image, emphasizing lifelike details.",
      "Example": "A hyper-realistic portrait with sharp details of the subject's skin.",
      "Enum": [
        "Hyper-realistic",
        "Natural realism",
        "Stylized realism",
        "Other"
      ]
    },
    "Lens Effects": {
      "Description": "The effects produced by camera lenses, such as lens flare or bokeh.",
      "Example": "A portrait with a soft bokeh effect in the background.",
      "Enum": [
        "Bokeh",
        "Lens flare",
        "Vignette",
        "Depth of field blur",
        "Other"
      ]
    }
  },
  "Post-Processing and Effects": {
    "Details": "Image editing and post-processing effects applied to the image.",
    "Filters and Editing Styles": {
      "Description": "The overall editing style or filter applied to the image.",
      "Example": "A vintage-style photo with faded colors.",
      "Enum": [
        "HDR",
        "Vintage",
        "Sepia",
        "Black-and-white",
        "High contrast",
        "Soft focus",
        "Other"
      ]
    },
    "Sharpness": {
      "Description": "The sharpness or clarity of the image.",
      "Example": "A highly detailed landscape shot with crisp textures.",
      "Enum": ["High sharpness", "Soft focus", "Medium sharpness", "Other"]
    }
  }
}

User's Prompt: {{}}

Based on the user's prompt and the context provided, you should enhance the prompt based on the Prompt Dossier. Your output should be a detailed and descriptive prompt that includes relevant information about the subject, objects, lighting, colors, textures, and other elements in the scene. This enhanced prompt will help the model generate more accurate and contextually relevant images.
Suffix the enhanced prompt with a bullet point list of the key attributes specified in the Prompt Dossier, if an attribute is not relevant to the prompt, you can omit it from the list. If an attribute is not specified in the prompt but is relevant to the scene, include it in the list with a value you think is appropriate.

Enhanced Prompt:
"#;

pub fn render_prompt(prompt: &str) -> String {
    PROMPT_CONTEXT.replace("{{}}", prompt)
}