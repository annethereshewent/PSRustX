#[derive(Clone, Copy)]
pub enum TextureColors {
  FourBit = 0,
  EightBit = 1,
  FifteenBit = 2
}

#[derive(Clone, Copy)]
pub enum Field {
  Bottom = 0,
  Top = 1
}

#[derive(Clone, Copy)]
pub enum VideoMode {
  Ntsc = 0,
  Pal = 1
}

#[derive(Clone, Copy)]
pub enum ColorDepth {
  FifteenBit = 0,
  TwentyFourBit = 1
}

#[derive(Clone, Copy)]
pub enum  DmaDirection {
  Off = 0,
  CputoGP0 = 2,
  GpuReadToCpu = 3
}

pub struct GpuStatRegister {
  pub texture_x_base: u8,
  pub texture_y_base1: u8,
  pub texture_y_base2: u8,

  pub semi_transparency: u8,
  pub texture_colors: TextureColors,
  pub dither_enabled: bool,
  pub draw_to_display: bool,
  pub force_mask_bit: bool,
  pub preserved_masked_pixels: bool,
  pub interlace_field: Field,
  pub reverse_flag: bool,
  pub hres1: u8,
  pub hres2: u8,
  pub vres: u8,
  pub video_mode: VideoMode,
  pub display_color_depth: ColorDepth,
  pub vertical_interlace: bool,
  pub display_enable: bool,
  pub irq_enabled: bool,
  pub dma_dir: DmaDirection,
  pub ready_for_command: bool,
  pub ready_vram_to_cpu: bool,
  pub ready_rcv_dma_block: bool,
  pub even_odd: bool
}

impl GpuStatRegister {
  pub fn new() -> Self {
    Self {
      texture_x_base: 0,
      texture_y_base1: 0,
      texture_y_base2: 0,
      semi_transparency: 0,
      texture_colors: TextureColors::FourBit,
      dither_enabled: false,
      draw_to_display: false,
      force_mask_bit: false,
      preserved_masked_pixels: false,
      interlace_field: Field::Bottom,
      reverse_flag: false,
      hres1: 0,
      hres2: 0,
      vres: 0,
      video_mode: VideoMode::Ntsc,
      display_color_depth: ColorDepth::FifteenBit,
      vertical_interlace: false,
      display_enable: false,
      irq_enabled: false,
      dma_dir: DmaDirection::Off,
      ready_for_command: true,
      ready_rcv_dma_block: true,
      ready_vram_to_cpu: true,
      even_odd: false
    }
  }

  pub fn update_draw_mode(&mut self, val: u32) {
    self.texture_x_base = (val & 0xf) as u8;
    self.texture_y_base1 = ((val >> 4) & 0b1) as u8;
    self.semi_transparency = ((val >> 5) & 0b11) as u8;

    self.texture_colors = match (val >> 7) & 0b11 {
      0 => TextureColors::FourBit,
      1 => TextureColors::EightBit,
      2 => TextureColors::FifteenBit,
      n => panic!("unhandled texture depth received: {n}")
    };

    self.dither_enabled = ((val >> 9) & 0b1) == 1;
    self.draw_to_display = ((val >> 10) & 0b1) == 1;
    self.texture_y_base2 = ((val >> 11) & 0b1) as u8;
  }

  pub fn value(&self) -> u32 {
    let mut result = 0u32;

    result |= self.texture_x_base as u32;
    result |= (self.texture_y_base1 as u32) << 4;
    result |= (self.semi_transparency as u32) << 5;
    result |= (self.texture_colors as u32) << 7;
    result |= (self.dither_enabled as u32) << 9;
    result |= (self.draw_to_display as u32 ) << 10;
    result |= (self.force_mask_bit as u32) << 11;
    result |= (self.preserved_masked_pixels as u32) << 12;
    result |= (self.interlace_field as u32) << 13;
    result |= (self.texture_y_base2 as u32) << 15;
    result |= (self.hres2 as u32) << 16;
    result |= (self.hres1 as u32) << 17;
    result |= (self.vres as u32) << 19;
    result |= (self.video_mode as u32) << 20;
    result |= (self.display_color_depth as u32) << 21;
    result |= (self.vertical_interlace as u32) << 22;
    result |= (self.display_enable as u32) << 23;
    result |= (self.irq_enabled as u32) << 24;
    result |= (0b111) << 26;
    result |= (self.dma_dir as u32) << 29;

    let dma_request = match self.dma_dir {
      DmaDirection::Off => 0,
      DmaDirection::CputoGP0 => (result >> 28) & 0b1,
      DmaDirection::GpuReadToCpu => 1
    };

    result |= dma_request << 25;

    result
  }
}