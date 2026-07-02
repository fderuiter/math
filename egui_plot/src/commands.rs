use egui::{Key, Modifiers, Response, Ui, Context, Id};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandTrigger {
    Key(Key),
    Shortcut(Modifiers, Key),
    AltClick,
}

#[derive(Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub trigger: CommandTrigger,
    pub desktop_only: bool,
    pub context: String,
}

#[derive(Default, Clone)]
pub struct CommandRegistryData {
    pub commands: Vec<CommandMetadata>,
}

impl CommandRegistryData {
    pub fn register_and_check(
        ctx: &Context,
        name: &str,
        description: &str,
        trigger: CommandTrigger,
        desktop_only: bool,
        context_name: &str,
        ui: Option<&Ui>,
        response: Option<&Response>,
    ) -> bool {
        let mut data = ctx.data_mut(|d| d.get_temp::<Self>(Id::new("CMD_REGISTRY")).unwrap_or_default());
        
        // Avoid duplicates in the same frame
        if !data.commands.iter().any(|c| c.name == name && c.context == context_name) {
            data.commands.push(CommandMetadata {
                name: name.to_owned(),
                description: description.to_owned(),
                trigger: trigger.clone(),
                desktop_only,
                context: context_name.to_owned(),
            });
            ctx.data_mut(|d| d.insert_temp(Id::new("CMD_REGISTRY"), data));
        }

        let input_mode = ctx.data(|d| d.get_temp::<bool>(Id::new("INPUT_MODE_TOUCH")).unwrap_or(false));
        if desktop_only && input_mode {
            return false;
        }

        let mut triggered = false;
        
        match trigger {
            CommandTrigger::Key(k) => {
                if let Some(ui) = ui {
                    if ui.input(|i| i.key_pressed(k)) {
                        triggered = true;
                    }
                } else if ctx.input(|i| i.key_pressed(k)) {
                    triggered = true;
                }
            }
            CommandTrigger::Shortcut(mods, k) => {
                if let Some(ui) = ui {
                    if ui.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(mods, k))) {
                        triggered = true;
                    }
                } else if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(mods, k))) {
                    triggered = true;
                }
            }
            CommandTrigger::AltClick => {
                if let (Some(ui), Some(resp)) = (ui, response)
                    && resp.clicked() && ui.input(|i| i.modifiers.alt) {
                        triggered = true;
                    }
            }
        }

        if triggered {
            let msg = format!("Action executed: {name}");
            ctx.data_mut(|d| d.insert_temp(Id::new("aria_live_message"), msg));
        }

        triggered
    }
}
