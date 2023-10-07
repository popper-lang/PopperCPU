use super::{
    PluginError,
    PluginInfo,
    Plugin
};

use crate::cpu::Cpu;


pub struct IoPlugin;

impl<'a> Plugin<'a> for IoPlugin {

    fn plugin_info() -> PluginInfo<'a> {
        PluginInfo {
            author: "NightProg",
            name: "Input Output Plugin",
            version: "0.0.1"
        }
    }

    fn handle(cpu: &Cpu) -> Result<&'a str, PluginError<'a>> {
        let mut output_buffer = [0; 4];
        cpu.ram.read_memory(500, &mut output_buffer);
        println!(
            "{}", u32::from_le_bytes(output_buffer)
        );

        Ok("")
    }
}