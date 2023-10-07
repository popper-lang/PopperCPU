use crate::cpu::Cpu;

#[cfg(feature = "plugin_io")]
pub mod io;


#[derive(Debug)]
pub struct PluginInfo<'a> {
    pub name: &'a str,
    pub author: &'a str,
    pub version: &'a str
}


#[derive(Debug)]
pub enum PluginError<'a> {
    Error(&'a str),
    Warn(Vec<&'a str>),
    Info(Vec<&'a str>),
}



pub trait Plugin<'a> {
    fn plugin_info() -> PluginInfo<'a>;

    fn handle(cpu: &Cpu) -> Result<&'a str, PluginError<'a>>;


}