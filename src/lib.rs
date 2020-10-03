use std::rc::Rc;

use thiserror::Error;

use xplm::{
    menu::{ActionItem, Item, Menu},
    plugin::{Plugin, PluginInfo},
    xplane_plugin,
};
use xplm_sys;

struct FlightSimCompanionPlugin {
    plugin_menu: Menu,
}

impl FlightSimCompanionPlugin {
    fn new() -> Result<Self, FlightSimCompanionPluginError>
    {
        let menu = Menu::new("Companion")?;
        let reload_item = ActionItem::new(
            "Reload",
            |item: &ActionItem| unsafe {
                xplm_sys::XPLMReloadPlugins();
            },
        )?;
        menu.add_child(reload_item);

        let settings_item = ActionItem::new(
            "Settings",
            |_item: &ActionItem| unsafe {},
        )?;
        menu.add_child(settings_item);

        Ok(Self { plugin_menu: menu })
    }
}

impl Plugin for FlightSimCompanionPlugin {
    type Error = FlightSimCompanionPluginError;

    fn start() -> Result<Self, Self::Error> {
        xplm::debug("Flightsimcompanionplugin start");
        let plugin = FlightSimCompanionPlugin::new()?;
        Ok(plugin)
    }

    fn info(&self) -> PluginInfo {
        PluginInfo {
            name: "Flight Sim Companion Plugin".into(),
            signature: "io.iotpi.sim.flightsimcompanion"
                .into(),
            description: "Plugin for Flight Sim Companion"
                .into(),
        }
    }

    fn enable(&mut self) -> Result<(), Self::Error> {
        xplm::debug("Flightsimcompanionplugin enable");
        self.plugin_menu.add_to_plugins_menu();
        Ok(())
    }

    fn disable(&mut self) {
        self.plugin_menu.remove_from_plugins_menu();
    }
}

#[derive(Error, Debug)]
enum FlightSimCompanionPluginError {
    #[error("Menu error")]
    Menu {
        #[from]
        source: std::ffi::NulError,
    },
}

xplane_plugin!(FlightSimCompanionPlugin);
