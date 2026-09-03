use clap::ValueEnum;

#[derive(ValueEnum, Clone, Default, PartialEq, Debug)]
pub enum Platform {
    #[default]
    ArmMacos,
    X86Macos,
    X86Linux,
    X86Windows,
}
